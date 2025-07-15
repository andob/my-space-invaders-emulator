mod linux_frontend;

use std::thread;
use std::time::Duration;
use anyhow::{Context, Result};
use emulator::codeloc;
use emulator::system::System;
use crate::linux_frontend::LinuxFrontend;

const ROM_BYTES : &[u8] = include_bytes!("../../game.rom");

fn main() -> Result<()>
{
    let frontend = LinuxFrontend::new().context(codeloc!())?;
    let mut system = System::new(ROM_BYTES, frontend);

    loop
    {
        system.render_next_frame().context(codeloc!())?;
        thread::sleep(Duration::from_micros(500));
    }
}
