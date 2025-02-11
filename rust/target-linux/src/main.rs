mod sdl_frontend;

use anyhow::{Context, Result};
use emulator::codeloc;
use emulator::system::frontend::Frontend;
use emulator::system::System;
use crate::sdl_frontend::SDLFrontend;

const ROM_BYTES : &[u8] = include_bytes!("../../game.rom");

fn main() -> Result<()>
{
    let frontend = SDLFrontend::new().context(codeloc!())?;
    let mut system = System::new(ROM_BYTES, frontend);
    return system.run();
}
