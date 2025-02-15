use std::time::Duration;

fn main() {
    println!("{:?}",windowpicker::get_hwnd_on_click(true));
    std::thread::sleep(Duration::from_millis(100));
}
