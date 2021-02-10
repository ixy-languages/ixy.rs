use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::num::Wrapping;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::os::unix::io::RawFd;
use std::rc::Rc;
use std::sync::atomic::{self, Ordering};
use std::time::Duration;
use std::{io, mem, slice, thread};

use crate::memory;
use crate::memory::{Dma, Packet, PACKET_HEADROOM};
use crate::pci::{self, read_io16, read_io32, read_io8, write_io16, write_io32, write_io8};
use crate::virtio_constants::*;
use crate::{DeviceStats, IxyDevice, Mempool};

// we're currently only supporting legacy Virtio via PCI so this is fixed (4.1.5.1.3.1)
const QUEUE_ALIGNMENT: usize = 4096;

static NET_HEADER: virtio_net_hdr = virtio_net_hdr {
    flags: 0,
    gso_type: VIRTIO_NET_HDR_GSO_NONE,
    hdr_len: 14 + 20 + 8,
    // ignored fields
    csum_offset: 0,
    csum_start: 0,
    gso_size: 0,
};

// NOTE: Currently we only support the legacy interface (device id == 0x1000)
// NOTE: We currently don't keep track of a "driver ring wrap counter" following upstream ixy
pub struct VirtioDevice {
    pci_addr: String,
    bar0: File,

    rx_queue: Virtqueue,
    tx_queue: Virtqueue,
    ctrl_queue: Virtqueue,

    rx_mempool: Rc<Mempool>,
    // tx buffers are managed by user
    ctrl_mempool: Rc<Mempool>,

    tx_inflight: VecDeque<Packet>,
    rx_inflight: VecDeque<Packet>,

    // statistics
    rx_pkts: u64,
    tx_pkts: u64,
    rx_bytes: u64,
    tx_bytes: u64,
}

impl IxyDevice for VirtioDevice {
    fn get_driver_name(&self) -> &str {
        "ixy-virtio"
    }

    fn is_card_iommu_capable(&self) -> bool {
        false
    }

    fn get_vfio_container(&self) -> Option<RawFd> {
        None
    }

    fn get_pci_addr(&self) -> &str {
        &self.pci_addr
    }

    fn get_mac_addr(&self) -> [u8; 6] {
        let mut bar0 = self.bar0.try_clone().unwrap();
        let mut mac = [0; 6];
        for (i, byte) in mac.iter_mut().enumerate() {
            *byte = read_io8(&mut bar0, (20 + i) as u64).unwrap();
        }
        mac
    }

    fn set_mac_addr(&self, mac: [u8; 6]) {
        // since we're using the legacy interface we can update the MAC address without having
        // negotiated `VIRTIO_NET_F_CTRL_MAC_ADDR` during initialization
        let mut bar0 = self.bar0.try_clone().unwrap();
        for (i, byte) in mac.iter().enumerate() {
            write_io8(&mut bar0, *byte, (20 + i) as u64).unwrap();
        }
    }

