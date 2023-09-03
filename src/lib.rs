// use windows_sys;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, PostThreadMessageA, SetWindowsHookExA, HHOOK, KBDLLHOOKSTRUCT,
    MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};
// use windows::Win32::System::LibraryLoader::LoadLibraryA;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyNameTextA, MapVirtualKeyA, MAPVK_VK_TO_VSC_EX,
};

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

    match wparam.0 as u32 {
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

pub unsafe fn dump_keys() {
    for i in 0..256 {
        //
        let mut v = String::new();
        for _ in 0..64 {
            v.insert_str(0, " ");
        }
        let scancode = MapVirtualKeyA(i, MAPVK_VK_TO_VSC_EX) as i32;
        let ret = GetKeyNameTextA(scancode << 16, &mut v.as_mut_vec());
        let v = &v[..ret as usize];
        println!("{i} reg: {ret} v: {v:?}");
    }
}

pub enum KeyAction {
    Up,
    Down,
}
pub struct KeyInput {
    action: KeyAction,
    code: u8,
}

type HookHandler = Box<dyn Fn(u8, u8)>;
type HookMap = std::collections::HashMap<KeyInput, HookHandler>;

use std::cell::RefCell;
use std::sync::atomic::AtomicU32;
pub struct InputHook {
    thread: Option<std::thread::JoinHandle<()>>,
    thread_id: std::sync::Arc<AtomicU32>,
}
impl InputHook {
    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(0);
    }

    pub fn new(map: HookMap) -> Self {
        let thread_id = std::sync::Arc::new(AtomicU32::new(0));
        let tid = thread_id.clone();
        let thread = Some(std::thread::spawn(move || {
            unsafe {
                tid.store(GetCurrentThreadId(), std::sync::atomic::Ordering::Relaxed);

                let hh = SetWindowsHookExA(WH_KEYBOARD_LL, Some(foo), HINSTANCE(0), 0);
                println!("hh: 0x{hh:x?}");

                // https://stackoverflow.com/a/65571485
                // This hook is called in the context of the thread that installed it. The call is made by sending a message to the thread that installed the hook. Therefore, the thread that installed the hook must have a message loop.
                let mut message: MSG = std::mem::zeroed();
                GetMessageW(&mut message, HWND(0), 0, 0);
                println!("Shutting down");
            }
        }));
        InputHook { thread, thread_id }
    }
}
impl Drop for InputHook {
    fn drop(&mut self) {
        // First, let the message drop out of the GetMessage block by sending it a  message.
        unsafe {
            let tid = self.thread_id.load(std::sync::atomic::Ordering::Relaxed);
            PostThreadMessageA(tid, 0, WPARAM(0), LPARAM(0)).expect("other thread must be running");
        }
        let t = self.thread.take();
        t.unwrap().join().expect("join should succeed");
    }
}

pub fn main() {
    let hh = InputHook::new(Default::default());
    for i in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
