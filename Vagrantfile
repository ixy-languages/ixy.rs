Vagrant.configure('2') do |config|
  config.vm.box = 'debian/buster64'

  config.vm.provider :virtualbox do |vb|
    vb.customize ['modifyvm', :id, '--nicpromisc2', 'allow-all']
    vb.customize ['modifyvm', :id, '--nicpromisc3', 'allow-all']
  end

  config.vm.provider :libvirt do |libvirt|
    # NOTE: Sometimes `./setup-hugetlbfs.sh` hangs; `vagrant ssh`ing into the VM unblocks it somehow.
    # TODO: Disable STP once https://github.com/vagrant-libvirt/vagrant-libvirt/pull/1038 is merged
    #       (we don't need it at and it produces some unwanted packets that are captured)
    #       For now you can use `sudo brctl stp <virbr> off`
  end

  config.vm.provision 'shell', privileged: false, inline: <<-SHELL
    sudo apt-get update
    sudo apt-get install -y curl
    curl -sSf https://sh.rustup.rs | sh -s -- -y
  SHELL

  # IPs are required but not actually used by the examples
  config.vm.define :pktgen do |config|
    config.vm.network 'private_network', ip: '10.100.1.11', nic_type: 'virtio', virtualbox__intnet: 'ixy_net1',
                                         libvirt__network_name: 'ixy_net1', libvirt__dhcp_enabled: false
  end

  config.vm.define :fwd do |config|
    config.vm.network 'private_network', ip: '10.100.1.12', nic_type: 'virtio', virtualbox__intnet: 'ixy_net1',
                                         libvirt__network_name: 'ixy_net1', libvirt__dhcp_enabled: false
    config.vm.network 'private_network', ip: '10.100.2.11', nic_type: 'virtio', virtualbox__intnet: 'ixy_net2',
                                         libvirt__network_name: 'ixy_net2', libvirt__dhcp_enabled: false
  end

  config.vm.define :pcap do |config|
    config.vm.network 'private_network', ip: '10.100.2.12', nic_type: 'virtio', virtualbox__intnet: 'ixy_net2',
                                         libvirt__network_name: 'ixy_net2', libvirt__dhcp_enabled: false
  end
end
