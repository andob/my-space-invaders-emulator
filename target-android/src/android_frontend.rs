use std::{mem, slice};
use std::collections::HashMap;
use android_activity::{AndroidApp, InputStatus};
use android_activity::input::{InputEvent, KeyAction, KeyEvent, Keycode};
use anyhow::{anyhow, Context, Result};
use maplit2::hashmap;
use ndk::hardware_buffer_format::HardwareBufferFormat;
use ndk::native_window::{NativeWindow, NativeWindowBufferLockGuard};
use emulator::codeloc;
use emulator::system::frontend::{Event, Frontend, IEventFetcher, Key, WINDOW_HEIGHT, WINDOW_WIDTH};
use emulator::system::frontend::unsafe_frontend::IUnsafeCanvas;

const COLOR_BLACK : u32 = 0xFF000000;

pub struct AndroidFrontend {}
impl AndroidFrontend
{
    pub fn new(app : AndroidApp) -> Result<Frontend>
    {
        while app.native_window().is_none()
        {
            app.poll_events(None, |_| {});
        }

        let window_size = (WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        let window = app.native_window().ok_or(anyhow!(codeloc!()))?;
        let buffer_format = Some(HardwareBufferFormat::R8G8B8A8_UNORM);
        window.set_buffers_geometry(window_size.0, window_size.1, buffer_format).context(codeloc!())?;
        let canvas = Canvas::new(window).context(codeloc!())?;

        let event_fetcher = Box::new(EventFetcher::new(app));
        return Ok(Frontend::new(Box::new(canvas), event_fetcher));
    }
}

struct Canvas
{
    window : NativeWindow,
    current_color : u32,
    rects_to_draw : Vec<PendingRect>,
}

struct PendingRect
{
    color : u32, x : usize, y : usize,
    width : usize, height : usize
}

impl Canvas
{
    pub fn new(window : NativeWindow) -> Result<Canvas>
    {
        return Ok(Canvas { window, current_color:0, rects_to_draw:Vec::new() })
    }

    unsafe fn with_pixels<F>(&mut self, mut callback : F) -> Result<()>
    where F : FnMut(&mut[u32], &NativeWindowBufferLockGuard) -> Result<()>
    {
        let mut buffer = self.window.lock(None).context(codeloc!())?;

        let pixels_pointer = buffer.bits() as *mut u32;
        let pixels_count = buffer.stride() * buffer.height();
        let pixels = slice::from_raw_parts_mut(pixels_pointer, pixels_count);

        return callback(pixels, &buffer).context(codeloc!());
    }
}

impl IUnsafeCanvas for Canvas
{
    unsafe fn clear(&mut self)
    {
        self.rects_to_draw.push(PendingRect
        {
            color: COLOR_BLACK, x:0, y:0,
            width: WINDOW_WIDTH, height: WINDOW_HEIGHT,
        });
    }

    unsafe fn set_draw_color(&mut self, r : u8, g : u8, b : u8)
    {
        self.current_color = (0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    }

    unsafe fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32)
    {
        self.rects_to_draw.push(PendingRect
        {
            color: self.current_color, x: x as usize, y: y as usize,
            width: width as usize, height: height as usize,
        });
    }

    unsafe fn present(&mut self) -> Result<()>
    {
        let rects_to_draw = mem::take(&mut self.rects_to_draw);

        return self.with_pixels(move |pixels, buffer|
        {
            for rect in &rects_to_draw
            {
                let stride = buffer.stride();
                for rect_iy in 0..rect.height
                {
                    let base_address = (rect.y + rect_iy) * stride + rect.x;
                    for rect_ix in 0..rect.width
                    {
                        pixels[base_address + rect_ix] = rect.color;
                    }
                }
            }

            return Ok(());
        }).context(codeloc!());
    }
}

struct EventFetcher
{
    app : AndroidApp,
    keymap : HashMap<u32, Key>,
}

impl EventFetcher
{
    pub fn new(app : AndroidApp) -> EventFetcher
    {
        let keymap = hashmap!
        {
            Keycode::C.into() => Key::INSERT_COIN,
            Keycode::Keycode1.into() => Key::SELECT_ONE_PLAYER,
            Keycode::Keycode2.into() => Key::SELECT_TWO_PLAYERS,
            Keycode::DpadLeft.into() => Key::PLAYER1_LEFT,
            Keycode::DpadRight.into() => Key::PLAYER1_RIGHT,
            Keycode::Space.into() => Key::PLAYER1_SHOOT,
            Keycode::A.into() => Key::PLAYER2_LEFT,
            Keycode::D.into() => Key::PLAYER2_RIGHT,
            Keycode::S.into() => Key::PLAYER2_SHOOT,
        };

        return EventFetcher { app, keymap };
    }
}

impl IEventFetcher for EventFetcher
{
    fn fetch_events(&mut self) -> Vec<Event>
    {
        let mut output_events = Vec::new();

        if let Ok(mut input_event_iterator) = self.app.input_events_iter()
        {
            input_event_iterator.next(|event|
            {
                if let InputEvent::KeyEvent(event) = event
                {
                    if event.action() == KeyAction::Down
                    {
                        if let Some(key) = self.keymap.get(&event.key_code().into())
                        {
                            output_events.push(Event::KeyDown(*key));
                        }
                    }
                    else if event.action() == KeyAction::Up
                    {
                        if let Some(key) = self.keymap.get(&event.key_code().into())
                        {
                            output_events.push(Event::KeyUp(*key));
                        }
                    }
                }

                return InputStatus::Handled;
            });
        }

        return output_events;
    }
}
