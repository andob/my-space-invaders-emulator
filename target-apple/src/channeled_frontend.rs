use std::mem;
use std::sync::mpsc::{Receiver, Sender};
use anyhow::{Context, Result};
use emulator::codeloc;
use emulator::system::frontend::{Event, Frontend, ICanvas, IEventFetcher};

pub struct ChanneledFrontend {}
impl ChanneledFrontend
{
    pub fn new(canvas_command_sender : Sender<Vec<CanvasCommand>>, event_receiver : Receiver<Event>) -> Frontend
    {
        let canvas = Box::new(ChanneledCanvas::new(canvas_command_sender));
        let event_fetcher = Box::new(ChanneledEventFetcher::new(event_receiver));
        return Frontend::new(canvas, event_fetcher);
    }
}

pub enum CanvasCommand
{
    Clear,
    SetDrawColor(u8, u8, u8),
    FillRect(i32, i32, u32, u32),
}

pub struct ChanneledCanvas
{
    sender : Sender<Vec<CanvasCommand>>,
    current_batch : Vec<CanvasCommand>,
}

impl ChanneledCanvas
{
    pub fn new(sender : Sender<Vec<CanvasCommand>>) -> ChanneledCanvas
    {
        return ChanneledCanvas { sender, current_batch:Vec::new() };
    }
}

impl ICanvas for ChanneledCanvas
{
    fn clear(&mut self)
    {
        self.current_batch.push(CanvasCommand::Clear);
    }

    fn set_draw_color(&mut self, r : u8, g : u8, b : u8)
    {
        let command = CanvasCommand::SetDrawColor(r, g, b);
        self.current_batch.push(command);
    }

    fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32)
    {
        let command = CanvasCommand::FillRect(x, y, width, height);
        self.current_batch.push(command);
    }

    fn present(&mut self) -> Result<()>
    {
        let canvas_command_batch = mem::take(&mut self.current_batch);
        return self.sender.send(canvas_command_batch).context(codeloc!());
    }
}

pub struct ChanneledEventFetcher
{
    receiver : Receiver<Event>
}

impl ChanneledEventFetcher
{
    pub fn new(receiver : Receiver<Event>) -> ChanneledEventFetcher
    {
        return ChanneledEventFetcher { receiver };
    }
}

impl IEventFetcher for ChanneledEventFetcher
{
    fn fetch_events(&mut self) -> Vec<Event>
    {
        let mut output_events = Vec::new();

        while let Ok(event) = self.receiver.try_recv()
        {
            output_events.push(event);
        }

        return output_events;
    }
}
