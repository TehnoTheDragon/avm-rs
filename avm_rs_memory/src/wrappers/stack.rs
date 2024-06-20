use crate::{mem::{Memory, MemorySliceTrait}, pointer::Pointer};

macro_rules! impl_stack_push {
    ( $type:ident, $name:ident, $write:ident, $width:literal ) => {
        pub fn $name(&mut self, value: $type) {
            self.pointer.$write(self.top, value);
            self.top += $width;
        }
    };
}

macro_rules! impl_stack_pop {
    ( $type:ident, $name:ident, $read:ident, $width:literal ) => {
        pub fn $name(&mut self) -> $type {
            self.top -= $width;
            self.pointer.$read(self.top)
        }
    };
}

#[derive(Debug)]
pub struct Stack {
    pub pointer: Pointer,
    top: usize,
}

impl Stack {
    pub fn new(pointer: Pointer) -> Self {
        Self {
            pointer: pointer,
            top: 0,
        }
    }

    pub fn reset(&mut self) {
        self.top = 0;
    }

    pub fn set_top(&mut self, top: usize) {
        if self.top > self.pointer.size {
            panic!("Stack overflow");
        }
        self.top = top;
    }

    impl_stack_push!(u8, push_u8, write_u8, 1);
    impl_stack_pop!(u8, pop_u8, read_u8, 1);

    impl_stack_push!(u16, push_u16, write_u16, 2);
    impl_stack_pop!(u16, pop_u16, read_u16, 2);

    impl_stack_push!(u32, push_u32, write_u32, 4);
    impl_stack_pop!(u32, pop_u32, read_u32, 4);

    impl_stack_push!(u64, push_u64, write_u64, 8);
    impl_stack_pop!(u64, pop_u64, read_u64, 8);

    impl_stack_push!(u128, push_u128, write_u128, 16);
    impl_stack_pop!(u128, pop_u128, read_u128, 16);

    impl_stack_push!(i128, push_i128, write_i128, 16);
    impl_stack_pop!(i128, pop_i128, read_i128, 16);

    impl_stack_push!(f32, push_f32, write_f32, 4);
    impl_stack_pop!(f32, pop_f32, read_f32, 4);

    impl_stack_push!(f64, push_f64, write_f64, 8);
    impl_stack_pop!(f64, pop_f64, read_f64, 8);

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.pointer.write_bytes(self.top, bytes);
        self.top += bytes.len();
    }

    pub fn pop_bytes(&mut self, length: usize) -> Vec<u8> {
        self.top -= length;
        self.pointer.read_bytes(self.top, length).to_vec()
    }

    pub fn push_string(&mut self, string: &str) {
        self.pointer.write_string(self.top, string);
        self.top += string.len();
    }

    pub fn pop_string(&mut self, length: usize) -> String {
        self.top -= length;
        self.pointer.read_string(self.top, length)
    }
}