    fn rx_batch(
        &mut self,
        _queue_id: u16,
        buffer: &mut VecDeque<Packet>,
        num_packets: usize,
    ) -> usize {
        // 2.6.14

        mfence();
        // remove received packets from the virtqueue and make them available to the user
        for _ in 0..num_packets {
            if self.rx_queue.last_used_idx == self.rx_queue.used.idx {
                break;
            }

            let used =
                &self.rx_queue.used[self.rx_queue.last_used_idx.0 % self.rx_queue.size].clone();
            self.rx_queue.last_used_idx += Wrapping(1);

            // mark used descriptor as unused again
            let desc = &mut self.rx_queue.descriptors_mut()[used.id as usize];
            assert_eq!(
                desc.flags, VIRTQ_DESC_F_WRITE,
                "unsupported flags on rx descriptor: {:x}",
                desc.flags
            );
            desc.addr = 0;

            let mut buf = self.rx_inflight.pop_front().unwrap();
            // adjust buffer length to actual packet size
            buf.len = used.len as usize - mem::size_of::<virtio_net_hdr>();

            self.rx_bytes += buf.len as u64;
            self.rx_pkts += 1;
            buffer.push_back(buf);
        }

        // add new descriptors to the available ring so the device can fill those up
        let mut queued = 0;
        for idx in 0..self.rx_queue.size {
            let desc = &mut self.rx_queue.descriptors_mut()[idx as usize];
            if desc.addr != 0 {
                continue;
            }

            let buf = memory::alloc_pkt(
                &self.rx_mempool,
                self.rx_mempool.entry_size() - PACKET_HEADROOM,
            )
            .expect("rx memory pool exhausted");

            *desc = VirtqDesc {
                len: buf.len as u32 + mem::size_of::<virtio_net_hdr>() as u32,
                addr: buf.get_phys_addr() - mem::size_of::<virtio_net_hdr>(),
                flags: VIRTQ_DESC_F_WRITE,
                next: 0,
            };

            let avail_idx = (self.rx_queue.available.idx + Wrapping(queued)).0 % self.rx_queue.size;
            self.rx_queue.available[avail_idx] = idx;

            queued += 1;
            self.rx_inflight.push_back(buf);
        }

        // notify device
        mfence();
        self.rx_queue.available.idx += Wrapping(queued);
        mfence();
        self.notify_queue(0).expect("notify queue 0 failed");

        buffer.len()
    }

    fn tx_batch(&mut self, _queue_id: u16, buffer: &mut VecDeque<Packet>) -> usize {
        // 2.6.13

        mfence();
        // free all processed packets
        while self.tx_queue.last_used_idx != self.tx_queue.used.idx {
            let used_idx =
                self.tx_queue.used[self.tx_queue.last_used_idx.0 % self.tx_queue.size].id;
            self.tx_queue.descriptors_mut()[used_idx as usize] = VirtqDesc::default();
            self.tx_queue.last_used_idx += Wrapping(1);
            mem::drop(self.tx_inflight.pop_front());
            mfence();
        }

        // add user-supplied packets to the available ring for sending out
        let mut sent = 0;
        let mut idx = 0;
        while let Some(mut packet) = buffer.pop_front() {
            // we cant use `tx_queue.free_descriptor_indices()` here due to borrowck
            while idx < self.tx_queue.size {
                let desc = &self.tx_queue.descriptors()[idx as usize];
                if desc.addr == 0 {
                    break;
                }
                idx += 1;
            }

            // queue is full; put back the packet we've taken out
            if idx == self.tx_queue.size {
                buffer.push_front(packet);
                break;
            }

            // Virtio expects a header in front of the actual packet data
            let net_header = unsafe { any_as_u8_slice(&NET_HEADER) };
            packet
                .headroom_mut(net_header.len())
                .copy_from_slice(net_header);

            self.tx_queue.descriptors_mut()[idx as usize] = VirtqDesc {
                len: (packet.len() + net_header.len()) as u32,
                addr: packet.get_phys_addr() - net_header.len(),
                flags: 0,
                next: 0,
            };

            let avail_idx = (self.tx_queue.available.idx + Wrapping(sent)).0 % self.tx_queue.size;
            self.tx_queue.available[avail_idx] = idx;

            self.tx_bytes += packet.len() as u64;
            self.tx_pkts += 1;

            sent += 1;
            self.tx_inflight.push_back(packet);
        }

        // notify device
        mfence();
        self.tx_queue.available.idx += Wrapping(sent);
        mfence();
        self.notify_queue(1).expect("notify queue 1 failed");

        sent as usize
    }

    fn read_stats(&self, stats: &mut DeviceStats) {
        stats.rx_pkts = self.rx_pkts;
        stats.tx_pkts = self.tx_pkts;
        stats.rx_bytes = self.rx_bytes;
        stats.tx_bytes = self.tx_bytes;
    }

