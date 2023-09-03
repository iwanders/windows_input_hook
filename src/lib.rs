use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, PostThreadMessageA, SetWindowsHookExA, UnhookWindowsHookEx, HHOOK,
    KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

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
    if code < 0 {
        return CallNextHookEx(HHOOK(0), code, wparam, lparam);
    }

    // let current_id = GetCurrentThreadId();
    // println!("current id; {current_id}");
    let z = std::mem::transmute::<_, *const KBDLLHOOKSTRUCT>(lparam);
    // println!("z.vkCode: {}", (*z).vkCode);
    // println!("z.scanCode: {}", (*z).scanCode);
    // println!("z.flags: 0x{:x?}", (*z).flags);
    // println!("time: {:?}", (*z).time);

    let action = match wparam.0 as u32 {
        WM_KEYDOWN => KeyAction::Down,
        WM_KEYUP => KeyAction::Up,
        WM_SYSKEYDOWN => KeyAction::Down,
        WM_SYSKEYUP => KeyAction::Up,
        _ => {
            panic!("unsupported key action {}", wparam.0);
        }
    };

    let input = KeyInput {
        action,
        code: (*z).vkCode as u8,
    };
    InputHook::MAP.with(|z| {
        let mut l = z.borrow_mut();
        if let Some(f) = l.get_mut(&input) {
            f(input);
        }
    });

    return CallNextHookEx(HHOOK(0), code, wparam, lparam);
}

pub unsafe fn dump_keys() {
    for i in 0..256 {
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

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum KeyAction {
    Up,
    Down,
}
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct KeyInput {
    action: KeyAction,
    code: u8,
}
impl KeyInput {
    pub fn down(code: u8) -> Self {
        KeyInput {
            action: KeyAction::Down,
            code,
        }
    }
    pub fn up(code: u8) -> Self {
        KeyInput {
            action: KeyAction::Up,
            code,
        }
    }
}

type HookFunction = dyn FnMut(KeyInput) + Send;
type HookHandler = Box<HookFunction>;
type HookMap = std::collections::HashMap<KeyInput, HookHandler>;

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, AtomicU32};
pub struct InputHook {
    thread: Option<std::thread::JoinHandle<()>>,
    thread_id: std::sync::Arc<AtomicU32>,
    running: std::sync::Arc<AtomicBool>,
}
impl InputHook {
    thread_local! {
        static MAP: RefCell<HookMap> = RefCell::new(Default::default());
    }

    pub fn new(map: HookMap) -> Self {
        let thread_id = std::sync::Arc::new(AtomicU32::new(0));
        let running = std::sync::Arc::new(AtomicBool::new(true));

        let tid = thread_id.clone();
        let t_running = running.clone();
        let thread = Some(std::thread::spawn(move || {
            unsafe {
                // Store the hook map;
                InputHook::MAP.with(|z| {
                    let mut l = z.borrow_mut();
                    *l = map;
                });

                // Store the thread id such that we can later exit this thread by sending a message.
                let current_id = GetCurrentThreadId();
                tid.store(current_id, std::sync::atomic::Ordering::Relaxed);

                // Set the hook.
                let hh = SetWindowsHookExA(WH_KEYBOARD_LL, Some(foo), HINSTANCE(0), 0)
                    .expect("hook did not succeed");

                // https://stackoverflow.com/a/65571485
                // This hook is called in the context of the thread that installed it. The call is made by sending a message to the thread that installed the hook. Therefore, the thread that installed the hook must have a message loop, which we place here.
                // I think other systems could send messages, to lets wrap it into a loop with a mutex to quit if we
                // really want to.
                while t_running.load(std::sync::atomic::Ordering::Relaxed) {
                    let mut message: MSG = std::mem::zeroed();
                    GetMessageW(&mut message, HWND(0), 0, 0);
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }

                // Unhook the things.
                UnhookWindowsHookEx(hh).expect("unhook did not succeed");
            }
        }));
        InputHook {
            thread,
            thread_id,
            running,
        }
    }
}
impl Drop for InputHook {
    fn drop(&mut self) {
        // Set the boolean to stop running.
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);

        unsafe {
            // Send a message to the thread to drop out of the GetMessage loop.
            let tid = self.thread_id.load(std::sync::atomic::Ordering::Relaxed);
            PostThreadMessageA(tid, 0, WPARAM(0), LPARAM(0)).expect("other thread must be running");
        }

        // Finally, join the thread.
        let t = self.thread.take();
        t.unwrap().join().expect("join should succeed");
    }
}

pub fn main() {
    let mut map: HookMap = std::collections::HashMap::new();

    map.insert(
        KeyInput::down(65),
        Box::new(|v: KeyInput| {
            println!("A: {v:?}");
        }),
    );
    let _h = InputHook::new(map);
    for _i in 0..50 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
