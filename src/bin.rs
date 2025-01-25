use std::{
    ffi::{c_int, c_void},
    ptr, time,
};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{GetLastError, HINSTANCE, HWND, RECT},
        Graphics::Gdi::{CreateSolidBrush, DrawTextA, FillRect, GetDC, DT_CENTER, HBRUSH},
        UI::WindowsAndMessaging::{
            CreateWindowExA, RegisterClassA, RegisterClassExA, HMENU, WNDCLASSA, WNDCLASSEXA, WNDCLASSEXW, WS_EX_LAYERED, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_MAXIMIZE, WS_POPUP, WS_SYSMENU, WS_VISIBLE
        },
    },
};

fn main() {
    unsafe {
        // let hwnd=windowpicker::get_hwnd_on_click();
        let p1 = PCSTR::from_raw(String::from("class").as_ptr());
        let p2 = PCSTR::from_raw(String::from("name").as_ptr());
        let class=WNDCLASSEXA {
            cbSize:size_of::<WNDCLASSEXA>() as u32,
            lpszClassName: p2,
            style:
            ..Default::default()
        };
        let r= RegisterClassExA(ptr::addr_of!(class));
        if r==0{
           println!("{:?}",GetLastError().to_hresult().message());
        }

        let hwnd = CreateWindowExA(
            WS_EX_TOPMOST | WS_EX_LAYERED | WS_EX_TRANSPARENT,
            p1,
            p2,
            WS_POPUP | WS_VISIBLE,
            0,
            0,
            200,
            200,
            Some(HWND(0u32 as *mut c_int as *mut c_void)),
            Some(HMENU(0u32 as *mut c_int as *mut c_void)),
            Some(HINSTANCE(0u32 as *mut c_int as *mut c_void)),
            None,
        )
        .unwrap();
        println!("{:?}", hwnd);
        let hdc = unsafe { GetDC(Some(hwnd)) };
        println!("{:?}", hdc);
        let mut sti = String::from("text");
        let sti = sti.as_bytes_mut();
        let mut lprc = RECT {
            left: -100,
            top: -100,
            right: 10 + 200,
            bottom: 200 + 60,
        };
        let lprc = ptr::addr_of_mut!(lprc);
        // let mut hwnd=0;
        // let hwnd:HWND=HWND(&mut hwnd as *mut c_int as *mut c_void);
        let hbr = CreateSolidBrush(windows::Win32::Foundation::COLORREF(
            255 | (100 << 16) | (20 << 8),
        ));
        let hdc = unsafe { GetDC(Some(hwnd)) };
        loop {
            // DrawTextA(hdc, sti, lprc, DT_CENTER);

            FillRect(hdc, lprc, hbr);
        }
    }
}
