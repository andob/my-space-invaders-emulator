use anyhow::{Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use emulator::codeloc;
use emulator::system::frontend::{Event, Frontend, ICanvas, IEventFetcher, Key};

pub struct WebFrontend {}
impl WebFrontend
{
    pub fn new() -> Result<Frontend>
    {
        let window = window().context(codeloc!())?;
        let document = window.document().context(codeloc!())?;
        let canvas = document.get_element_by_id("canvas").context(codeloc!())?;
        let canvas = canvas.dyn_into::<HtmlCanvasElement>().ok().context(codeloc!())?;
        let context = canvas.get_context("2d").ok().context(codeloc!())?.unwrap();
        let context = context.dyn_into::<CanvasRenderingContext2d>().ok().context(codeloc!())?;

        return Ok(Frontend
        {
            event_fetcher: Box::new(EventFetcher::new()),
            canvas: Box::new(CanvasWrapper { canvas, context }),
        })
    }
}

struct CanvasWrapper
{
    canvas : HtmlCanvasElement,
    context : CanvasRenderingContext2d,
}

impl ICanvas for CanvasWrapper
{
    fn clear(&mut self)
    {
        self.context.clear_rect(0f64, 0f64, self.canvas.width() as f64, self.canvas.height() as f64);
    }

    fn set_draw_color(&mut self, r : u8, g : u8, b : u8)
    {
        let fill_style = format!("#{:02X}{:02X}{:02X}", r, g, b);
        self.context.set_fill_style_str(fill_style.as_str());
    }

    fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32)
    {
        self.context.fill_rect(x as f64, y as f64, width as f64, height as f64);
    }

    fn present(&mut self) {}
}

struct EventFetcher
{
    event_buffer : Vec<Event>
}

impl EventFetcher
{
    pub fn new() -> EventFetcher
    {
        return EventFetcher { event_buffer:Vec::new() };
    }
}

impl IEventFetcher for EventFetcher
{
    fn notify(&mut self, event : Event)
    {
        self.event_buffer.push(event);
    }

    fn fetch_events(&mut self) -> Vec<Event>
    {
        let events = self.event_buffer.clone();
        self.event_buffer.clear();
        return events;
    }
}

pub fn parse_js_key(js_key : String) -> Option<Key>
{
    return match js_key.to_uppercase().as_str()
    {
        "C" => Some(Key::INSERT_COIN),
        "1" => Some(Key::SELECT_ONE_PLAYER),
        "2" => Some(Key::SELECT_TWO_PLAYERS),
        "ARROWLEFT" => Some(Key::PLAYER1_LEFT),
        "ARROWRIGHT" => Some(Key::PLAYER1_RIGHT),
        " " => Some(Key::PLAYER1_SHOOT),
        "A" => Some(Key::PLAYER2_LEFT),
        "D" => Some(Key::PLAYER2_RIGHT),
        "S" => Some(Key::PLAYER2_SHOOT),
        _ => None
    }
}
