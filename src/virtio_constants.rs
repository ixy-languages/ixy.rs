#![allow(dead_code)]
#![allow(clippy::all)]
// Copied from the ixy C driver
// Amended with updates from the newer Virtio spec v1.1

/*-
 *   BSD LICENSE
 *
 *   Copyright(c) 2010-2014 Intel Corporation. All rights reserved.
 *   All rights reserved.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following conditions
 *   are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *     * Neither the name of Intel Corporation nor the names of its
 *       contributors may be used to endorse or promote products derived
 *       from this software without specific prior written permission.
 *
 *   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 *   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 *   OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 *   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 *   DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 *   THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 *   (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 *   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*
 * VirtIO Header, located in BAR 0.
 */
pub const VIRTIO_PCI_HOST_FEATURES: u64        = 0;  /* host's supported features (32bit, RO)*/
pub const VIRTIO_PCI_GUEST_FEATURES: u64       = 4;  /* guest's supported features (32, RW) */
pub const VIRTIO_PCI_QUEUE_PFN: u64            = 8;  /* physical address of VQ (32, RW) */
pub const VIRTIO_PCI_QUEUE_NUM: u64            = 12; /* number of ring entries (16, RO) */
pub const VIRTIO_PCI_QUEUE_SEL: u64            = 14; /* current VQ selection (16, RW) */
pub const VIRTIO_PCI_QUEUE_NOTIFY: u64         = 16; /* notify host regarding VQ (16, RW) */
pub const VIRTIO_PCI_STATUS: u64               = 18; /* device status register (8, RW) */
pub const VIRTIO_PCI_ISR: u64                  = 19; /* interrupt status register, reading also clears the register (8, RO) */
/* Only if MSIX is enabled: */
pub const VIRTIO_MSI_CONFIG_VECTOR: u64        = 20; /* configuration change vector (16, RW) */
pub const VIRTIO_MSI_QUEUE_VECTOR: u64         = 22; /* vector for selected VQ notifications (16, RW) */

/* Status byte for guest to report progress. */
pub const VIRTIO_CONFIG_STATUS_RESET: u8       = 0x00;
pub const VIRTIO_CONFIG_STATUS_ACK: u8         = 0x01;
pub const VIRTIO_CONFIG_STATUS_DRIVER: u8      = 0x02;
pub const VIRTIO_CONFIG_STATUS_DRIVER_OK: u8   = 0x04;
pub const VIRTIO_CONFIG_STATUS_FEATURES_OK: u8 = 0x08;
pub const VIRTIO_CONFIG_STATUS_FAILED: u8      = 0x80;

/*
 * How many bits to shift physical queue address written to QUEUE_PFN.
 * 12 is historical, and due to x86 page size.
 */
pub const VIRTIO_PCI_QUEUE_ADDR_SHIFT: usize   = 12;

/* This marks a buffer as continuing via the next field. */
pub const VIRTQ_DESC_F_NEXT: u16               = 1;
/* This marks a buffer as write-only (otherwise read-only). */
pub const VIRTQ_DESC_F_WRITE: u16              = 2;
/* This means the buffer contains a list of buffer descriptors. */
pub const VIRTQ_DESC_F_INDIRECT: u16           = 4;

