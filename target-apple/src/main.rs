mod channeled_frontend;
mod content_view;

use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
use anyhow::{Context, Result};
use block2::RcBlock;
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2::rc::Retained;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSBackingStoreType, NSWindow, NSWindowStyleMask};
use objc2_foundation::{NSNotificationCenter, NSNotificationName, NSPoint, NSRect, NSSize, NSString, NSTimeInterval};
use emulator::codeloc;
use emulator::system::frontend::{Event, WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use emulator::system::System;
use crate::channeled_frontend::{CanvasCommand, ChanneledFrontend};
use crate::content_view::{ContentView, ContentViewInterfaceVars};

const ROM_BYTES : &[u8] = include_bytes!("../../game.rom");

fn main() -> Result<()>
{
    let (canvas_command_sender, canvas_command_receiver) = channel::<Vec<CanvasCommand>>();
    let (event_sender, event_receiver) = channel::<Event>();

    thread::spawn(||
    {
        let frontend = ChanneledFrontend::new(canvas_command_sender, event_receiver);
        let mut system = System::new(ROM_BYTES, frontend);

        loop
        {
            system.render_next_frame().unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    return unsafe { app_main(event_sender, canvas_command_receiver) };
}

unsafe fn app_main(event_sender : Sender<Event>, canvas_command_receiver : Receiver<Vec<CanvasCommand>>) -> Result<()>
{
    let main_thread_marker = MainThreadMarker::new().context(codeloc!())?;

    let app = Rc::new(NSApplication::sharedApplication(main_thread_marker));
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

    let window_size = NSSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
    let window_frame = NSRect::new(NSPoint::new(0., 0.), window_size);
    let window_style = NSWindowStyleMask::Titled | NSWindowStyleMask::Closable | NSWindowStyleMask::Miniaturizable;
    let window = NSWindow::initWithContentRect_styleMask_backing_defer(
        NSWindow::alloc(main_thread_marker), window_frame, window_style, NSBackingStoreType::Buffered, false);

    let view_args = ContentViewInterfaceVars::new(event_sender, canvas_command_receiver);
    let view = ContentView::new(main_thread_marker, window_frame, view_args);
    window.setContentView(Some(&*view));
    window.setInitialFirstResponder(Some(&*view));

    window.center();
    window.setTitle(&NSString::from_str(WINDOW_TITLE));
    window.makeKeyAndOrderFront(None);

    terminate_app_on_window_close(app.clone(), &window);

    app.activate();
    app.run();
    return Ok(());
}

unsafe fn terminate_app_on_window_close(app : Rc<Retained<NSApplication>>, window : &Retained<NSWindow>)
{
    let notification_name = NSNotificationName::from_str("NSWindowWillCloseNotification");
    let window_closed_handler = RcBlock::new(move |_| { app.terminate(None) });

    NSNotificationCenter::defaultCenter().addObserverForName_object_queue_usingBlock(
        Some(&notification_name), Some(&window), None, &*window_closed_handler);
}
