use anyhow::{Context, Result};
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use crate::codeloc;
use crate::system::frontend::{Event, Frontend, ICanvas, IEventFetcher};

struct EventFetcher {}

struct CanvasWrapper
{
    canvas : HtmlCanvasElement,
    context : CanvasRenderingContext2d,
}

impl Frontend
{
    // #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
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
            event_fetcher: Box::new(EventFetcher {}),
            canvas: Box::new(CanvasWrapper { canvas, context }),
        })
    }
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

impl IEventFetcher for EventFetcher
{
    fn fetch_events(&mut self) -> Vec<Event>
    {
        return Vec::new();
    }
}
