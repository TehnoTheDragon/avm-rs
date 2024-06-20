use std::io::Read;

use avm_rs_component::{cpu::CPUTrait, register::{RegisterF64, RegisterTrait, RegisterU64}};
use avm_rs_memory::{access_memory, mem::{create_memory, Memory, MemorySliceTrait}, share_memory, vmem::VirtualMemory, wrappers::stack::Stack};

pub struct MiniCPU {
    memory: Memory,
    stack: Stack,
    ipoffset: usize,

    pub ru64: [RegisterU64; 7],
    pub fu64: [RegisterF64; 4],

    // sp - 5
    // ip - 6
    // fl - 7
}

const MINICPU_ARRAY_REGISTER_U64: RegisterU64 = RegisterU64(0);
const MINICPU_ARRAY_REGISTER_F64: RegisterF64 = RegisterF64(0.0);
impl MiniCPU {
    pub fn new(memory: Memory, stack: Stack) -> Self {
        Self {
            memory: memory,
            stack: stack,
            ipoffset: 0,
            ru64: [MINICPU_ARRAY_REGISTER_U64; 7],
            fu64: [MINICPU_ARRAY_REGISTER_F64; 4],
        }
    }

    pub fn start_at(&mut self, offset: usize) {
        self.ipoffset = offset;
    }

    fn fetch_instruction(&mut self) -> (u8, u8, u8, u8) {
        let instcode = access_memory!(self.memory).read_u32(self.ipoffset + self.ru64[6].0 as usize);
        self.ru64[5].0 += 4;

        let inst = ((instcode >> 24) & 0xFF) as u8;
        let ins2 = ((instcode >> 16) & 0xFF) as u8;
        let op2 = ((instcode >> 8) & 0xFF) as u8;
        let op3 = (instcode & 0xFF) as u8;
        (inst, ins2, op2, op3)
    }
}

impl CPUTrait for MiniCPU {
    fn reset(&mut self) {
        self.stack.reset();
        self.ru64[6].0 = 0;
    }

    fn step(&mut self) {
        let (inst, ins2, op2, op3) = self.fetch_instruction();

        match inst {
            // mov <X:register(u|f)>, <Y:register(u|f)> | mov <X:register(u|f)>, constant
            1 => {
                match ins2 {
                    // mov ru, ru
                    0 => {
                        self.ru64[op2 as usize].0 = self.ru64[op3 as usize].0;
                    }
                    // mov fu, fu
                    1 => {
                        self.fu64[op2 as usize].0 = self.fu64[op3 as usize].0;
                    }
                    // mov ru, constant
                    2 => {
                        self.ru64[0].0 = op2 as u64 * op3 as u64;
                    }
                    _ => panic!("Unknown instruction: {inst}:{ins2}", )
                }
            }
            _ => panic!("Unknown instruction: {}", inst),
        }
    }
}

fn main() {
    let mut memory = create_memory(1024 * 512);
    let mut virtual_memory = VirtualMemory::new(share_memory!(memory));
    let mut stack = Stack::new(virtual_memory.allocate(1024));
    let mut cpu = MiniCPU::new(memory.clone(), stack);

    access_memory!(memory).write_bytes(0, vec![1, 2, 3, 4].as_slice());

    cpu.reset();
    cpu.step();
    println!("{}", cpu.ru64[0].0);
    println!("{}", access_memory!(memory));
}