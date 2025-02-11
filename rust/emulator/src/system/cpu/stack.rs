use crate::address_from_high_low;
use crate::system::cpu::CPU;

pub struct CPUStack
{
    pointer : u16,
    max_address : u16,
    min_address : u16,
}

impl CPUStack
{
    pub fn new() -> CPUStack
    {
        return CPUStack
        {
            pointer: 0xFFFF,
            max_address: 0xFFFF,
            min_address: 0x0000,
        }
    }

    pub fn get_pointer(&self) -> u16
    {
        return self.pointer;
    }

    pub fn set_pointer(&mut self, pointer : u16)
    {
        self.pointer = pointer;
        self.max_address = pointer;
        self.min_address = pointer - 0xFF;
    }

    pub fn push_byte(cpu : &mut CPU, value : u8)
    {
        cpu.stack.pointer -= 1;

        //stack overflow!
        if cpu.stack.pointer < cpu.stack.min_address
        {
            cpu.stack.pointer = cpu.stack.max_address;
        }

        cpu.ram[cpu.stack.pointer] = value;
    }

    pub fn pop_byte(cpu : &mut CPU) -> u8
    {
        let value = cpu.ram[cpu.stack.pointer];

        cpu.stack.pointer += 1;

        //stack underflow!
        if cpu.stack.pointer > cpu.stack.max_address
        {
            cpu.stack.pointer = cpu.stack.min_address;
        }

        return value;
    }

    pub fn push_address(cpu : &mut CPU, address : u16)
    {
        CPUStack::push_byte(cpu, (address >> 8) as u8);
        CPUStack::push_byte(cpu, (address & 0xFF) as u8);
    }

    pub fn pop_address(cpu : &mut CPU) -> u16
    {
        let low = CPUStack::pop_byte(cpu);
        let high = CPUStack::pop_byte(cpu);
        return address_from_high_low(high, low);
    }
}
