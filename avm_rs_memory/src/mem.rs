use std::{fmt::Display, sync::{Arc, Mutex}};

pub trait MemorySliceTrait {
    fn len(&self) -> usize;
    fn assert_access_range(&self, at: usize, length: usize);

    fn read_u8(&self, at: usize) -> u8;
    fn read_i8(&self, at: usize) -> i8;
    fn write_u8(&mut self, at: usize, value: u8);
    fn write_i8(&mut self, at: usize, value: i8);

    fn read_u16(&self, at: usize) -> u16;
    fn read_i16(&self, at: usize) -> i16;
    fn write_u16(&mut self, at: usize, value: u16);
    fn write_i16(&mut self, at: usize, value: i16);

    fn read_u32(&self, at: usize) -> u32;
    fn read_i32(&self, at: usize) -> i32;
    fn write_u32(&mut self, at: usize, value: u32);
    fn write_i32(&mut self, at: usize, value: i32);

    fn read_u64(&self, at: usize) -> u64;
    fn read_i64(&self, at: usize) -> i64;
    fn write_u64(&mut self, at: usize, value: u64);
    fn write_i64(&mut self, at: usize, value: i64);

    fn read_u128(&self, at: usize) -> u128;
    fn read_i128(&self, at: usize) -> i128;
    fn write_u128(&mut self, at: usize, value: u128);
    fn write_i128(&mut self, at: usize, value: i128);

    fn read_f32(&self, at: usize) -> f32;
    fn write_f32(&mut self, at: usize, value: f32);

    fn read_f64(&self, at: usize) -> f64;
    fn write_f64(&mut self, at: usize, value: f64);

    fn read_bytes(&self, at: usize, length: usize) -> Vec<u8>;
    fn write_bytes(&mut self, at: usize, bytes: &[u8]);
    fn read_string(&self, at: usize, length: usize) -> String;
    fn write_string(&mut self, at: usize, value: &str);
}

#[derive(Debug)]
pub struct _Memory(Vec<u8>);

impl _Memory {
    pub fn new(size: usize) -> Self {
        Self(vec![0u8; size])
    }

    pub fn display(&self, at: usize, length: usize) -> String {
        self.assert_access_range(at, length);

        let mut buf = String::new();

        if at > 0 {
            buf += &format!("\n... (- {} bytes)\n", at);
        }

        for i in 0..length {
            buf += &format!("{:02x} ", self.0[at + i]);
            if i % 16 == 15 {
                buf += "| ";
                for j in (0..16).rev() {
                    buf += &format!("{}", if self.0[at + i - j] < 32 || self.0[at + i - j] > 126 { '.' } else { self.0[at + i - j] as char });
                }
                if i < length - 1 {
                    buf += "\n";
                }
            }
        }

        if length < self.0.len() - at {
            buf += &format!("\n... (+ {} more bytes)", self.0.len() - at - length);
        }

        buf
    }
}

impl MemorySliceTrait for _Memory {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn assert_access_range(&self, at: usize, length: usize) {
        if at + length > self.0.len() {
            panic!("Memory size is 0x{:04X}({}), but trying to access at 0x{at:04X}({at}) with 0x{length:04X}({length}) bytes", self.0.len(), self.0.len());
        }
    }

    fn read_u8(&self, at: usize) -> u8 {
        self.assert_access_range(at, 1);
        self.0[at]
    }

    fn read_i8(&self, at: usize) -> i8 {
        self.assert_access_range(at, 1);
        self.0[at] as i8
    }

    fn write_u8(&mut self, at: usize, value: u8) {
        self.assert_access_range(at, 1);
        self.0[at] = value;
    }

    fn write_i8(&mut self, at: usize, value: i8) {
        self.assert_access_range(at, 1);
        self.0[at] = value as u8;
    }

    fn read_u16(&self, at: usize) -> u16 {
        self.assert_access_range(at, 2);
        ((self.0[at] as u16) << 8) | (self.0[at + 1] as u16)
    }

    fn read_i16(&self, at: usize) -> i16 {
        self.assert_access_range(at, 2);
        self.read_u16(at) as i16
    }

    fn write_u16(&mut self, at: usize, value: u16) {
        self.assert_access_range(at, 2);
        self.0[at] = (value >> 8) as u8;
        self.0[at + 1] = value as u8;
    }

