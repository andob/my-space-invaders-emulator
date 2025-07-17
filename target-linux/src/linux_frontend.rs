use std::collections::HashMap;
use std::rc::Rc;
use anyhow::{anyhow, Context, Result};
use maplit2::hashmap;
use xcb::{x, Connection};
use xcb::x::{ChangeGc, ChangeProperty, CopyArea, CreateGc, CreatePixmap, CreateWindow, Cw, Drawable, EventMask, Gc, Gcontext, MapWindow, Pixmap, PolyFillRectangle, PropMode, Rectangle, Window, WindowClass, ATOM_STRING, ATOM_WM_NAME, COPY_FROM_PARENT};
use emulator::codeloc;
use emulator::system::frontend::{Event, Frontend, ICanvas, IEventFetcher, Key, WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE};

pub struct LinuxFrontend {}
impl LinuxFrontend
{
    pub fn new() -> Result<Frontend>
    {
        let (connection, _screen_number) = Connection::connect(None).context(codeloc!())?;
        let screen = connection.get_setup().roots().last().ok_or(anyhow!(codeloc!()))?;

        let window = connection.generate_id::<Window>();
        let cookie = connection.send_request_checked(&CreateWindow
        {
            depth: COPY_FROM_PARENT as u8, wid: window, parent: screen.root(),
            x: 0, y: 0, width: WINDOW_WIDTH as u16, height: WINDOW_HEIGHT as u16,
            border_width: 0, class: WindowClass::InputOutput,
            visual: screen.root_visual(), value_list: &[
                Cw::BackPixel(screen.black_pixel()),
                Cw::EventMask(EventMask::KEY_PRESS | EventMask::KEY_RELEASE),
            ],
        });

        connection.check_request(cookie).context(codeloc!())?;

        let cookie = connection.send_request_checked(&ChangeProperty
        {
            mode: PropMode::Replace, window,
            property: ATOM_WM_NAME, r#type: ATOM_STRING,
            data: WINDOW_TITLE.as_bytes(),
        });

        connection.check_request(cookie).context(codeloc!())?;

        let graphics_context_value_list =
        [
            Gc::Foreground(screen.white_pixel()),
            Gc::Background(screen.black_pixel()),
            Gc::GraphicsExposures(false), //disable exposure events
        ];

        let window_graphics_context = connection.generate_id::<Gcontext>();
        let cookie = connection.send_request_checked(&CreateGc
        {
            cid: window_graphics_context,
            drawable: Drawable::Window(window),
            value_list: &graphics_context_value_list,
        });

        connection.check_request(cookie).context(codeloc!())?;

        let pixmap = connection.generate_id::<Pixmap>();
        let cookie = connection.send_request_checked(&CreatePixmap
        {
            depth: screen.root_depth(), pid: pixmap,
            drawable: Drawable::Window(window),
            width: WINDOW_WIDTH as u16,
            height: WINDOW_HEIGHT as u16,
        });

        connection.check_request(cookie).context(codeloc!())?;

        let pixmap_graphics_context = connection.generate_id::<Gcontext>();
        let cookie = connection.send_request_checked(&CreateGc
        {
            cid: pixmap_graphics_context,
            drawable: Drawable::Pixmap(pixmap),
            value_list: &graphics_context_value_list,
        });

        connection.check_request(cookie).context(codeloc!())?;

        //show the window (AKA map the window)
        connection.send_request(&MapWindow { window });
        connection.flush().context(codeloc!())?;

        let connection_ref = Rc::new(connection);
        let canvas = Box::new(Canvas
        {
            connection: connection_ref.clone(),
            window, window_graphics_context,
            pixmap, pixmap_graphics_context,
        });

        let event_fetcher = Box::new(EventFetcher::new(connection_ref));
        return Ok(Frontend::new(canvas, event_fetcher));
    }
}

struct Canvas
{
    connection : Rc<Connection>,
    window : Window,
    window_graphics_context : Gcontext,
    pixmap : Pixmap,
    pixmap_graphics_context : Gcontext,
}

impl ICanvas for Canvas
{
    fn clear(&mut self)
    {
        self.set_draw_color(0, 0, 0);
        self.fill_rect(0, 0, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    }

    fn set_draw_color(&mut self, r : u8, g : u8, b : u8)
    {
        let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        self.connection.send_request(&ChangeGc
        {
            gc: self.pixmap_graphics_context,
            value_list: &[ Gc::Foreground(color) ],
        });
    }

    fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32)
    {
        let rectangle = Rectangle
        {
            x: x as i16, y: y as i16,
            width: width as u16, height: height as u16,
        };

        self.connection.send_request(&PolyFillRectangle
        {
            drawable: Drawable::Pixmap(self.pixmap),
            gc: self.pixmap_graphics_context,
            rectangles: &[rectangle],
        });
    }

    fn present(&mut self) -> Result<()>
    {
        let cookie = self.connection.send_request_checked(&CopyArea
        {
            src_drawable: Drawable::Pixmap(self.pixmap),
            dst_drawable: Drawable::Window(self.window),
            gc: self.window_graphics_context,
            src_x: 0, src_y: 0, dst_x: 0, dst_y: 0,
            width: WINDOW_WIDTH as u16,
            height: WINDOW_HEIGHT as u16,
        });

        self.connection.check_request(cookie).context(codeloc!())?;
        self.connection.flush().context(codeloc!())?;
        return Ok(());
    }
}

struct EventFetcher
{
    connection : Rc<Connection>,
    keymap : HashMap<u8, Key>,
}

impl EventFetcher
{
    pub fn new(connection : Rc<Connection>) -> EventFetcher
    {
        let keymap = hashmap!
        {
            /*C*/ 0x36 => Key::INSERT_COIN,
            /*1*/ 0x0A => Key::SELECT_ONE_PLAYER,
            /*2*/ 0x0B => Key::SELECT_TWO_PLAYERS,
            /*←*/ 0x71 => Key::PLAYER1_LEFT,
            /*→*/ 0x72 => Key::PLAYER1_RIGHT,
            /* */ 0x41 => Key::PLAYER1_SHOOT,
            /*A*/ 0x26 => Key::PLAYER2_LEFT,
            /*D*/ 0x28 => Key::PLAYER2_RIGHT,
            /*S*/ 0x27 => Key::PLAYER2_SHOOT,
        };

        return EventFetcher { connection, keymap };
    }
}

impl IEventFetcher for EventFetcher
{
    fn fetch_events(&mut self) -> Vec<Event>
    {
        let mut output_events = Vec::new();

        while let Ok(Some(event)) = self.connection.poll_for_event()
        {
            match event
            {
                xcb::Event::X(x::Event::KeyPress(key)) =>
                {
                    if let Some(key) = self.keymap.get(&(key.detail() as u8))
                    {
                        output_events.push(Event::KeyDown(*key));
                    }
                }
                xcb::Event::X(x::Event::KeyRelease(key)) =>
                {
                    if let Some(key) = self.keymap.get(&(key.detail() as u8))
                    {
                        output_events.push(Event::KeyUp(*key));
                    }
                }
                _ => {}
            }
        }

        return output_events;
    }
}
