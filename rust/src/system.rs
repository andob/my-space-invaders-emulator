use crate::system::cpu::CPU;

mod cpu;

#[inline(always)]
pub fn address_from_high_low(high : u8, low : u8) -> u16
{
    return ((high as u16) << 8) | (low as u16);
}

pub struct System
{
    cpu : CPU
}

impl System
{
    pub fn new() -> System
    {
        return System { cpu: CPU::new() };
    }
}
