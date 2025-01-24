use std::ptr;

use willhook::{ mouse_hook, MouseButton, MouseButtonPress};
use windows::Win32::{
    Foundation::{HWND, POINT},
    UI::WindowsAndMessaging::{GetCursorPos, WindowFromPoint},
};


pub fn get_hwnd_on_click() -> HWND {
    unsafe { WindowFromPoint(get_mouse_pos_on_click()) }
}
pub fn get_hwnd_on_move() -> HWND {
    unsafe { WindowFromPoint(get_mouse_pos_on_move()) }
}
pub fn get_hwnd_under_mouse() -> HWND {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(ptr::addr_of_mut!(point)).unwrap() };
    unsafe { WindowFromPoint(point) }
}
pub fn get_mouse_pos_on_click() -> POINT {
    loop {
        let event = match mouse_hook().unwrap().recv().unwrap() {
            willhook::InputEvent::Mouse(mouse_event) => match mouse_event.event {
                willhook::MouseEventType::Press(mouse_press_event) => Some(mouse_press_event),
                _ => None,
            },
            _ => None,
        };
        if let Some(event) = event {
            if event.pressed == MouseButtonPress::Down
                && event.button == MouseButton::Left(willhook::MouseClick::SingleClick)
            {
                let mut point = POINT { x: 0, y: 0 };
                unsafe { GetCursorPos(ptr::addr_of_mut!(point)).unwrap() };
                return point;
            }
        }
    }
}
pub fn get_mouse_pos_on_move() -> POINT {
    loop {
        let event = match mouse_hook().unwrap().recv().unwrap() {
            willhook::InputEvent::Mouse(mouse_event) => match mouse_event.event {
                willhook::MouseEventType::Move(mouse_move_event) => Some(mouse_move_event),
                _=>None
            },
            _ => None,
        };
        if let Some(event) = event {
            if let  Some(point) = event.point
            {
                return POINT { x: point.x, y: point.y };
            }
        }
    }
}