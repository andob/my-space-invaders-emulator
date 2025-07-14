use anyhow::Result;
use emulator::system::frontend::dummy_frontend::DummyFrontend;
use emulator::system::System;

const ROM_BYTES : &[u8] = include_bytes!("../../game.rom");

fn main() -> Result<()>
{
    let frontend = DummyFrontend::new();
    let mut system = System::new(ROM_BYTES, frontend);
    return system.run();
}