    fn write_i16(&mut self, at: usize, value: i16) {
        self.assert_access_range(at, 2);
        self.write_u16(at, value as u16);
    }

    fn read_u32(&self, at: usize) -> u32 {
        self.assert_access_range(at, 4);
        ((self.0[at] as u32) << 24) | ((self.0[at + 1] as u32) << 16) | ((self.0[at + 2] as u32) << 8) | (self.0[at + 3] as u32)
    }

    fn read_i32(&self, at: usize) -> i32 {
        self.assert_access_range(at, 4);
        self.read_u32(at) as i32
    }

    fn write_u32(&mut self, at: usize, value: u32) {
        self.assert_access_range(at, 4);
        self.0[at] = (value >> 24) as u8;
        self.0[at + 1] = (value >> 16) as u8;
        self.0[at + 2] = (value >> 8) as u8;
        self.0[at + 3] = value as u8;
    }

    fn write_i32(&mut self, at: usize, value: i32) {
        self.assert_access_range(at, 4);
        self.write_u32(at, value as u32);
    }

    fn read_u64(&self, at: usize) -> u64 {
        self.assert_access_range(at, 8);
        self.read_u32(at) as u64 | (self.read_u32(at + 4) as u64) << 32
    }

    fn read_i64(&self, at: usize) -> i64 {
        self.assert_access_range(at, 8);
        self.read_u64(at) as i64
    }

    fn write_u64(&mut self, at: usize, value: u64) {
        self.assert_access_range(at, 8);
        self.write_u32(at, value as u32);
        self.write_u32(at + 4, (value >> 32) as u32);
    }

    fn write_i64(&mut self, at: usize, value: i64) {
        self.assert_access_range(at, 8);
        self.write_u64(at, value as u64);
    }

    fn read_u128(&self, at: usize) -> u128 {
        self.assert_access_range(at, 16);
        self.read_u64(at) as u128 | (self.read_u64(at + 8) as u128) << 64
    }

    fn read_i128(&self, at: usize) -> i128 {
        self.assert_access_range(at, 16);
        self.read_u128(at) as i128
    }

     fn write_u128(&mut self, at: usize, value: u128) {
        self.assert_access_range(at, 16);
        self.write_u64(at, value as u64);
        self.write_u64(at + 8, (value >> 64) as u64);
    }

    fn write_i128(&mut self, at: usize, value: i128) {
        self.assert_access_range(at, 16);
        self.write_u128(at, value as u128);
    }

    fn read_f32(&self, at: usize) -> f32 {
        self.assert_access_range(at, 4);
        f32::from_le_bytes(self.0[at..at + 4].try_into().unwrap())
    }

    fn write_f32(&mut self, at: usize, value: f32) {
        self.assert_access_range(at, 4);
        self.0[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn read_f64(&self, at: usize) -> f64 {
        self.assert_access_range(at, 8);
        f64::from_le_bytes(self.0[at..at + 8].try_into().unwrap())
    }

    fn write_f64(&mut self, at: usize, value: f64) {
        self.assert_access_range(at, 8);
        self.0[at..at + 8].copy_from_slice(&value.to_le_bytes());
    }

    fn read_bytes(&self, at: usize, length: usize) -> Vec<u8> {
        self.assert_access_range(at, length);
        self.0[at..at + length].to_vec()
    }

    fn write_bytes(&mut self, at: usize, value: &[u8]) {
        self.assert_access_range(at, value.len());
        self.0[at..at + value.len()].copy_from_slice(value);
    }

    fn read_string(&self, at: usize, length: usize) -> String {
        self.assert_access_range(at, length);
        String::from_utf8_lossy(&self.0[at..at + length]).into_owned()
    }

    fn write_string(&mut self, at: usize, value: &str) {
        self.assert_access_range(at, value.len());
        self.0[at..at + value.len()].copy_from_slice(value.as_bytes());
    }
}

impl Display for _Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display(0, self.len().min(128)))
    }
}

pub type Memory = Arc<Mutex<_Memory>>;

pub fn create_memory(size: usize) -> Memory {
    Arc::new(Mutex::new(_Memory::new(size)))
}

#[macro_export]
macro_rules! access_memory {
    ( $memory:expr ) => {
        $memory.lock().unwrap()
    };
}

#[macro_export]
macro_rules! share_memory {
    ( $memory:expr ) => {
        std::sync::Arc::clone(&$memory)
    };
}