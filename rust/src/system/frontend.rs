mod sdl_frontend;
mod web_frontend;

use anyhow::{anyhow, Result};
use crate::system::cpu::CPU;
use crate::system::cpu::ram::RAM;

const DISPLAY_WIDTH : usize = 224;
const DISPLAY_HEIGHT : usize = 256;
const DISPLAY_START_ADDRESS : u16 = 0x2400;
const DISPLAY_END_ADDRESS : u16 = 0x4000;
const BLOCK_WIDTH : usize = 3;
const BLOCK_HEIGHT : usize = 3;

pub struct Frontend
{
    canvas : Box<dyn ICanvas>,
    event_fetcher : Box<dyn IEventFetcher>,
}

trait ICanvas
{
    fn clear(&mut self);
    fn set_draw_color(&mut self, r : u8, g : u8, b : u8);
    fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32);
    fn present(&mut self);
}

trait IEventFetcher
{
    fn fetch_events(&mut self) -> Vec<Event>;
}

enum Event
{
    Quit, KeyDown(Keycode), KeyUp(Keycode)
}

enum Keycode
{
    C, A, S, D, Left, Right, Space, Kp1, Kp2, NUM_1, NUM_2
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
                    self.canvas.fill_rect(x, y, BLOCK_WIDTH as u32, BLOCK_HEIGHT as u32);
                }
            }
        }

        self.canvas.present();

        return Ok(());
    }

    pub fn handle_events(&mut self, cpu : &mut CPU) -> Result<()>
    {
        for event in self.event_fetcher.fetch_events()
        {
            match event
            {
                Event::Quit { .. } => { return Err(anyhow!("User quit!")); }
                Event::KeyDown(keycode) =>
                {
                    if let Some((player, key)) = self.parse_keycode(keycode)
                    {
                        if player == 1 { cpu.in1 = cpu.in1 | 1u8 << key; }
                        else if player == 2 { cpu.in2 = cpu.in2 | 1u8 << key; }
                    }
                }
                Event::KeyUp(keycode) =>
                {
                    if let Some((player, key)) = self.parse_keycode(keycode)
                    {
                        if player == 1 { cpu.in1 = cpu.in1 & !(1u8 << key); }
                        else if player == 2 { cpu.in2 = cpu.in2 & !(1u8 << key); }
                    }
                }
            }
        }

        return Ok(());
    }

    fn parse_keycode(&self, keycode : Keycode) -> Option<(u8, u8)>
    {
        let (player, key) = match keycode
        {
            Keycode::C => (1, 0), //insert coin
            Keycode::Kp1 | Keycode::NUM_1 => (1, 2), //1 player
            Keycode::Kp2 | Keycode::NUM_2 => (1, 2), //2 players
            Keycode::Space => (1, 4), //player 1 shoot
            Keycode::Left => (1, 5), //player 1 left
            Keycode::Right => (1, 6), //player 1 right
            Keycode::S => (2, 4), //player 2 shoot
            Keycode::A => (2, 5), //player 2 left
            Keycode::D => (2, 6), //player 2 right
            _ => return None
        };

        return Some((player, key));
    }
}
