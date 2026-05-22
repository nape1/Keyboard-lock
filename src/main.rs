#![windows_subsystem = "windows"]

mod hook;

use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::Graphics::Gdi::*,
};

static mut BUTTON_HWND: HWND = HWND(std::ptr::null_mut());
static mut IS_LOCKED: bool = false;

fn main() -> Result<()> {
    unsafe {
        let instance: HINSTANCE = GetModuleHandleW(None)?.into();
        let window_class = w!("KeyboardLockClass");

        // Load the icon from the embedded resource (MAINICON is usually 1)
        let icon = LoadIconW(instance, PCWSTR(1 as *const u16)).ok();

        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: instance,
            lpszClassName: window_class,
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: HBRUSH(COLOR_WINDOW.0 as *mut _),
            hIcon: icon.unwrap_or(HICON(std::ptr::null_mut())),
            ..Default::default()
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST,
            window_class,
            w!("Keyboard Lock"),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU,
            CW_USEDEFAULT, CW_USEDEFAULT, 220, 100,
            None, None, instance, None,
        )?;

        BUTTON_HWND = CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("BUTTON"),
            w!("LOCK Keyboard"),
            WS_TABSTOP | WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
            10, 10, 185, 40,
            hwnd, HMENU(1 as *mut _), instance, None,
        )?;

        let _ = ShowWindow(hwnd, SW_SHOW);

        let mut message = MSG::default();
        while GetMessageW(&mut message, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }

        Ok(())
    }
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_COMMAND => {
                if wparam.0 == 1 {
                    IS_LOCKED = !IS_LOCKED;
                    hook::set_locked(IS_LOCKED);
                    
                    let text = if IS_LOCKED {
                        w!("UNLOCK Keyboard")
                    } else {
                        w!("LOCK Keyboard")
                    };
                    
                    let _ = SetWindowTextW(BUTTON_HWND, text);
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                hook::uninstall_hook();
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
