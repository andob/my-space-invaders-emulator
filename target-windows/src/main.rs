use std::thread;
use std::time::Duration;
use anyhow::{Context, Result};
use emulator::codeloc;
use emulator::system::frontend::dummy_frontend::DummyFrontend;
use emulator::system::System;

const ROM_BYTES : &[u8] = include_bytes!("../../game.rom");

fn main() -> Result<()>
{
    //todo implement windows frontend
    let frontend = DummyFrontend::new();
    let mut system = System::new(ROM_BYTES, frontend);

    loop
    {
        system.render_next_frame().context(codeloc!())?;
        thread::sleep(Duration::from_millis(10));
    }
}
