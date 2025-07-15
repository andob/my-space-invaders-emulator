use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::system::frontend::{Event, Frontend, ICanvas, IEventFetcher};

pub struct DummyFrontend {}
struct DummyFrontendCanvas {}
struct DummyFrontendEventFetcher {}

impl DummyFrontend
{
    pub fn new() -> Frontend
    {
        return Frontend
        {
            canvas: Box::new(DummyFrontendCanvas{}),
            event_fetcher: Box::new(DummyFrontendEventFetcher{}),
        }
    }
}

impl ICanvas for DummyFrontendCanvas
{
    fn clear(&mut self) {}
    fn set_draw_color(&mut self, _r : u8, _g : u8, _b : u8) {}
    fn fill_rect(&mut self, _x : i32, _y : i32, _width : u32, _height : u32) {}
    fn present(&mut self) {}
}

impl IEventFetcher for DummyFrontendEventFetcher
{
    fn notify(&mut self, _event : Event) {}
    fn fetch_events(&mut self) -> Vec<Event> { Vec::new() }
}
