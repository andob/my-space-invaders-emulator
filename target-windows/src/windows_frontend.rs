use std::ffi::OsStr;
use std::collections::HashMap;
use std::iter;
use std::os::windows::ffi::OsStrExt;
use anyhow::{Context, Result};
use maplit2::hashmap;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC, DeleteObject, FillRect, GetDC, ReleaseDC, SelectObject, UpdateWindow, HBITMAP, HBRUSH, HDC, SRCCOPY};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_1, VK_2, VK_A, VK_C, VK_D, VK_LEFT, VK_RIGHT, VK_S, VK_SPACE};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetSystemMetrics, LoadCursorW, PeekMessageW, PostQuitMessage, RegisterClassExW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, HICON, IDC_ARROW, MSG, PM_REMOVE, SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY, WM_KEYDOWN, WM_KEYUP, WNDCLASSEXW, WS_OVERLAPPED, WS_SYSMENU};
use emulator::system::frontend::{Event, Frontend, Key, WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use emulator::system::frontend::unsafe_frontend::{IUnsafeCanvas, IUnsafeEventFetcher};
use emulator::codeloc;

const CLASS_NAME : &str = "EmulatorWindowClass";

pub struct WindowsFrontend {}
impl WindowsFrontend
{
    pub fn new() -> Result<Frontend>
    {
        return unsafe { Self::new_unsafe() };
    }

    #[allow(unused_must_use)]
    pub unsafe fn new_unsafe() -> Result<Frontend>
    {
        let class_name = encode_as_wide_string(CLASS_NAME);
        let window_title = encode_as_wide_string(WINDOW_TITLE);

        let window_class = WNDCLASSEXW
        {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(Self::window_events_callback),
            cbClsExtra: 0, cbWndExtra: 0,
            hInstance: HINSTANCE::default(),
            hIcon: HICON::default(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: HBRUSH::default(),
            lpszMenuName: PCWSTR::default(),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hIconSm: HICON::default(),
        };

        RegisterClassExW(&window_class);

        let handle_to_window = CreateWindowExW(
            /*exStyle*/ WINDOW_EX_STYLE::default(),
            /*className*/ PCWSTR(class_name.as_ptr()),
            /*title*/ PCWSTR(window_title.as_ptr()),
            /*style*/ WS_OVERLAPPED | WS_SYSMENU,
            /*x*/ GetSystemMetrics(SM_CXSCREEN)/2 - (WINDOW_WIDTH/2) as i32,
            /*y*/ GetSystemMetrics(SM_CYSCREEN)/2 - (WINDOW_HEIGHT/2) as i32,
            /*width*/ WINDOW_WIDTH as i32,
            /*height*/ WINDOW_HEIGHT as i32,
            /*parent*/ None, /*menu*/ None,
            /*hinstance*/ Some(HINSTANCE::default()),
            /*lpparam*/ None
        ).context(codeloc!())?;

        ShowWindow(handle_to_window, SW_SHOW);
        UpdateWindow(handle_to_window);

        let window_graphics_context = GetDC(Some(handle_to_window));
        let memory_graphics_context = CreateCompatibleDC(Some(window_graphics_context));
        let bitmap = CreateCompatibleBitmap(window_graphics_context, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        SelectObject(memory_graphics_context, bitmap.into());

        let canvas = Box::new(Canvas
        {
            handle_to_window, window_graphics_context, memory_graphics_context,
            bitmap, current_brush: CreateSolidBrush(COLORREF(0))
        });

        let event_fetcher = Box::new(EventFetcher::new(handle_to_window));
        return Ok(Frontend::new(canvas, event_fetcher));
    }

    unsafe extern "system" fn window_events_callback(hwnd : HWND, message: u32, wparam : WPARAM, lparam : LPARAM) -> LRESULT
    {
        if message == WM_DESTROY
        {
            PostQuitMessage(0);
            return LRESULT(0);
        }

        return DefWindowProcW(hwnd, message, wparam, lparam);
    }
}

fn encode_as_wide_string(string : &str) -> Vec<u16>
{
    return OsStr::new(string).encode_wide().chain(iter::once(0)).collect();
}

pub struct Canvas
{
    handle_to_window : HWND,
    window_graphics_context : HDC,
    memory_graphics_context : HDC,
    bitmap : HBITMAP,
    current_brush : HBRUSH,
}

#[allow(unused_must_use)]
impl IUnsafeCanvas for Canvas
{
    unsafe fn clear(&mut self)
    {
        IUnsafeCanvas::set_draw_color(self, 0, 0, 0);
        IUnsafeCanvas::fill_rect(self, 0, 0, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    }

    unsafe fn set_draw_color(&mut self, r : u8, g : u8, b : u8)
    {
        let color = (r as u32) | ((g as u32) << 8) | ((b as u32) << 16);

        DeleteObject(self.current_brush.into());
        self.current_brush = CreateSolidBrush(COLORREF(color));
    }

    unsafe fn fill_rect(&mut self, x : i32, y : i32, width : u32, height : u32)
    {
        let right = x + (width as i32);
        let bottom = y + (height as i32);
        let rect = RECT { left:x, top:y, right, bottom };

        FillRect(self.memory_graphics_context, &rect, self.current_brush);
    }

    unsafe fn present(&mut self) -> Result<()>
    {
        //copy pixels from one graphics context to another
        return BitBlt(
            /*destination context*/ self.window_graphics_context,
            /*destination x,y*/ 0, 0,
            /*width, height*/ WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32,
            /*source context*/ Some(self.memory_graphics_context),
            /*source x,y*/ 0, 0,
            /*operation*/ SRCCOPY,
        ).context(codeloc!());
    }
}

#[allow(unused_must_use)]
impl Drop for Canvas
{
    fn drop(&mut self)
    {
        unsafe
        {
            DeleteObject(self.bitmap.into());
            DeleteObject(self.current_brush.into());
            DeleteDC(self.memory_graphics_context);
            ReleaseDC(Some(self.handle_to_window), self.window_graphics_context);
        }
    }
}

pub struct EventFetcher
{
    handle_to_window : HWND,
    keymap : HashMap<u16, Key>,
}

impl EventFetcher
{
    pub fn new(handle_to_window : HWND) -> EventFetcher
    {
        let keymap = hashmap!
        {
            VK_C.0 => Key::INSERT_COIN,
            VK_1.0 => Key::SELECT_ONE_PLAYER,
            VK_2.0 => Key::SELECT_TWO_PLAYERS,
            VK_LEFT.0 => Key::PLAYER1_LEFT,
            VK_RIGHT.0 => Key::PLAYER1_RIGHT,
            VK_SPACE.0 => Key::PLAYER1_SHOOT,
            VK_A.0 => Key::PLAYER2_LEFT,
            VK_S.0 => Key::PLAYER2_RIGHT,
            VK_D.0 => Key::PLAYER2_SHOOT,
        };

        return EventFetcher { handle_to_window, keymap };
    }
}

impl IUnsafeEventFetcher for EventFetcher
{
    #[allow(unused_must_use)]
    unsafe fn fetch_events(&mut self) -> Vec<Event>
    {
        let mut output_events = Vec::new();

        let mut message = MSG::default();
        while PeekMessageW(&mut message, Some(self.handle_to_window), 0, 0, PM_REMOVE).as_bool()
        {
            TranslateMessage(&message);
            DispatchMessageW(&message);

            if message.message == WM_KEYDOWN
            {
                if let Some(key) = self.keymap.get(&(message.wParam.0 as u16))
                {
                    output_events.push(Event::KeyDown(*key));
                }
            }
            else if message.message == WM_KEYUP
            {
                if let Some(key) = self.keymap.get(&(message.wParam.0 as u16))
                {
                    output_events.push(Event::KeyUp(*key));
                }
            }
        }

        return output_events;
    }
}