    fn reset_stats(&mut self) {
        self.rx_pkts = 0;
        self.tx_pkts = 0;
        self.rx_bytes = 0;
        self.tx_bytes = 0;
    }

    fn get_link_speed(&self) -> u16 {
        // Virtio doesn't have a "link speed" per se so we just return something reasonable
        1000
    }
}

impl VirtioDevice {
    /// Returns an initialized `VirtioDevice` on success.
    pub fn init(pci_addr: &str) -> Result<Self, Box<dyn Error>> {
        // `getuid()` can't fail according to the man page
        if unsafe { libc::getuid() } != 0 {
            warn!("not running as root, this will probably fail");
        }

        pci::unbind_driver(pci_addr)?;
        pci::enable_dma(pci_addr)?;

        // 3.1: device initialization
        let mut bar0 = pci::pci_open_resource(pci_addr, "resource0")?;
        debug!("configuring bar0");

        // 1) Reset the device
        write_io8(&mut bar0, VIRTIO_CONFIG_STATUS_RESET, VIRTIO_PCI_STATUS)?;
        while read_io8(&mut bar0, VIRTIO_PCI_STATUS)? != VIRTIO_CONFIG_STATUS_RESET {
            thread::sleep(Duration::from_micros(100));
        }

        // 2) Set ACKNOWLEDGE status bit; OS noticed the device
        write_io8(&mut bar0, VIRTIO_CONFIG_STATUS_ACK, VIRTIO_PCI_STATUS)?;

        // 3) Set DRIVER status bit; OS can drive the device
        write_io8(&mut bar0, VIRTIO_CONFIG_STATUS_DRIVER, VIRTIO_PCI_STATUS)?;

        // 4) Negotiate features
        let host_features = read_io32(&mut bar0, VIRTIO_PCI_HOST_FEATURES)?;
        debug!("device features: {:b}", host_features);
        let required_features = (1 << VIRTIO_NET_F_CSUM) // we may offload checksumming to the device
            | (1 << VIRTIO_NET_F_GUEST_CSUM) // we can handle packets with invalid checksums
            | (1 << VIRTIO_NET_F_CTRL_VQ) // enable the control queue
            | (1 << VIRTIO_NET_F_CTRL_RX) // required to enable promiscuous mode
            | (1 << VIRTIO_NET_F_MAC) // required to read MAC address
            | (1 << VIRTIO_F_ANY_LAYOUT); // we don't make assumptions about message framing
        if (host_features & required_features) != required_features {
            debug!("device features:   {:032b}", host_features);
            debug!("required features: {:032b}", required_features);
            panic!("device does not support all required features");
        }
        debug!(
            "guest features before negotiation: {:032b}",
            read_io32(&mut bar0, VIRTIO_PCI_GUEST_FEATURES)?
        );
        write_io32(&mut bar0, required_features, VIRTIO_PCI_GUEST_FEATURES)?;
        debug!(
            "guest features after negotiation:  {:032b}",
            read_io32(&mut bar0, VIRTIO_PCI_GUEST_FEATURES)?
        );

        // 5) Skipped due to legacy interface
        // 6) Skipped due to legacy interface

        // 7) Perform network device specific initialization
        let rx_queue = Self::setup_virtqueue(&mut bar0, VirtqueueType::Receive, 0)?;
        let tx_queue = Self::setup_virtqueue(&mut bar0, VirtqueueType::Transmit, 1)?;
        let ctrl_queue = Self::setup_virtqueue(&mut bar0, VirtqueueType::Control, 2)?;

        // 2.6.13: allocate buffers to send to the device
        // we allocate more bufs than what would fit in the rx queue, because we don't want to
        // stall rx if users hold buffers for longer
        let rx_mempool = Mempool::allocate(rx_queue.size as usize * 4, 2048)?;
        let ctrl_mempool = Mempool::allocate(ctrl_queue.size as usize, 2048)?;

        mfence();

        // 8) Signal OK
        write_io8(&mut bar0, VIRTIO_CONFIG_STATUS_DRIVER_OK, VIRTIO_PCI_STATUS)?;
        info!("initialization complete");

        let mut device = VirtioDevice {
            pci_addr: pci_addr.to_owned(),
            bar0,
            rx_inflight: VecDeque::with_capacity(rx_queue.size as usize),
            tx_inflight: VecDeque::with_capacity(tx_queue.size as usize),
            rx_queue,
            tx_queue,
            ctrl_queue,
            rx_mempool,
            ctrl_mempool,
            rx_pkts: 0,
            tx_pkts: 0,
            rx_bytes: 0,
            tx_bytes: 0,
        };

        // recheck status
        device.check_pci_config_status()?;
        device.set_promiscuous(true)?;

        Ok(device)
    }

