use avm_rs_memory::mem::Memory;

fn main() {
    let mut memory = Memory::new(128);
    memory.write_string(0, "hello!");
    println!("{memory}");
}