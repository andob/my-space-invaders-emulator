pub mod dummy_frontend;
pub mod unsafe_frontend;

use alloc::boxed::Box;
use alloc::vec::Vec;
use anyhow::{Context, Result};
use crate::codeloc;
use crate::system::cpu::CPU;
use crate::system::cpu::ram::RAM;

pub const DISPLAY_WIDTH : usize = 224;
pub const DISPLAY_HEIGHT : usize = 256;
pub const DISPLAY_START_ADDRESS : u16 = 0x2400;
pub const DISPLAY_END_ADDRESS : u16 = 0x4000;
pub const BLOCK_WIDTH : usize = 3;
pub const BLOCK_HEIGHT : usize = 3;
pub const WINDOW_WIDTH : usize = DISPLAY_WIDTH * BLOCK_WIDTH;
pub const WINDOW_HEIGHT : usize = DISPLAY_HEIGHT * BLOCK_HEIGHT;
pub const WINDOW_TITLE : &str = "Space Invaders";

pub struct Frontend
{
    pub canvas : Box<dyn ICanvas>,
    external_event_buffer : Vec<Event>,
    pub event_fetcher : Box<dyn IEventFetcher>,
}

impl Frontend
{
    pub fn new(canvas : Box<dyn ICanvas>, event_fetcher : Box<dyn IEventFetcher>) -> Frontend
    {
        return Frontend { canvas, external_event_buffer: Vec::new(), event_fetcher };
    }
}

pub trait ICanvas
{
    fn clear(&mut self);
    fn set_draw_color(&mut self, r : u8, g : u8, b : u8);
    fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32);
    fn present(&mut self) -> Result<()>;
}

pub trait IEventFetcher
{
    fn fetch_events(&mut self) -> Vec<Event>;
}

#[derive(Copy, Clone)]
pub enum Event { KeyDown(Key), KeyUp(Key) }

#[derive(Copy, Clone)]
pub struct Key { player : u8, code : u8 }

impl Key
{
    pub const INSERT_COIN : Key = Key { player:1, code:0 };
    pub const SELECT_ONE_PLAYER : Key = Key { player:1, code:2 };
    pub const SELECT_TWO_PLAYERS : Key = Key { player:1, code:1 };
    pub const PLAYER1_SHOOT : Key = Key { player:1, code:4 };
    pub const PLAYER1_LEFT : Key = Key { player:1, code:5 };
    pub const PLAYER1_RIGHT : Key = Key { player:1, code:6 };
    pub const PLAYER2_SHOOT : Key = Key { player:2, code:4 };
    pub const PLAYER2_LEFT : Key = Key { player:2, code:5 };
    pub const PLAYER2_RIGHT : Key = Key { player:2, code:6 };
}

impl Frontend
{
    pub fn render_frame(&mut self, ram : &RAM) -> Result<()>
    {
        self.canvas.set_draw_color(0, 0, 0);
        self.canvas.clear();

        let mut display_address = DISPLAY_START_ADDRESS;
        for ix in 0..DISPLAY_WIDTH
        {
            for iy in (0..DISPLAY_HEIGHT).step_by(8)
            {
                let mut byte = ram[display_address];
                display_address += 1;
                for bit_index in 0..8
                {
                    let bit = (byte & 1) == 1;
                    byte = byte >> 1;
                    if !bit { continue; }

                    let (mut r, mut g, mut b) = (0x00, 0x00, 0x00);
                    if iy > 200 && iy < 220 { r = 0xFF; }
                    else if iy < 80 { g = 0xFF; }
                    else { (r, g, b) = (0xFF, 0xFF, 0xFF); }

                    self.canvas.set_draw_color(r, g, b);

                    let x = (ix * BLOCK_WIDTH) as i32;
                    let y = ((DISPLAY_HEIGHT - iy - bit_index) * BLOCK_HEIGHT) as i32;
                    let (width, height) = (BLOCK_WIDTH as u32, BLOCK_HEIGHT as u32);
                    self.canvas.fill_rect(x, y, width, height);
                }
            }
        }

        self.canvas.present().context(codeloc!())?;

        return Ok(());
    }

    pub fn handle_events(&mut self, cpu : &mut CPU)
    {
        let events =
        [
            self.external_event_buffer.clone(),
            self.event_fetcher.fetch_events(),
        ].concat();

        self.external_event_buffer.clear();
        
        for event in events
        {
            match event
            {
                Event::KeyDown(key) =>
                {
                    if key.player == 1 { cpu.in1 = cpu.in1 | 1u8 << key.code; }
                    else if key.player == 2 { cpu.in2 = cpu.in2 | 1u8 << key.code; }
                }
                Event::KeyUp(key) =>
                {
                    if key.player == 1 { cpu.in1 = cpu.in1 & !(1u8 << key.code); }
                    else if key.player == 2 { cpu.in2 = cpu.in2 & !(1u8 << key.code); }
                }
            }
        }
    }

    pub fn notify(&mut self, event : Event)
    {
        self.external_event_buffer.push(event);
    }
}
