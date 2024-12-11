extern crate kvm_bindings;
extern crate kvm_ioctls;
use kvm_bindings::{
    kvm_create_guest_memfd, kvm_enable_cap, kvm_userspace_memory_region2, KVM_CAP_GUEST_MEMFD,
    KVM_CAP_USER_MEMORY2, KVM_MEM_GUEST_MEMFD, KVM_MEMORY_ATTRIBUTE_PRIVATE
};
use kvm_ioctls::Kvm;
use kvm_ioctls::Cap;
use std::os::fd::RawFd;

#[cfg(target_arch = "x86_64")]
fn run() {
    let kvm = Kvm::new().unwrap();
    // VM type not yet available in kvm_bindings 0.10.0
    // KVM_X86_SNP_VM
    let vm = kvm.create_vm_with_type(1).unwrap();


    let mut config = kvm_enable_cap {
        cap: KVM_CAP_GUEST_MEMFD,
        ..Default::default()
    };

    println!("kvm guest memfd is {:?}", kvm.check_extension(Cap::GuestMemfd));
    println!("kvm user memory2 is {:?}", kvm.check_extension(Cap::UserMemory2));
    println!("kvm memory attributes is {:?}", kvm.check_extension(Cap::MemoryAttributes));
    println!("kvm memory attributes private is {:?}", kvm.check_extension_int(Cap::MemoryAttributes));
    println!("vm guest memfd is {:?}", vm.check_extension(Cap::GuestMemfd));
    println!("vm user memory2 is {:?}", vm.check_extension(Cap::UserMemory2));
    println!("vm memory attributes is {:?}", vm.check_extension(Cap::MemoryAttributes));
    println!("vm memory attributes private is {:?}", vm.check_extension_int(Cap::MemoryAttributes));

    let gmem = kvm_create_guest_memfd {
        size: 1 << 48,  // minimum size is 0x1000
        flags: 0,
        reserved: [0; 6],
    };

    let fd: RawFd = unsafe { vm.create_guest_memfd(gmem).unwrap() };
    println!("fd = {:?}", fd);


    let address_space = unsafe { libc::mmap(0 as _, 10000, 3, 34, -1, 0) };
    let userspace_addr = address_space as *const u8 as u64;
    println!("userspace_addr: {:#x}", userspace_addr);
    let mem_region = kvm_userspace_memory_region2 {
        slot: 0,
        flags: KVM_MEM_GUEST_MEMFD,
        guest_phys_addr: 0x1000 as u64, // Multiply of 0x1000 this number is probably just some
                                         // offset starting for the guest.
        memory_size: 0x1000 as u64, // Multiply of 0x1000
        userspace_addr: userspace_addr,  // This number must be the mmapped address.
        guest_memfd_offset: 0x0000, // guest_memfd_offset + memory_size < file descriptor size.
        guest_memfd: fd as u32,
        pad1: 0,
        pad2: [0; 14],
    };
    let result = unsafe {
        vm.set_user_memory_region2(mem_region)
    };
    if result.is_err() {
        println!("failed to set_user_memory_region2 {:?}", result);
    } else {
        println!("successful");
    }

    let address_space1 = unsafe { libc::mmap(0 as _, 10000, 3, 34, -1, 0) };
    let userspace_addr1 = address_space1 as *const u8 as u64;
    println!("userspace_addr: {:#x}", userspace_addr1);
    let mem_region1 = kvm_userspace_memory_region2 {
        slot: 0,
        flags: KVM_MEM_GUEST_MEMFD,
        guest_phys_addr: 0x3000 as u64, // Multiply of 0x1000 this number is probably just some
                                         // offset starting for the guest.
        memory_size: 0x1000 as u64, // Multiply of 0x1000
        userspace_addr: userspace_addr1,  // This number must be the mmapped address.
        guest_memfd_offset: 0x3000, // guest_memfd_offset + memory_size < file descriptor size.
        guest_memfd: fd as u32,
        pad1: 0,
        pad2: [0; 14],
    };
    let result1 = unsafe {
        vm.set_user_memory_region2(mem_region)
    };
    if result1.is_err() {
        println!("failed to set_user_memory_region2 {:?}", result1);
    } else {
        println!("successful");
    }
}
fn main(){
    run();
}

