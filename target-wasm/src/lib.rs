mod web_frontend;

use std::cell::RefCell;
use std::rc::Rc;
use anyhow::Context;
use wasm_bindgen::prelude::wasm_bindgen;
use emulator::system::frontend::dummy_frontend::DummyFrontend;
use emulator::system::frontend::Event;
use emulator::system::System;
use crate::web_frontend::{parse_js_key, WebFrontend};

const ROM_BYTES : &[u8] = include_bytes!("../../game.rom");

thread_local!
{
    static SYSTEM : Rc<RefCell<System>> =
        Rc::new(RefCell::new(System::new(ROM_BYTES, DummyFrontend::new())));
}

#[wasm_bindgen]
pub fn initialize()
{
    console_error_panic_hook::set_once();

    SYSTEM.with(|system|
    {
        let frontend = WebFrontend::new().unwrap();
        *system.borrow_mut() = System::new(ROM_BYTES, frontend);
    });
}

#[wasm_bindgen]
pub fn render_next_frame()
{
    SYSTEM.with(|system|
    {
        system.borrow_mut().render_next_frame().unwrap();
    });
}

#[wasm_bindgen]
pub fn on_key_down(js_key : String)
{
    if let Some(key) = parse_js_key(js_key)
    {
        SYSTEM.with(|system|
        {
            system.borrow_mut().notify(Event::KeyDown(key));
        });
    }
}

#[wasm_bindgen]
pub fn on_key_up(js_key : String)
{
    if let Some(key) = parse_js_key(js_key)
    {
        SYSTEM.with(|system|
        {
            system.borrow_mut().notify(Event::KeyUp(key));
        });
    }
}
