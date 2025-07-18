use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use maplit2::hashmap;
use objc2::{define_class, msg_send, sel, DeclaredClass, MainThreadMarker, MainThreadOnly};
use objc2::rc::Retained;
use objc2::runtime::NSObjectProtocol;
use objc2_app_kit::{NSEvent, NSGraphicsContext, NSView};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_core_graphics::{CGColor, CGContext};
use objc2_foundation::{NSDefaultRunLoopMode, NSRect, NSRunLoop, NSTimeInterval, NSTimer};
use emulator::system::frontend::{Event, Key, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::channeled_frontend::CanvasCommand;

const FPS : NSTimeInterval = 1.0 / 45.0;

define_class!(
    #[unsafe(super(NSView))]
    #[thread_kind = MainThreadOnly]
    #[ivars = ContentViewInterfaceVars]
    pub struct ContentView;

    unsafe impl NSObjectProtocol for ContentView {}

    impl ContentView
    {
        #[unsafe(method(drawRect:))]
        unsafe fn draw_rect(&self, _dirty : NSRect)
        {
            let context = &*NSGraphicsContext::currentContext().unwrap().CGContext();
            self.receive_and_execute_canvas_commands(context);
        }

        #[unsafe(method(refreshTimerTick:))]
        unsafe fn refresh_timer_tick(&self, _timer : &NSTimer)
        {
            self.setNeedsDisplay(true);
        }

        #[unsafe(method(acceptsFirstResponder))]
        unsafe fn accepts_first_responder(&self) -> bool { true }

        #[unsafe(method(keyDown:))]
        unsafe fn key_down(&self, event : &NSEvent)
        {
            self.send_key_down_event(event);
        }

        #[unsafe(method(keyUp:))]
        unsafe fn key_up(&self, event : &NSEvent)
        {
            self.send_key_up_event(event);
        }
    }
);

pub struct ContentViewInterfaceVars
{
    event_sender : Sender<Event>,
    canvas_command_receiver : Receiver<Vec<CanvasCommand>>,
    keymap : HashMap<String, Key>,
}

impl ContentViewInterfaceVars
{
    pub fn new(event_sender : Sender<Event>, canvas_command_receiver : Receiver<Vec<CanvasCommand>>) -> ContentViewInterfaceVars
    {
        return ContentViewInterfaceVars { event_sender, canvas_command_receiver, keymap:get_keymap() };
    }
}

impl ContentView
{
    pub unsafe fn new(main_thread_marker : MainThreadMarker, frame : NSRect, vars : ContentViewInterfaceVars) -> Retained<ContentView>
    {
        let object = ContentView::alloc(main_thread_marker);
        let object = object.set_ivars(vars);
        let object : Retained<ContentView> = msg_send![super(object), initWithFrame:frame];

        let refresh_timer = NSTimer::scheduledTimerWithTimeInterval_target_selector_userInfo_repeats
            (FPS, &*****object, sel!(refreshTimerTick:), None, true);
        NSRunLoop::currentRunLoop().addTimer_forMode(&refresh_timer, NSDefaultRunLoopMode);

        return object;
    }

    unsafe fn receive_and_execute_canvas_commands(&self, context : &CGContext)
    {
        while let Ok(canvas_command_batch) = self.ivars().canvas_command_receiver.try_recv()
        {
            for canvas_command in canvas_command_batch
            {
                self.execute_canvas_command(canvas_command, context);
            }
        }
    }

    unsafe fn execute_canvas_command(&self, canvas_command : CanvasCommand, context : &CGContext)
    {
        match canvas_command
        {
            CanvasCommand::Clear =>
            {
                let black = CGColor::new_srgb(0.0, 0.0, 0.0, 1.0);
                CGContext::set_fill_color_with_color(Some(context), Some(&black));

                let window_size = CGSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
                let rect = CGRect::new(CGPoint::new(0.0, 0.0), window_size);
                CGContext::clear_rect(Some(context), rect);
            }

            CanvasCommand::SetDrawColor(r, g, b) =>
            {
                let (r, g, b) = ((r as f64)/255.0, (g as f64)/255.0, (b as f64)/255.0);
                let color = CGColor::new_srgb(r, g, b, 1.0);
                CGContext::set_fill_color_with_color(Some(context), Some(&color));
            }

            CanvasCommand::FillRect(x, y, width, height) =>
            {
                let location = CGPoint::new(x as f64, (WINDOW_HEIGHT as f64) - (y as f64));
                let size = CGSize::new(width as f64, height as f64);
                let rect = CGRect::new(location, size);
                CGContext::fill_rect(Some(context), rect);
            }
        }
    }

    unsafe fn send_key_down_event(&self, event : &NSEvent)
    {
        let key_as_string = event.charactersIgnoringModifiers().unwrap_or_default().to_string();
        if let Some(key) = self.ivars().keymap.get(&key_as_string)
        {
            self.ivars().event_sender.send(Event::KeyDown(*key)).unwrap_or_default();
        }
    }

    unsafe fn send_key_up_event(&self, event : &NSEvent)
    {
        let key_as_string = event.charactersIgnoringModifiers().unwrap_or_default().to_string();
        if let Some(key) = self.ivars().keymap.get(&key_as_string)
        {
            self.ivars().event_sender.send(Event::KeyUp(*key)).unwrap_or_default();
        }
    }
}

fn get_keymap() -> HashMap<String, Key>
{
    return hashmap!
    {
        String::from('c') => Key::INSERT_COIN,
        String::from('1') => Key::SELECT_ONE_PLAYER,
        String::from('2') => Key::SELECT_TWO_PLAYERS,
        String::from('\u{F702}') => Key::PLAYER1_LEFT,
        String::from('\u{F703}') => Key::PLAYER1_RIGHT,
        String::from(' ') => Key::PLAYER1_SHOOT,
        String::from('a') => Key::PLAYER2_LEFT,
        String::from('d') => Key::PLAYER2_RIGHT,
        String::from('s') => Key::PLAYER2_SHOOT,
    };
}