/* The feature bitmap for virtio net */
pub const VIRTIO_NET_F_CSUM: usize             = 0;  /* Host handles pkts w/ partial csum */
pub const VIRTIO_NET_F_GUEST_CSUM: usize       = 1;  /* Guest handles pkts w/ partial csum */
pub const VIRTIO_NET_F_MTU: usize              = 3;  /* Initial MTU advice. */
pub const VIRTIO_NET_F_MAC: usize              = 5;  /* Host has given MAC address. */
pub const VIRTIO_NET_F_GUEST_TSO4: usize       = 7;  /* Guest can handle TSOv4 in. */
pub const VIRTIO_NET_F_GUEST_TSO6: usize       = 8;  /* Guest can handle TSOv6 in. */
pub const VIRTIO_NET_F_GUEST_ECN: usize        = 9;  /* Guest can handle TSO[6] w/ ECN in. */
pub const VIRTIO_NET_F_GUEST_UFO: usize        = 10; /* Guest can handle UFO in. */
pub const VIRTIO_NET_F_HOST_TSO4: usize        = 11; /* Host can handle TSOv4 in. */
pub const VIRTIO_NET_F_HOST_TSO6: usize        = 12; /* Host can handle TSOv6 in. */
pub const VIRTIO_NET_F_HOST_ECN: usize         = 13; /* Host can handle TSO[6] w/ ECN in. */
pub const VIRTIO_NET_F_HOST_UFO: usize         = 14; /* Host can handle UFO in. */
pub const VIRTIO_NET_F_MRG_RXBUF: usize        = 15; /* Host can merge receive buffers. */
pub const VIRTIO_NET_F_STATUS: usize           = 16; /* virtio_net_config.status available */
pub const VIRTIO_NET_F_CTRL_VQ: usize          = 17; /* Control channel available */
pub const VIRTIO_NET_F_CTRL_RX: usize          = 18; /* Control channel RX mode support */
pub const VIRTIO_NET_F_CTRL_VLAN: usize        = 19; /* Control channel VLAN filtering */
pub const VIRTIO_NET_F_CTRL_RX_EXTRA: usize    = 20; /* Extra RX mode control support */
pub const VIRTIO_NET_F_GUEST_ANNOUNCE: usize   = 21; /* Guest can announce device on the network */
pub const VIRTIO_NET_F_MQ: usize               = 22; /* Device supports Receive Flow Steering */
pub const VIRTIO_NET_F_CTRL_MAC_ADDR: usize    = 23; /* Set MAC address */

/* Do we get callbacks when the ring is completely used, even if we've suppressed them? */
pub const VIRTIO_F_NOTIFY_ON_EMPTY: usize      = 24;

/* Can the device handle any descriptor layout? */
pub const VIRTIO_F_ANY_LAYOUT: usize           = 27;

/* We support indirect buffer descriptors */
pub const VIRTIO_RING_F_INDIRECT_DESC: usize   = 28;

pub const VIRTIO_F_VERSION_1: usize            = 32;
pub const VIRTIO_F_IOMMU_PLATFORM: usize       = 33;


/**
 * Control the RX mode, ie. promiscuous, allmulti, etc...
 * All commands require an "out" sg entry containing a 1 byte
 * state value, zero = disable, non-zero = enable.  Commands
 * 0 and 1 are supported with the VIRTIO_NET_F_CTRL_RX feature.
 * Commands 2-5 are added with VIRTIO_NET_F_CTRL_RX_EXTRA.
 */
pub const VIRTIO_NET_CTRL_RX: u8               = 0;
pub const VIRTIO_NET_CTRL_RX_PROMISC: u8       = 0;
pub const VIRTIO_NET_CTRL_RX_ALLMULTI: u8      = 1;
pub const VIRTIO_NET_CTRL_RX_ALLUNI: u8        = 2;
pub const VIRTIO_NET_CTRL_RX_NOMULTI: u8       = 3;
pub const VIRTIO_NET_CTRL_RX_NOUNI: u8         = 4;
pub const VIRTIO_NET_CTRL_RX_NOBCAST: u8       = 5;

pub const VIRTIO_NET_OK: u8                    = 0;
pub const VIRTIO_NET_ERR: u8                   = 1;

pub const VIRTIO_MAX_CTRL_DATA: usize          = 2048;

/**
 * This is the first element of the scatter-gather list.  If you don't
 * specify GSO or CSUM features, you can simply ignore the header.
 */
#[repr(C)]
pub struct virtio_net_hdr {
    pub flags: u8,
    pub gso_type: u8,
    pub hdr_len: u16,     // Ethernet + IP + tcp/udp hdrs
    pub gso_size: u16,    // Bytes to append to hdr_len per frame
    pub csum_start: u16,  // Position to start checksumming from
    pub csum_offset: u16, // Offset after that to place checksum
}

pub const VIRTIO_NET_HDR_F_NEEDS_CSUM: u8      = 1;    /**< Use csum_start,csum_offset*/
pub const VIRTIO_NET_HDR_F_DATA_VALID: u8      = 2;    /**< Checksum is valid */

