use std::collections::HashMap;
use anyhow::{anyhow, Context, Result};
use maplit2::hashmap;
use sdl2::event::Event as SDLEvent;
use sdl2::keyboard::Keycode as SDLKeycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use emulator::codeloc;
use emulator::system::frontend::{Event, Frontend, ICanvas, IEventFetcher, Key, BLOCK_HEIGHT, BLOCK_WIDTH, DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct SDLFrontend {}
impl SDLFrontend
{
    pub fn new() -> Result<Frontend>
    {
        let sdl = sdl2::init().map_err(|msg|anyhow!(msg)).context(codeloc!())?;
        let video_subsystem = sdl.video().map_err(|msg|anyhow!(msg)).context(codeloc!())?;

        let window_title = "Space Invaders";
        let window_width = (DISPLAY_WIDTH * BLOCK_WIDTH) as u32;
        let window_height = (DISPLAY_HEIGHT * BLOCK_HEIGHT) as u32;
        let window = video_subsystem.window(window_title, window_width, window_height).build().context(codeloc!())?;

        let opengl_driver_index = sdl2::render::drivers().position(|d| d.name=="opengl").unwrap();
        let canvas = window.into_canvas().index(opengl_driver_index as u32).accelerated().build().context(codeloc!())?;

        return Ok(Frontend
        {
            event_fetcher: Box::new(EventFetcher::new(sdl)),
            canvas: Box::new(CanvasWrapper { canvas }),
        })
    }
}

struct CanvasWrapper { canvas : WindowCanvas }

impl ICanvas for CanvasWrapper
{
    fn clear(&mut self)
    {
        self.canvas.clear();
    }

    fn set_draw_color(&mut self, r : u8, g : u8, b : u8)
    {
        self.canvas.set_draw_color(Color::from((r, g, b)));
    }

    fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32)
    {
        if let Err(error) = self.canvas.fill_rect(Rect::new(x, y, width, height))
        {
            println!("{}", error);
        }
    }

    fn present(&mut self)
    {
        self.canvas.present();
    }
}

struct EventFetcher
{
    sdl : Sdl,
    keymap : HashMap<SDLKeycode, Key>,
}

impl EventFetcher
{
    fn new(sdl : Sdl) -> EventFetcher
    {
        let keymap = hashmap!
        {
            SDLKeycode::C => Key::INSERT_COIN,
            SDLKeycode::NUM_1 => Key::SELECT_ONE_PLAYER,
            SDLKeycode::NUM_2 => Key::SELECT_TWO_PLAYERS,
            SDLKeycode::Left => Key::PLAYER1_LEFT,
            SDLKeycode::Right => Key::PLAYER1_RIGHT,
            SDLKeycode::Space => Key::PLAYER1_SHOOT,
            SDLKeycode::A => Key::PLAYER2_LEFT,
            SDLKeycode::D => Key::PLAYER2_RIGHT,
            SDLKeycode::S => Key::PLAYER2_SHOOT,
        };

        return EventFetcher { sdl, keymap };
    }
}

impl IEventFetcher for EventFetcher
{
    fn notify(&mut self, _event : Event) {}

    fn fetch_events(&mut self) -> Vec<Event>
    {
        let mut events = Vec::<Event>::new();

        if let Ok(mut sdl_event_pump) = self.sdl.event_pump()
        {
            for sdl_event in sdl_event_pump.poll_iter()
            {
                match sdl_event
                {
                    SDLEvent::Quit { .. } =>
                    {
                        events.push(Event::Quit);
                    },

                    SDLEvent::KeyDown { keycode: Some(keycode), .. } =>
                    {
                        if let Some(key) = self.keymap.get(&keycode)
                        {
                            events.push(Event::KeyDown(*key));
                        }
                    }

                    SDLEvent::KeyUp { keycode: Some(keycode), .. } =>
                    {
                        if let Some(key) = self.keymap.get(&keycode)
                        {
                            events.push(Event::KeyUp(*key));
                        }
                    }
                    _ => {}
                }
            }
        }

        return events;
    }
}
