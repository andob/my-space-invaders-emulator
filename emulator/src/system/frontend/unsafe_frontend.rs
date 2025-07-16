use alloc::vec::Vec;
use anyhow::Result;
use crate::system::frontend::{Event, ICanvas, IEventFetcher};

pub trait IUnsafeCanvas
{
    unsafe fn clear(&mut self);
    unsafe fn set_draw_color(&mut self, r : u8, g : u8, b : u8);
    unsafe fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32);
    unsafe fn present(&mut self) -> anyhow::Result<()>;

}

impl <T : IUnsafeCanvas> ICanvas for T
{
    fn clear(&mut self)
    {
        unsafe { IUnsafeCanvas::clear(self) };
    }

    fn set_draw_color(&mut self, r : u8, g : u8, b : u8)
    {
        unsafe { IUnsafeCanvas::set_draw_color(self, r, g, b) };
    }

    fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32)
    {
        unsafe { IUnsafeCanvas::fill_rect(self, x, y, width, height) };
    }

    fn present(&mut self) -> Result<()>
    {
        return unsafe { IUnsafeCanvas::present(self) };
    }
}

pub trait IUnsafeEventFetcher
{
    unsafe fn fetch_events(&mut self) -> Vec<Event>;
}

impl <T : IUnsafeEventFetcher> IEventFetcher for T
{
    fn fetch_events(&mut self) -> Vec<Event>
    {
        return unsafe { IUnsafeEventFetcher::fetch_events(self) };
    }
}
