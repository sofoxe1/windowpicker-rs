use std::{
    borrow::Borrow, ffi::{c_int, c_void}, ptr, sync::{Arc, Mutex}, thread, time
};
use windows::{
    core::{HRESULT, PCSTR},
    Win32::{
        Foundation::{GetLastError, COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::{BeginPaint, CreateHatchBrush, CreateSolidBrush, DrawTextA, EndPaint, FillRect, FillRgn, FrameRect, GetDC, RedrawWindow, COLOR_WINDOW, DT_CENTER, HBRUSH, HDC, HRGN, HS_HORIZONTAL, HS_VERTICAL, PAINTSTRUCT, RDW_ERASE, RDW_FRAME, RDW_INVALIDATE},
        UI::WindowsAndMessaging::{
            CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, GetWindowRect, RegisterClassA, RegisterClassExA, SetLayeredWindowAttributes, SetWindowPos, ShowWindow, TranslateMessage, HMENU, HWND_TOP, LWA_ALPHA, LWA_COLORKEY, MSG, SHOW_WINDOW_CMD, SWP_NOSIZE, SWP_SHOWWINDOW, SW_MAX, SW_MAXIMIZE, WM_DESTROY, WM_PAINT, WNDCLASSA, WNDCLASSEXA, WNDCLASSEXW, WNDPROC, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_MAXIMIZE, WS_POPUP, WS_SYSMENU, WS_VISIBLE
        },
    },
};
 extern "system" fn callback(hwnd:HWND, uMsg:u32, wParam:WPARAM, lParam:LPARAM) -> LRESULT{
    // println!("hwnd:{:?},uMsg:{:?},wprarm:{:?},lparam:{:?}",hwnd,uMsg,wParam,lParam);
    // if idk==WM_DESTROY{
    //     println!("destroy");
    // }
    
  return   match uMsg{
        WM_PAINT=>{
            unsafe {
                println!("draw");
            let mut ps:PAINTSTRUCT=PAINTSTRUCT{..Default::default()};
            let hdc = BeginPaint(hwnd,  ptr::addr_of_mut!(ps));
            let hbr = CreateSolidBrush(windows::Win32::Foundation::COLORREF(
                        255 | (100 << 16) | (20 << 8),
                    ));
            FrameRect(hdc, ptr::addr_of!(ps.rcPaint), hbr);
            // let _ = FillRgn(hdc, ptr::addr_of!(ps.rcPaint), hbr);
            
    //         let hbr = CreateHatchBrush(HS_VERTICAL,windows::Win32::Foundation::COLORREF(
    //             255 | (100 << 16) | (20 << 8),
    //         ));
    // let _ = FillRect(hdc, ptr::addr_of!(ps.rcPaint), hbr);
            let _ = EndPaint(hwnd, ptr::addr_of_mut!(ps));
            windows::Win32::Foundation::LRESULT(0)
            }
        },
        _=>unsafe { DefWindowProcA(hwnd, uMsg, wParam, lParam) }
    };
}

fn main() {
    unsafe {
        // let hwnd=windowpicker::get_hwnd_on_click();
        let p1 = PCSTR::from_raw(String::from("class").as_ptr());
        let p2 = PCSTR::from_raw(String::from("name").as_ptr());
        let class=WNDCLASSEXA {
            cbSize:size_of::<WNDCLASSEXA>() as u32,
            lpszClassName: p1,
            lpfnWndProc: Some(callback),
            ..Default::default()
        };
        let r= RegisterClassExA(ptr::addr_of!(class));
        if r==0{
           println!("{:?}",GetLastError().to_hresult().message());
        }
        else {
            println!("s:{}",r);
        }
        println!("s2");
        let hwnd = CreateWindowExA(
            WS_EX_TOPMOST | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW ,
            // WS_EX_TOPMOST,
            p1,
            p2,
            WS_POPUP|WS_VISIBLE,
            0,
            0,
            200,
            200,
            Some(HWND(0u32 as *mut c_int as *mut c_void)),
            Some(HMENU(0u32 as *mut c_int as *mut c_void)),
            Some(HINSTANCE(0u32 as *mut c_int as *mut c_void)),
            None,
        ).unwrap();
        let crkey=COLORREF(0x00000000);
        SetLayeredWindowAttributes(hwnd, crkey, 100, LWA_COLORKEY).unwrap();

        ShowWindow(hwnd, SHOW_WINDOW_CMD(5)).unwrap();
        println!("{:?}",hwnd);

        //its safe... hopefully
        let s_hwnd: usize=hwnd.0 as usize;
        let s_hwnd=Arc::new(s_hwnd);
        let t_hwnd=s_hwnd.clone();
        let pool_rate=time::Duration::from_millis(5);
        let t1=thread::spawn(move ||{
            thread::sleep(std::time::Duration::from_millis(100));
            let lock= t_hwnd;
            // let hwnd=HWND((*lock) as *mut c_void );
            loop {
                thread::sleep(pool_rate);

                let hwnd=HWND((*lock) as *mut c_void );
                let pos=windowpicker::get_mouse_pos_on_move();
                let other_hwnd=windowpicker::get_hwnd_on_move();
                if other_hwnd==HWND((*lock) as *mut c_void){
                    panic!();
                }
                let mut rect=RECT{..Default::default()};
                if GetWindowRect(other_hwnd, ptr::addr_of_mut!(rect)).is_err(){
                    continue;
                }
                // println!("{}:{}:{}:{}",rect.left,rect.top, rect.right-rect.left, rect.bottom-rect.top);
                SetWindowPos(HWND((*lock) as *mut c_void ), Some(HWND_TOP) ,rect.left,rect.top, rect.right-rect.left, rect.bottom-rect.top, SWP_SHOWWINDOW).unwrap();
                // RedrawWindow(Some(HWND((*lock) as *mut c_void )), None, None, RDW_FRAME|RDW_INVALIDATE|RDW_ERASE);
                let mut ps:PAINTSTRUCT=PAINTSTRUCT{..Default::default()};
                let hdc = BeginPaint(hwnd,  ptr::addr_of_mut!(ps));
                let hbr = CreateSolidBrush(COLORREF(0x00000000));
                FillRect(hdc, ptr::addr_of!(ps.rcPaint), hbr);
                RedrawWindow(Some(hwnd), None, None, RDW_FRAME|RDW_INVALIDATE|RDW_ERASE);

            }

    });

        let mut msg = MSG{..Default::default()};
    loop{
        let lock=Some( HWND((*s_hwnd) as *mut c_void));
        println!("lock");
        let b = GetMessageA(ptr::addr_of_mut!(msg), Some(HWND((*s_hwnd) as *mut c_void)), 0, 0).as_bool();
    // while GetMessageA(ptr::addr_of_mut!(msg), Some( HWND((*s_hwnd.lock().unwrap()) as *mut c_void)), 0, 0).as_bool() 
        if !b{break;}
        let z=TranslateMessage(ptr::addr_of_mut!(msg));
        let r=z.ok();
        if !r.is_ok() && r.as_ref().unwrap_err().code()!=HRESULT(0){
            panic!("{:?}",z);
        }
        DispatchMessageA(ptr::addr_of_mut!(msg));
        drop(lock);
    }


      
    t1.join().unwrap();

    // println!("s");
    //     println!("{:?}", hwnd);
    //     let hdc = unsafe { GetDC(Some(hwnd)) };
    //     println!("{:?}", hdc);
    //     let mut sti = String::from("text");
    //     let sti = sti.as_bytes_mut();
    //     let mut lprc = RECT {
    //         left: -100,
    //         top: -100,
    //         right: 10 + 200,
    //         bottom: 200 + 60,
    //     };
    //     let lprc = ptr::addr_of_mut!(lprc);
    //     // let mut hwnd=0;
    //     // let hwnd:HWND=HWND(&mut hwnd as *mut c_int as *mut c_void);
    //     let hbr = CreateSolidBrush(windows::Win32::Foundation::COLORREF(
    //         255 | (100 << 16) | (20 << 8),
    //     ));

    //     // let hdc = unsafe { GetDC(Some(hwnd)) };
    //     loop {
    //         // DrawTextA(hdc, sti, lprc, DT_CENTER);

    //         FillRect(hdc, lprc, hbr);
    //     }
    }
}