    fn notify_queue(&mut self, queue_idx: u16) -> Result<(), io::Error> {
        write_io16(&mut self.bar0, queue_idx, VIRTIO_PCI_QUEUE_NOTIFY)
    }

    fn check_pci_config_status(&mut self) -> Result<(), io::Error> {
        assert_ne!(
            read_io8(&mut self.bar0, VIRTIO_PCI_STATUS)?,
            VIRTIO_CONFIG_STATUS_FAILED,
            "device signaled unrecoverable config error"
        );
        Ok(())
    }

    fn set_promiscuous(&mut self, value: bool) -> Result<(), io::Error> {
        let command = VirtioNetCtrlPromisc::new(value).into();
        self.send_command(&command)?;
        info!("set promiscuous mode to {}", value);
        Ok(())
    }

    fn send_command<C: VirtioNetCtrlCommand>(
        &mut self,
        command: &VirtioNetCtrl<C>,
    ) -> Result<(), io::Error> {
        let cmd_len = mem::size_of::<VirtioNetCtrl<C>>();
        mfence();
        let idx = self
            .ctrl_queue
            .free_descriptor_indices()
            .next()
            .expect("command queue full");
        debug!(
            "found free descriptor at index {} (out of {})",
            idx, self.ctrl_queue.size
        );

        let mut buf = memory::alloc_pkt(&self.ctrl_mempool, cmd_len).unwrap();
        buf.copy_from_slice(unsafe { any_as_u8_slice(command) });

        // one descriptor for everything; should work as we negotiated VIRTIO_F_ANY_LAYOUT during
        // init but doesn't in practice
        // let desc = &mut ctrl_queue.descriptors_mut()[idx as usize];
        // desc.len = cmd_len as u32;
        // desc.addr = buf.get_phys_addr();
        // desc.flags = VIRTQ_DESC_F_WRITE;
        // desc.next = 0;

        // device-readable payload: cmd header
        let desc = &mut self.ctrl_queue.descriptors_mut()[idx as usize];
        desc.len = 2;
        desc.addr = buf.get_phys_addr();
        desc.flags = VIRTQ_DESC_F_NEXT;
        desc.next = idx + 1;
        // device-readable payload: cmd data
        let desc = &mut self.ctrl_queue.descriptors_mut()[(idx + 1) as usize];
        desc.len = mem::size_of::<C>() as u32;
        desc.addr = buf.get_phys_addr() + 2;
        desc.flags = VIRTQ_DESC_F_NEXT;
        desc.next = idx + 2;
        // device-writable tail: ack flag
        let desc = &mut self.ctrl_queue.descriptors_mut()[(idx + 2) as usize];
        desc.len = 1;
        desc.addr = buf.get_phys_addr() + 2 + mem::size_of::<C>();
        desc.flags = VIRTQ_DESC_F_WRITE;
        desc.next = 0;

        let avail_idx = self.ctrl_queue.available.idx.0 % self.ctrl_queue.size;
        self.ctrl_queue.available[avail_idx] = idx;

        mfence();
        self.ctrl_queue.available.idx += Wrapping(1);
        mfence();
        self.notify_queue(2)?;

        #[allow(clippy::while_immutable_condition)]
        while self.ctrl_queue.last_used_idx == self.ctrl_queue.used.idx {
            mfence();
            debug!("waiting...");
            thread::sleep(Duration::from_millis(100));
        }
        assert_eq!(
            (self.ctrl_queue.last_used_idx + Wrapping(1)).0 % self.ctrl_queue.size,
            self.ctrl_queue.used.idx.0
        );
        self.ctrl_queue.last_used_idx = self.ctrl_queue.used.idx;

        let used = &self.ctrl_queue.used[self.ctrl_queue.used.idx.0];

        debug!(
            "used ctrl buffer @ {:p} id {} len {}",
            &used, used.id, used.len
        );
        assert_eq!(
            used.id, idx,
            "used buffer has different index than the one sent"
        );

        // ensure that the command was correctly acknowledged
        let ack = unsafe { (*(buf.get_virt_addr() as *const VirtioNetCtrl<C>)).ack };
        assert_eq!(
            ack, VIRTIO_NET_OK,
            "sent command was not acknowledged correctly"
        );

        Ok(())
    }

