use avm_rs_memory::{access_memory, mem::{create_memory, MemorySliceTrait}, share_memory, vmem::VirtualMemory, wrappers::stack::Stack};

fn main() {
    let mut memory = create_memory(1024 * 512);
    let mut virtual_memory = VirtualMemory::new(share_memory!(memory));
    let mut stack = Stack::new(virtual_memory.allocate(1024));
    println!("{}", access_memory!(memory));
}