use anyhow::{Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use emulator::codeloc;
use emulator::system::frontend::{Frontend, ICanvas, Key};
use emulator::system::frontend::dummy_frontend::DummyFrontendEventFetcher;

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

        let canvas = Box::new(CanvasWrapper::new(canvas, context));
        let event_fetcher = Box::new(DummyFrontendEventFetcher{});
        return Ok(Frontend::new(canvas, event_fetcher));
    }
}

struct CanvasWrapper
{
    canvas : HtmlCanvasElement,
    context : CanvasRenderingContext2d,
}

impl CanvasWrapper
{
    pub fn new(canvas : HtmlCanvasElement, context : CanvasRenderingContext2d) -> CanvasWrapper
    {
        return CanvasWrapper { canvas, context };
    }
}

impl ICanvas for CanvasWrapper
{
    fn clear(&mut self)
    {
        let canvas_width = self.canvas.width() as f64;
        let canvas_height = self.canvas.height() as f64;
        self.context.clear_rect(0f64, 0f64, canvas_width, canvas_height);
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

    fn present(&mut self) -> Result<()> { Ok(()) }
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