    fn setup_virtqueue(
        bar0: &mut File,
        virtq_type: VirtqueueType,
        index: u16,
    ) -> Result<Virtqueue, Box<dyn Error>> {
        assert!(
            virtq_type.is_valid_index(index),
            "invalid queue index {} for {:?}",
            index,
            virtq_type
        );

        // 4.1.5.1.3: create virtqueue itself
        write_io16(bar0, index, VIRTIO_PCI_QUEUE_SEL)?;
        let max_queue_size = read_io16(bar0, VIRTIO_PCI_QUEUE_NUM)?;
        debug!(
            "max queue size of queue #{} ({:?}): {}",
            index, virtq_type, max_queue_size
        );
        assert!(max_queue_size > 0, "queue #{} doesn't exist", index);
        let virtqueue_mem_size = Virtqueue::size(max_queue_size);
        let mem: Dma<u8> = Dma::allocate(virtqueue_mem_size, true)?;
        debug!(
            "allocated {:#x} bytes for virtqueue at {:p}",
            virtqueue_mem_size, mem.virt
        );
        write_io32(
            bar0,
            (mem.phys >> VIRTIO_PCI_QUEUE_ADDR_SHIFT) as u32,
            VIRTIO_PCI_QUEUE_PFN,
        )?;

        // DMA memory already follows stricter alignment than `VirtqDesc`
        #[allow(clippy::cast_ptr_alignment)]
        let mut virtq = unsafe { Virtqueue::new(max_queue_size, mem.virt as *mut VirtqDesc) };
        debug!("virtq desc:  {:p}", virtq.desc);
        debug!("virtq avail: {:p}", virtq.available.ptr);
        debug!("virtq used:  {:p}", virtq.used.ptr);
        for i in 0..virtq.size {
            virtq.descriptors_mut()[i as usize] = VirtqDesc::default();
            virtq.available[i] = 0;
            virtq.used[i] = VirtqUsedElem::default();
        }
        virtq.available.idx = Wrapping(0);
        virtq.used.idx = Wrapping(0);

        // optimization hint to not get interrupted when the device consumes a buffer
        virtq.available.flags = VIRTQ_AVAIL_F_NO_INTERRUPT;
        virtq.used.flags = 0;

        Ok(virtq)
    }
}

#[derive(Debug, Clone, Copy)]
enum VirtqueueType {
    Receive,
    Transmit,
    Control,
}

impl VirtqueueType {
    fn is_valid_index(self, index: u16) -> bool {
        // we don't support VIRTIO_NET_F_MQ atm so there are only 3 queues
        let valid = match self {
            VirtqueueType::Receive => 0,
            VirtqueueType::Transmit => 1,
            VirtqueueType::Control => 2,
        };
        index == valid
    }
}

