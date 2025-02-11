use anyhow::Context;
use crate::address_from_high_low;
use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::interrupts::CPUInterrupts;
use crate::system::cpu::opcodes::{build_opcodes_slice, Opcode};
use crate::system::cpu::ram::{RAM, RAM_SIZE};
use crate::system::cpu::stack::CPUStack;

mod flags;
mod registers;
mod opcodes;
pub mod ram;
mod stack;
mod interrupts;

#[allow(non_snake_case)]
pub struct CPU
{
    A : u8,
    B : u8,
    C : u8,
    D : u8,
    E : u8,
    H : u8,
    L : u8,
    program_counter : u16,
    stack : CPUStack,
    flags : CPUFlags,
    pub in1 : u8,
    pub in2 : u8,
    shift_register : u16,
    shift_register_offset : u8,
    are_interrupts_enabled : bool,
    pub ram : RAM,
}

pub struct CPURunEnvironment
{
    opcodes : Box<[Opcode]>
}

impl CPU
{
    pub fn new(rom_bytes : &[u8]) -> (CPU, CPURunEnvironment)
    {
        let cpu = CPU
        {
            A:0, B:0, C:0, D:0, E:0, H:0, L:0,
            program_counter: 0,
            stack: CPUStack::new(),
            flags: CPUFlags::from_byte(0),
            in1: 0, in2: 0,
            shift_register: 0,
            shift_register_offset: 0,
            are_interrupts_enabled: true,
            ram: RAM::new(rom_bytes),
        };

        let opcodes = build_opcodes_slice();
        return (cpu, CPURunEnvironment { opcodes });
    }

    pub fn next_byte(&mut self) -> u8
    {
        if (self.program_counter as usize) < RAM_SIZE
        {
            let value = self.ram[self.program_counter];
            self.program_counter += 1;
            return value;
        }

        return 0;
    }

    pub fn next_address(&mut self) -> u16
    {
        let low = self.next_byte();
        let high = self.next_byte();
        return address_from_high_low(high, low);
    }

    pub fn run_until_next_frame(&mut self, env : &CPURunEnvironment)
    {
        for interrupt_number in [1u16, 2u16]
        {
            let mut cycle_count = 0u16;
            while cycle_count <= 16600
            {
                let opcode = &env.opcodes[self.next_byte() as usize];
                // println!("{}", opcode.name);
                (opcode.lambda)(self);

                cycle_count += opcode.duration;
            }

            CPUInterrupts::interrupt(self, interrupt_number);
        }
    }
}
