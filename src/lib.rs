// use windows_sys;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, SetWindowsHookExA, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL,
    WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
    HHOOK,
};
// use windows::Win32::System::LibraryLoader::LoadLibraryA;
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyNameTextA;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM, HINSTANCE, HWND};

/// code: A code the hook procedure uses to determine how to process the message. If nCode is less than zero, the hook procedure must pass the message to the CallNextHookEx function without further processing and should return the value returned by CallNextHookEx. This parameter can be one of the following values.
/// Value	Meaning
/// HC_ACTION 0
/// The wParam and lParam parameters contain information about a keyboard message.
/// wparam: The identifier of the keyboard message. This parameter can be one of the following messages: WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, or WM_SYSKEYUP.
/// lparam: Type: LPARAM
/// A pointer to a KBDLLHOOKSTRUCT structure.
unsafe extern "system" fn foo(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    println!("things!: {code:?}");
    if code < 0 {
        return CallNextHookEx(HHOOK(0), code, wparam, lparam);
    }

    let z = std::mem::transmute::<_, *const KBDLLHOOKSTRUCT>(lparam);
    println!("z.vkCode: {}", (*z).vkCode);
    println!("z.scanCode: {}", (*z).scanCode);
    println!("z.flags: 0x{:x?}", (*z).flags);
    println!("time: {:?}", (*z).time);

    match wparam.0 as u32  {
        WM_KEYDOWN => {
            println!("down");
        }
        WM_KEYUP => {
            println!("up");
        }
        WM_SYSKEYDOWN => {
            println!("sys down");
        }
        WM_SYSKEYUP => {
            println!("sys up");
        }
        _ => {}
    }
    return CallNextHookEx(HHOOK(0), code, wparam, lparam);
}

pub fn main() {
    unsafe {
        // let h = LoadLibraryA("user32.dll");
        // let h = LoadLibraryA("kernel32.dll");
        // println!("h: 0x{h:x?}");
        println!("WH_KEYBOARD_LL: 0x{WH_KEYBOARD_LL:x?}");

        let hh = SetWindowsHookExA(WH_KEYBOARD_LL, Some(foo), HINSTANCE(0), 0);
        println!("hh: 0x{hh:x?}");

        let mut v: Vec<u8> = vec![0;64];
        GetKeyNameTextA(1, &mut v);
        println!("v: {v:?}");

        // https://stackoverflow.com/a/65571485
        // This hook is called in the context of the thread that installed it. The call is made by sending a message to the thread that installed the hook. Therefore, the thread that installed the hook must have a message loop.
        let mut message: MSG = std::mem::zeroed();
        GetMessageW(&mut message, HWND(0), 0, 0);
    }
}
