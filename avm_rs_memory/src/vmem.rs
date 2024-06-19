use std::fmt::Display;

use crate::{access_memory, mem::{Memory, MemorySliceTrait}, pointer::Pointer, share_memory};

#[derive(Debug)]
pub struct MappedMemory {
    pub address: usize,
    pub size: usize,
}

#[derive(Debug)]
pub struct VirtualMemory {
    memory: Memory,
    mapped_memory: Vec<MappedMemory>,
    last_address: usize,
}

impl VirtualMemory {
    pub fn new(memory: Memory) -> Self {
        Self {
            mapped_memory: Vec::new(),
            memory: memory,
            last_address: 0,
        }
    }

    fn find_fit(&mut self, size: usize) -> Option<usize> {
        let address = self.last_address;
        if address + size > access_memory!(self.memory).len() {
            return None;
        }
        self.last_address = address + size;
        Some(address)
    }

    fn find_mapped_memory(&self, address: usize) -> Option<&MappedMemory> {
        self.mapped_memory.iter().find(|x| x.address <= address && x.address + x.size > address)
    }

    fn map(&mut self, address: usize, size: usize) -> usize {
        self.mapped_memory.push(MappedMemory { address, size });
        self.mapped_memory.len() - 1
    }

    fn unmap(&mut self, index: usize) {
        self.mapped_memory.remove(index);
    }

    pub fn allocate(&mut self, size: usize) -> Pointer {
        let address = self.find_fit(size);
        match address {
            Some(address) => {
                self.map(address, size);
                Pointer::new(share_memory!(self.memory), address, size)
            },
            None => {
                let left_memory = access_memory!(self.memory).len() - self.last_address;
                panic!("Out of memory, memory left is 0x{left_memory:04X}({left_memory}) but trying to allocate 0x{size:04X}({size})");
            },
        }
    }

    pub fn deallocate(&mut self, address: usize) {
        if let Some(mapped_memory) = self.find_mapped_memory(address) {
            self.unmap(mapped_memory.address);
            return;
        }
        panic!("Unmapped address: 0x{address:04X}");
    }

    pub fn free(&mut self, pointer: &mut Pointer) {
        self.deallocate(pointer.address);
        pointer.size = 0;
    }
}

impl Display for VirtualMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.mapped_memory.len() {
            write!(f, "address: 0x{:04X}, size: 0x{:04X}", self.mapped_memory[i].address, self.mapped_memory[i].address + self.mapped_memory[i].size)?;
            if i < self.mapped_memory.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}