pub struct Virtqueue {
    size: u16,
    desc: *mut VirtqDesc,
    available: RingWrapper<VirtqAvail>,
    used: RingWrapper<VirtqUsed>,
    last_used_idx: Wrapping<u16>,
}

impl Virtqueue {
    unsafe fn new(size: u16, ptr: *mut VirtqDesc) -> Virtqueue {
        let size_usize = size as usize;
        let avail = ptr.wrapping_add(size_usize) as *mut VirtqAvail;
        let used =
            align((*avail).ring.as_mut_ptr().wrapping_add(size_usize) as _) as *mut VirtqUsed;

        Virtqueue {
            size,
            desc: ptr,
            available: RingWrapper { ptr: avail, size },
            used: RingWrapper { ptr: used, size },
            last_used_idx: Wrapping(0),
        }
    }

    pub fn descriptors(&self) -> &[VirtqDesc] {
        unsafe { slice::from_raw_parts(self.desc, self.size as usize) }
    }

    pub fn descriptors_mut(&mut self) -> &mut [VirtqDesc] {
        unsafe { slice::from_raw_parts_mut(self.desc, self.size as usize) }
    }

    pub fn free_descriptor_indices(&self) -> impl Iterator<Item = u16> + '_ {
        self.descriptors()
            .iter()
            .enumerate()
            .filter_map(|(idx, desc)| {
                if desc.addr == 0 {
                    Some(idx as u16)
                } else {
                    None
                }
            })
    }

    fn size(queue_size: u16) -> usize {
        // from 2.6.2
        let queue_size = queue_size as usize;
        align(mem::size_of::<VirtqDesc>() * queue_size + mem::size_of::<u16>() * (3 + queue_size))
            + align(mem::size_of::<u16>() * 3 + mem::size_of::<VirtqUsedElem>() * queue_size)
    }
}

struct RingWrapper<T: Ring> {
    ptr: *mut T,
    size: u16,
}

impl<T: Ring> Index<u16> for RingWrapper<T> {
    type Output = <T as Ring>::Element;
    fn index(&self, idx: u16) -> &Self::Output {
        assert!(
            idx < self.size,
            "index {} is greater than queue size {}",
            idx,
            self.size
        );
        unsafe { &*self.ring().add(idx as usize) }
    }
}

impl<T: Ring> IndexMut<u16> for RingWrapper<T> {
    fn index_mut(&mut self, idx: u16) -> &mut Self::Output {
        assert!(
            idx < self.size,
            "index {} is greater than queue size {}",
            idx,
            self.size
        );
        unsafe { &mut *self.ring_mut().add(idx as usize) }
    }
}

impl<T: Ring> Deref for RingWrapper<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T: Ring> DerefMut for RingWrapper<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

fn mfence() {
    atomic::fence(Ordering::SeqCst);
}

fn align(ptr: usize) -> usize {
    ptr + (ptr as *const u8).align_offset(QUEUE_ALIGNMENT)
}

// from https://stackoverflow.com/a/42186553
/// Creates a read-only view into the bytes of any sized type `T`. `T` must not contain
/// (uninitialized) padding bytes as reading them invokes undefined behavior.
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align() {
        // we use a function based on libstd; just checking against the macro from the spec to make
        // sure we have the same behavior
        fn align_spec(x: usize) -> usize {
            (x + (QUEUE_ALIGNMENT - 1)) & !(QUEUE_ALIGNMENT - 1)
        }

        for i in 0..1_000_000 {
            let aligned = align(i);
            assert!(aligned >= i);
            assert!(aligned - i < QUEUE_ALIGNMENT);
            assert_eq!(aligned % QUEUE_ALIGNMENT, 0);
            assert_eq!(aligned, align_spec(i));
        }
    }
}