pub const VIRTIO_NET_HDR_GSO_NONE: u8          = 0;    /**< Not a GSO frame */
pub const VIRTIO_NET_HDR_GSO_TCPV4: u8         = 1;    /**< GSO frame, IPv4 TCP (TSO) */
pub const VIRTIO_NET_HDR_GSO_UDP: u8           = 3;    /**< GSO frame, IPv4 UDP (UFO) */
pub const VIRTIO_NET_HDR_GSO_TCPV6: u8         = 4;    /**< GSO frame, IPv6 TCP */
pub const VIRTIO_NET_HDR_GSO_ECN: u8           = 0x80; /**< TCP has ECN set */


/* The Host uses this in used->flags to advise the Guest: don't kick me
 * when you add a buffer.  It's unreliable, so it's simply an
 * optimization.  Guest will still kick if it's out of buffers. */
pub const VIRTQ_USED_F_NO_NOTIFY: u16          = 1;
/* The Guest uses this in avail->flags to advise the Host: don't
 * interrupt me when you consume a buffer.  It's unreliable, so it's
 * simply an optimization. */
pub const VIRTQ_AVAIL_F_NO_INTERRUPT: u16      = 1;


use std::num::Wrapping;

/* VirtIO ring descriptors: 16 bytes.
 * These can chain together via "next". */
#[repr(C)]
#[derive(Default)]
pub struct VirtqDesc {
    pub addr: usize, /* Address (guest-physical). */
    pub len: u32,    /* Length. */
    pub flags: u16,  /* The flags as indicated above. */
    pub next: u16,   /* We chain unused descriptors via this. */
}

#[repr(C)]
pub struct VirtqAvail {
    pub flags: u16,
    pub idx: Wrapping<u16>,
    pub ring: [u16; 0],
}

#[repr(C)]
pub struct VirtqUsed {
    pub flags: u16,
    pub idx: Wrapping<u16>,
    pub ring: [VirtqUsedElem; 0],
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct VirtqUsedElem {
    /* Index of start of used descriptor chain. */
    pub id: u16,
    pub _padding: u16,
    /* Total length of the descriptor chain which was written to. */
    pub len: u32,
}

pub trait Ring {
    type Element;
    fn ring(&self) -> *const Self::Element;
    fn ring_mut(&mut self) -> *mut Self::Element;
}

impl Ring for VirtqAvail {
    type Element = u16;
    fn ring(&self) -> *const u16 {
        self.ring.as_ptr()
    }
    fn ring_mut(&mut self) -> *mut u16 {
        self.ring.as_mut_ptr()
    }
}

impl Ring for VirtqUsed {
    type Element = VirtqUsedElem;
    fn ring(&self) -> *const VirtqUsedElem {
        self.ring.as_ptr()
    }
    fn ring_mut(&mut self) -> *mut VirtqUsedElem {
        self.ring.as_mut_ptr()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VirtioNetCtrl<T: VirtioNetCtrlCommand> {
    pub class: u8,
    pub command: u8,
    pub command_data: T,
    pub ack: u8,
}

impl<T: VirtioNetCtrlCommand> From<T> for VirtioNetCtrl<T> {
    fn from(command_data: T) -> VirtioNetCtrl<T> {
        VirtioNetCtrl {
            class: T::CLASS,
            command: T::COMMAND,
            command_data,
            ack: 0,
        }
    }
}

/// A specific command to be sent through the control queue (wrapped in a [`VirtioNetCtrl`])
pub trait VirtioNetCtrlCommand {
    const CLASS: u8;
    const COMMAND: u8;
}

#[derive(Debug)]
pub struct VirtioNetCtrlPromisc(u8);

impl VirtioNetCtrlCommand for VirtioNetCtrlPromisc {
    const CLASS: u8   = VIRTIO_NET_CTRL_RX;
    const COMMAND: u8 = VIRTIO_NET_CTRL_RX_PROMISC;
}

impl VirtioNetCtrlPromisc {
    pub fn new(on: bool) -> VirtioNetCtrlPromisc {
        VirtioNetCtrlPromisc(on as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::mem;
    #[test]
    fn static_type_sizes() {
        assert_eq!(mem::size_of::<VirtioNetCtrl<VirtioNetCtrlPromisc>>(), 4);
    }
}
