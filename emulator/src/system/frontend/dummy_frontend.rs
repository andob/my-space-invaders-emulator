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
    fn set_draw_color(&mut self, r : u8, g : u8, b : u8) {}
    fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32) {}
    fn present(&mut self) {}
}

impl IEventFetcher for DummyFrontendEventFetcher
{
    fn notify(&mut self, event : Event) {}
    fn fetch_events(&mut self) -> Vec<Event> { Vec::new() }
}
