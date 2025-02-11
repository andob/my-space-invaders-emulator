use anyhow::Result;
use crate::system::cpu::CPU;
use crate::system::frontend::Frontend;

mod cpu;
mod frontend;

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

pub struct System
{
    cpu : CPU,
    frontend : Frontend,
}

impl System
{
    pub fn new(rom_bytes : &[u8]) -> System
    {
        return System
        {
            cpu: CPU::new(rom_bytes),
            frontend: Frontend::new().unwrap(),
        };
    }

    pub fn render_frame(&mut self) -> Result<()>
    {
        return self.cpu.run(|cpu|
        {
            self.frontend.render_frame(&cpu.ram)?;
            return self.frontend.handle_events(cpu);
        })
    }
}
