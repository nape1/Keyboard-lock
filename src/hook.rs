use std::sync::atomic::{AtomicBool, Ordering};
use std::ptr;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

static mut HOOK: HHOOK = HHOOK(ptr::null_mut());
static LOCKED: AtomicBool = AtomicBool::new(false);

pub fn set_locked(locked: bool) {
    if locked {
        install_hook();
    } else {
        uninstall_hook();
    }
}

fn install_hook() {
    unsafe {
        if !HOOK.0.is_null() {
            return;
        }
        let h_instance = GetModuleHandleW(None).unwrap();
        HOOK = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_proc),
            h_instance,
            0,
        ).expect("Failed to install hook");
        LOCKED.store(true, Ordering::SeqCst);
    }
}

pub fn uninstall_hook() {
    unsafe {
        if !HOOK.0.is_null() {
            let _ = UnhookWindowsHookEx(HOOK);
            HOOK = HHOOK(ptr::null_mut());
        }
        LOCKED.store(false, Ordering::SeqCst);
    }
}

unsafe extern "system" fn low_level_keyboard_proc(
    code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    unsafe {
        if code >= 0 && LOCKED.load(Ordering::SeqCst) {
            // We block keyboard input by returning 1
            return LRESULT(1);
        }
        CallNextHookEx(HOOK, code, w_param, l_param)
    }
}
