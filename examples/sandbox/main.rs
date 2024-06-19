use avm_rs_memory::{access_memory, mem::{create_memory, MemorySliceTrait}, share_memory, vmem::VirtualMemory};

fn main() {
    let mut memory = create_memory(32);
    let mut virtual_memory = VirtualMemory::new(share_memory!(memory));
    virtual_memory.allocate(4);
    let mut ptr1 = virtual_memory.allocate(4);
    ptr1.write_u8(0, 0x41);
    ptr1.write_u8(1, 0x41);
    ptr1.write_u8(2, 0x41);
    ptr1.write_u8(3, 0x41);
    println!("{}", access_memory!(memory));
}