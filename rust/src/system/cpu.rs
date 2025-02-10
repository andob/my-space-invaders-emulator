use crate::system::cpu::flags::CPUFlags;
use crate::system::address_from_high_low;
use crate::system::cpu::ram::{RAM, RAM_SIZE};
use crate::system::cpu::stack::CPUStack;

mod flags;
mod registers;
mod opcodes;
mod ram;
mod stack;

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
    in1 : u8,
    in2 : u8,
    shift_register : u16,
    shift_register_offset : u8,
    are_interrupts_enabled : bool,
    ram : RAM,
}

impl CPU
{
    pub fn new() -> CPU
    {
        return CPU
        {
            A:0, B:0, C:0, D:0, E:0, H:0, L:0,
            program_counter: 0,
            stack: CPUStack::new(),
            flags: CPUFlags::from_byte(0),
            in1: 0, in2: 0,
            shift_register: 0,
            shift_register_offset: 0,
            are_interrupts_enabled: true,
            ram: RAM::new(),
        }
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
}
