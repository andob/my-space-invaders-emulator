use alloc::string::String;
use anyhow::{Context, Result};
use crate::codeloc;
use crate::system::cpu::{CPURunEnvironment, CPU};
use crate::system::frontend::{Event, Frontend};

mod cpu;
pub mod frontend;

pub struct System
{
    cpu : (CPU, CPURunEnvironment),
    frontend : Frontend,
}

impl System
{
    pub fn new(rom_bytes : &[u8], frontend : Frontend) -> System
    {
        return System::new_verbose(rom_bytes, frontend, |_| {});
    }

    pub fn new_verbose(rom_bytes : &[u8], frontend : Frontend, logger : fn(&String) -> ()) -> System
    {
        return System { cpu: CPU::new(rom_bytes, logger), frontend };
    }

    pub fn render_next_frame(&mut self) -> Result<()>
    {
        let cpu = &mut self.cpu.0;
        let cpu_run_environment = &self.cpu.1;

        CPU::run_until_next_frame(cpu, cpu_run_environment);

        self.frontend.render_frame(&cpu.ram).context(codeloc!())?;

        self.frontend.handle_events(cpu);

        return Ok(());
    }

    pub fn notify(&mut self, event : Event)
    {
        self.frontend.notify(event);
    }
}
