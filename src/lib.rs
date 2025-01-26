use std::ptr;

use willhook::{ mouse_hook, Hook, MouseButton, MouseButtonPress};
use windows::Win32::UI::WindowsAndMessaging::{DestroyWindow, PeekMessageA, PM_REMOVE};
use windows::Win32::{
    Foundation::{HWND, POINT},
    UI::WindowsAndMessaging::{GetCursorPos, WindowFromPoint},
};
use std::{
    ffi::{c_int, c_void},
    thread, time,
};
use windows::{
    core::{HRESULT, PCSTR},
    Win32::{
        Foundation::{GetLastError, COLORREF, HINSTANCE, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, CreateSolidBrush, EndPaint, FillRect,
            FrameRect, RedrawWindow,
             PAINTSTRUCT, RDW_ERASE, RDW_FRAME, RDW_INVALIDATE,
        },
        UI::WindowsAndMessaging::{
            CreateWindowExA, DefWindowProcA, DispatchMessageA, GetWindowRect,
             RegisterClassExA, SetLayeredWindowAttributes, SetWindowPos, ShowWindow,
            TranslateMessage, HMENU, HWND_TOP, LWA_COLORKEY, MSG, SHOW_WINDOW_CMD,
            SWP_SHOWWINDOW, WM_PAINT,
            WNDCLASSEXA, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
            WS_EX_TRANSPARENT, WS_POPUP, WS_VISIBLE,
        },
    },
};
#[allow(non_snake_case)]
extern "system" fn callback(hwnd: HWND, uMsg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe {
        return match uMsg {
            WM_PAINT => {
                let mut ps: PAINTSTRUCT = PAINTSTRUCT {
                    ..Default::default()
                };
                let hdc = BeginPaint(hwnd, ptr::addr_of_mut!(ps));
                let hbr = CreateSolidBrush(COLORREF(0x00000000));
                FillRect(hdc, ptr::addr_of!(ps.rcPaint), hbr);
                let hbr = CreateSolidBrush(windows::Win32::Foundation::COLORREF(
                    255 | (100 << 16) | (20 << 8),
                ));
                FrameRect(hdc, ptr::addr_of!(ps.rcPaint), hbr);
                let _ = EndPaint(hwnd, ptr::addr_of_mut!(ps));

                windows::Win32::Foundation::LRESULT(0)
            }
            _ => DefWindowProcA(hwnd, uMsg, wParam, lParam),
        };
    }
}

