use avm_rs_memory::{mem::Memory, wrappers::stack::Stack};

use crate::register::*;

pub trait CPUTrait {
    fn step(&mut self);
    fn reset(&mut self);
}