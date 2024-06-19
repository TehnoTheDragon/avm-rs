use std::ops::{AddAssign, SubAssign};

use crate::{access_memory, mem::{Memory, MemorySliceTrait}};

#[derive(Debug)]
pub struct Pointer {
    memory: Memory,
    address: usize,
    size: usize,
}

impl Pointer {
    pub fn new(memory: Memory, address: usize, size: usize) -> Self {
        Self {
            memory: memory,
            address: address,
            size: size,
        }
    }
}

impl MemorySliceTrait for Pointer {
    fn assert_access_range(&self, at: usize, length: usize) {
        if at + length > self.size {
            let real_at = self.address + at;
            panic!("Access volatile memory at 0x{real_at:04X}({real_at}) with 0x{length:04X}({length}) bytes");
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn read_u8(&self, at: usize) -> u8 {
        self.assert_access_range(at, 1);
        access_memory!(self.memory).read_u8(self.address + at)
    }

    fn read_i8(&self, at: usize) -> i8 {
        self.assert_access_range(at, 1);
        access_memory!(self.memory).read_i8(self.address + at)
    }

    fn write_u8(&mut self, at: usize, value: u8) {
        self.assert_access_range(at, 1);
        access_memory!(self.memory).write_u8(self.address + at, value);
    }

    fn write_i8(&mut self, at: usize, value: i8) {
        self.assert_access_range(at, 1);
        access_memory!(self.memory).write_i8(self.address + at, value);
    }

    fn read_u16(&self, at: usize) -> u16 {
        self.assert_access_range(at, 2);
        access_memory!(self.memory).read_u16(self.address + at)
    }

    fn read_i16(&self, at: usize) -> i16 {
        self.assert_access_range(at, 2);
        access_memory!(self.memory).read_i16(self.address + at)
    }

    fn write_u16(&mut self, at: usize, value: u16) {
        self.assert_access_range(at, 2);
        access_memory!(self.memory).write_u16(self.address + at, value);
    }

    fn write_i16(&mut self, at: usize, value: i16) {
        self.assert_access_range(at, 2);
        access_memory!(self.memory).write_i16(self.address + at, value);
    }

    fn read_u32(&self, at: usize) -> u32 {
        self.assert_access_range(at, 4);
        access_memory!(self.memory).read_u32(self.address + at)
    }

    fn read_i32(&self, at: usize) -> i32 {
        self.assert_access_range(at, 4);
        access_memory!(self.memory).read_i32(self.address + at)
    }

    fn write_u32(&mut self, at: usize, value: u32) {
        self.assert_access_range(at, 4);
        access_memory!(self.memory).write_u32(self.address + at, value);
    }

    fn write_i32(&mut self, at: usize, value: i32) {
        self.assert_access_range(at, 4);
        access_memory!(self.memory).write_i32(self.address + at, value);
    }

    fn read_u64(&self, at: usize) -> u64 {
        self.assert_access_range(at, 8);
        access_memory!(self.memory).read_u64(self.address + at)
    }

    fn read_i64(&self, at: usize) -> i64 {
        self.assert_access_range(at, 8);
        access_memory!(self.memory).read_i64(self.address + at)
    }

    fn write_u64(&mut self, at: usize, value: u64) {
        self.assert_access_range(at, 8);
        access_memory!(self.memory).write_u64(self.address + at, value);
    }

    fn write_i64(&mut self, at: usize, value: i64) {
        self.assert_access_range(at, 8);
        access_memory!(self.memory).write_i64(self.address + at, value);
    }

    fn read_u128(&self, at: usize) -> u128 {
        self.assert_access_range(at, 16);
        access_memory!(self.memory).read_u128(self.address + at)
    }

    fn read_i128(&self, at: usize) -> i128 {
        self.assert_access_range(at, 16);
        access_memory!(self.memory).read_i128(self.address + at)
    }

    fn write_u128(&mut self, at: usize, value: u128) {
        self.assert_access_range(at, 16);
        access_memory!(self.memory).write_u128(self.address + at, value);
    }

    fn write_i128(&mut self, at: usize, value: i128) {
        self.assert_access_range(at, 16);
        access_memory!(self.memory).write_i128(self.address + at, value);
    }

    fn read_f32(&self, at: usize) -> f32 {
        self.assert_access_range(at, 4);
        access_memory!(self.memory).read_f32(self.address + at)
    }

    fn write_f32(&mut self, at: usize, value: f32) {
        self.assert_access_range(at, 4);
        access_memory!(self.memory).write_f32(self.address + at, value);
    }

    fn read_f64(&self, at: usize) -> f64 {
        self.assert_access_range(at, 8);
        access_memory!(self.memory).read_f64(self.address + at)
    }

    fn write_f64(&mut self, at: usize, value: f64) {
        self.assert_access_range(at, 8);
        access_memory!(self.memory).write_f64(self.address + at, value);
    }

    fn read_bytes(&self, at: usize, length: usize) -> Vec<u8> {
        self.assert_access_range(at, length);
        access_memory!(self.memory).read_bytes(self.address + at, length).to_vec()
    }

    fn write_bytes(&mut self, at: usize, value: &[u8]) {
        self.assert_access_range(at, value.len());
        access_memory!(self.memory).write_bytes(self.address + at, value);
    }

    fn read_string(&self, at: usize, length: usize) -> String {
        self.assert_access_range(at, length);
        access_memory!(self.memory).read_string(self.address + at, length)
    }

    fn write_string(&mut self, at: usize, value: &str) {
        self.assert_access_range(at, value.len());
        access_memory!(self.memory).write_string(self.address + at, value);
    }
}