fn draw_border()->usize{
    unsafe {
        let p1 = PCSTR::from_raw(String::from("class").as_ptr());
        let p2 = PCSTR::from_raw(String::from("name").as_ptr());
        let class = WNDCLASSEXA {
            cbSize: size_of::<WNDCLASSEXA>() as u32,
            lpszClassName: p1,
            lpfnWndProc: Some(callback),
            ..Default::default()
        };
        let r = RegisterClassExA(ptr::addr_of!(class));
        if r == 0 {
            panic!("{:?}", GetLastError().to_hresult().message());
        }
        let hwnd = CreateWindowExA(
            WS_EX_TOPMOST | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW,
            p1,
            p2,
            WS_POPUP | WS_VISIBLE,
            0,
            0,
            0,
            0,
            Some(HWND(0u32 as *mut c_int as *mut c_void)),
            Some(HMENU(0u32 as *mut c_int as *mut c_void)),
            Some(HINSTANCE(0u32 as *mut c_int as *mut c_void)),
            None,
        )
        .unwrap();
        SetLayeredWindowAttributes(hwnd, COLORREF(0x00000000), 100, LWA_COLORKEY).unwrap();
        //5 is for show fullscreen
        ShowWindow(hwnd, SHOW_WINDOW_CMD(5)).unwrap();

        //*mut c_void cant be send between threads so lets convert it to sth that can, definitively safe :)
        let u_hwnd: usize = hwnd.0 as usize;
        let t1_hwnd=u_hwnd.clone();
        let pool_rate = time::Duration::from_millis(5);
        let t1 = thread::spawn(move || {
            let mouse_hook = mouse_hook().unwrap();
            let hwnd = HWND((t1_hwnd) as *mut c_void);
            let mut old_hwnd=get_hwnd_under_mouse();
            loop {
                thread::sleep(pool_rate);

                let (other_hwnd,clicked):(HWND,bool) = get_hwnd_on_move_with_click(Some(&mouse_hook));
                if clicked{
                    let _ = DestroyWindow(hwnd);
                    return other_hwnd.0 as usize;
                }
                if other_hwnd == hwnd {
                    panic!();
                }
                if old_hwnd==other_hwnd{
                    continue;
                }
                let mut rect = RECT {..Default::default()};
                if GetWindowRect(other_hwnd, ptr::addr_of_mut!(rect)).is_err() {
                    continue;
                }
                SetWindowPos(hwnd,Some(HWND_TOP),rect.left,rect.top,rect.right - rect.left,rect.bottom - rect.top,SWP_SHOWWINDOW,).unwrap();
                old_hwnd=other_hwnd.clone();
            }
        });

        let mut msg = MSG {..Default::default()};
        let hwnd = HWND(u_hwnd as *mut c_void);
        loop {
            let result=PeekMessageA(ptr::addr_of_mut!(msg), Some( hwnd), 0, 0,PM_REMOVE).as_bool();
            if t1.is_finished(){
                break;
            }
            if !result{
                thread::sleep(time::Duration::from_millis(5));
                continue;
            }
          
            let z = TranslateMessage(ptr::addr_of_mut!(msg));
            let r = z.ok();
            if !r.is_ok() && r.as_ref().unwrap_err().code() != HRESULT(0) {
                panic!("{:?}", z);
            }
            DispatchMessageA(ptr::addr_of_mut!(msg));
        }
        return  t1.join().unwrap();
    }
}

pub fn get_hwnd_on_click(border:bool) -> HWND {
    if border{
        let t=thread::spawn(move ||{draw_border()});
        let r=t.join().unwrap();
        return HWND(r as *mut c_void);
    }
        return unsafe { WindowFromPoint(get_mouse_pos_on_click()) };
    

}
pub fn get_hwnd_on_move(hook:Option<&Hook>) -> HWND {
    unsafe { WindowFromPoint(get_mouse_pos_on_move(hook)) }
}
fn get_hwnd_on_move_with_click(hook:Option<&Hook>)->(HWND,bool){
    let (point,b)=get_mouse_pos_on_move_with_click(hook);
    return (unsafe { WindowFromPoint(point) },b);
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
pub fn get_mouse_pos_on_move(hook:Option<&Hook>) -> POINT {
    let hook=match hook {
        Some(v) => v,
        None => &mouse_hook().unwrap(),
    };
    loop {
        let event = match hook.recv().unwrap() {
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
fn get_mouse_pos_on_move_with_click(hook:Option<&Hook>) -> (POINT,bool) {
   
    let hook=match hook {
        Some(v) => v,
        None => &mouse_hook().unwrap(),
    };
    loop {
        // let hook=hook.unwrap();
        let event = match hook.recv().unwrap() {
            willhook::InputEvent::Mouse(mouse_event) => match mouse_event.event {
                willhook::MouseEventType::Move(mouse_move_event) => Some(mouse_move_event),
                willhook::MouseEventType::Press(mouse_press_event) => match mouse_press_event.button {
                    MouseButton::Left(_) => match mouse_press_event.pressed {
                        MouseButtonPress::Down => 
                        {
                            let mut point = POINT { ..Default::default() };
                            unsafe { GetCursorPos(ptr::addr_of_mut!(point)).unwrap() };
                            return (point,true);
                        }
                        ,
                        _=>None,
                    }
                _=>None
            },
            _ => None,
        }
            _=>None,
    };
        if let Some(event) = event {
            if let  Some(point) = event.point
            {
                return (POINT { x: point.x, y: point.y },false);
            }
        }
    }
}