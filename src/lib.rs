
// use windows_sys;
use windows::Win32::UI::WindowsAndMessaging::{SetWindowsHookExA, CallNextHookEx, WH_KEYBOARD_LL, HOOKPROC, GetMessageW, MSG};
use windows::Win32::System::LibraryLoader::LoadLibraryA;
use windows::Win32::Foundation::{WPARAM, LPARAM, LRESULT};

unsafe extern "system" fn foo(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    println!("things!: {code:?}");
    return CallNextHookEx(0, code, wparam, lparam);
    0
}

pub fn main() {

    unsafe {
        // let h = LoadLibraryA("user32.dll");
        let h = LoadLibraryA("kernel32.dll");
        println!("h: 0x{h:x?}");
        println!("WH_KEYBOARD_LL: 0x{WH_KEYBOARD_LL:x?}");

        let hh = SetWindowsHookExA(WH_KEYBOARD_LL, Some(foo), h, 0);
        println!("hh: 0x{hh:x?}");

        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            // let mut message: MSG = mem::zeroed();
            let mut message: MSG = std::mem::zeroed();
            GetMessageW(&mut message, 0, 0, 0);

        }
    }
    println!("slfjdslkfj");
}