use crate::system::address_from_high_low;
use crate::system::cpu::CPU;
use crate::system::cpu::flags::CPUFlags;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CPURegister
{
    A, B, C, D, E, H, L,
    PSW, BC, DE, HL,
    StackPointer,
    ProgramCounter,
    RAM
}

impl CPU
{
    pub fn read_register(&self, register : CPURegister) -> u16
    {
        return match register
        {
            CPURegister::A => self.A as u16,
            CPURegister::B => self.B as u16,
            CPURegister::C => self.C as u16,
            CPURegister::D => self.D as u16,
            CPURegister::E => self.E as u16,
            CPURegister::H => self.H as u16,
            CPURegister::L => self.L as u16,
            CPURegister::PSW => address_from_high_low(self.A, self.flags.to_byte()),
            CPURegister::BC => address_from_high_low(self.B, self.C),
            CPURegister::DE => address_from_high_low(self.D, self.E),
            CPURegister::HL => address_from_high_low(self.H, self.L),
            CPURegister::StackPointer => self.stack.get_pointer(),
            CPURegister::ProgramCounter => self.program_counter,
            CPURegister::RAM =>
            {
                let address = address_from_high_low(self.H, self.L);
                return self.ram[address] as u16;
            }
        }
    }

    pub fn write_register(&mut self, register : CPURegister, value : u16)
    {
        match register
        {
            CPURegister::A => { self.A = value as u8; }
            CPURegister::B => { self.B = value as u8; }
            CPURegister::C => { self.C = value as u8; }
            CPURegister::D => { self.D = value as u8; }
            CPURegister::E => { self.E = value as u8; }
            CPURegister::H => { self.H = value as u8; }
            CPURegister::L => { self.L = value as u8; }
            CPURegister::PSW => { self.A = (value >> 8) as u8; self.flags = CPUFlags::from_byte(value as u8); }
            CPURegister::BC => { self.B = (value >> 8) as u8; self.C = (value & 0xFF) as u8; }
            CPURegister::DE => { self.D = (value >> 8) as u8; self.E = (value & 0xFF) as u8; }
            CPURegister::HL => { self.H = (value >> 8) as u8; self.L = (value & 0xFF) as u8; }
            CPURegister::StackPointer => { self.stack.set_pointer(value); }
            CPURegister::ProgramCounter => { self.program_counter = value; }
            CPURegister::RAM =>
            {
                let address = address_from_high_low(self.H, self.L);
                self.ram[address] = value as u8;
            }
        }
    }
}
