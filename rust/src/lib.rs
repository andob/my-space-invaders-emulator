use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;
use crate::system::System;

mod system;

const ROM_BYTES : &[u8] = include_bytes!("game.rom");

thread_local!
{
    static SYSTEM : Rc<RefCell<System>> =
        Rc::new(RefCell::new(System::new(ROM_BYTES)));
}

#[wasm_bindgen]
pub fn setup_console_error_panic_hook()
{
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn render_next_frame()
{
    SYSTEM.with(|system|
    {
        if let Err(error) = system.borrow_mut().render_frame()
        {
            let message = format!("{}", error);
            window().unwrap().alert_with_message(message.as_str()).unwrap();
        }
    });
}
