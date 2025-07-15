#![no_std]

#[macro_use]
extern crate alloc;

pub mod system;

#[inline(always)]
pub fn address_from_high_low(high : u8, low : u8) -> u16
{
    return ((high as u16) << 8) | (low as u16);
}

#[macro_export]
macro_rules! codeloc
{
    () => { format!("{}:{}", file!(), line!()) }
}
