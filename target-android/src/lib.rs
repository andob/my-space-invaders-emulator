mod android_frontend;

use anyhow::Result;
use std::{panic, process, thread};
use std::time::Duration;
use android_activity::AndroidApp;
use anyhow::Context;
use android_logger::Config as AndroidLoggerConfig;
use emulator::system::System;
use emulator::codeloc;
use crate::android_frontend::AndroidFrontend;

const ROM_BYTES : &[u8] = include_bytes!("../../game.rom");

#[no_mangle]
fn android_main(app : AndroidApp)
{
    android_logger::init_once(AndroidLoggerConfig::default()
        .with_max_level(log::LevelFilter::Info));

    panic::set_hook(Box::new(|info|
    {
        log::error!("PANIC: {}", info);
        process::abort();
    }));

    if let Err(error) = android_main_impl(app)
    {
        log::error!("ERROR: {}", error);
        panic!("{}", error);
    }
}

fn android_main_impl(app : AndroidApp) -> Result<()>
{
    let frontend = AndroidFrontend::new(app).context(codeloc!())?;
    let mut system = System::new(ROM_BYTES, frontend);

    loop
    {
        system.render_next_frame().context(codeloc!())?;
        thread::sleep(Duration::from_millis(1));
    }
}
