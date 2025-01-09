#![windows_subsystem = "windows"]

use device_query::{DeviceEvents, DeviceState, Keycode};
use tray_item::{IconSource, TrayItem};

#[cfg(target_os = "windows")]
mod platform {
    use windows::Win32::{
        Foundation::{LPARAM, WPARAM},
        UI::{
            Input::Ime::{ImmGetDefaultIMEWnd, IMC_SETCONVERSIONMODE},
            WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL},
        },
    };

    pub fn to_english() {
        let hwnd = unsafe { GetForegroundWindow() };

        if hwnd.0.is_null() {
            return;
        }

        let ime_hwnd = unsafe { ImmGetDefaultIMEWnd(hwnd) };

        if ime_hwnd.0.is_null() {
            return;
        }

        unsafe {
            SendMessageW(
                ime_hwnd,
                WM_IME_CONTROL,
                Some(WPARAM(IMC_SETCONVERSIONMODE as usize)),
                Some(LPARAM(0)),
            );
        }
    }

    pub fn toggle_lang() {} // no need to implement
}

#[cfg(target_os = "macos")]
mod platform {
    use enigo::{
        Direction::{Click, Press, Release},
        Enigo, Key, Keyboard, Settings,
    };

    use core_foundation::{
        array::CFArray,
        base::{CFType, TCFType},
        string::{CFString, CFStringRef},
    };
    use libc::c_void;

    #[link(name = "Carbon", kind = "framework")]
    extern "C" {
        fn TISCreateInputSourceList(
            properties: *const c_void,
            includeAllInstalled: bool,
        ) -> *const c_void;
        static kTISPropertyInputSourceID: *const c_void;
        fn TISGetInputSourceProperty(
            inputSource: *const c_void,
            propertyKey: *const c_void,
        ) -> *const c_void;
        fn TISEnableInputSource(inputSource: *const c_void);
        fn TISSelectInputSource(inputSource: *const c_void);
    }

    pub fn to_english() {
        unsafe {
            let target_id = CFString::new("com.apple.keylayout.ABC");

            let all_inputs_ref = TISCreateInputSourceList(std::ptr::null_mut(), true);

            if all_inputs_ref.is_null() {
                return;
            }

            let all_inputs: CFArray<CFType> =
                CFArray::wrap_under_create_rule(all_inputs_ref as *const _);
            let count = all_inputs.len();

            for i in 0..count {
                if let Some(item_ref) = all_inputs.get(i) {
                    let input_source: *const c_void = item_ref.as_CFTypeRef();

                    if input_source.is_null() {
                        continue;
                    }

                    let prop_ref =
                        TISGetInputSourceProperty(input_source, kTISPropertyInputSourceID);

                    if !prop_ref.is_null() {
                        let source_id = CFString::wrap_under_get_rule(prop_ref as CFStringRef);

                        if source_id == target_id {
                            TISEnableInputSource(input_source);
                            TISSelectInputSource(input_source);
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn toggle_lang() {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        let _ = enigo.key(Key::Control, Press);
        let _ = enigo.key(Key::Space, Click);
        let _ = enigo.key(Key::Control, Release);
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod platform {
    pub fn to_english() {}
    pub fn toggle_lang() {}
}

fn main() {
    #[cfg(target_os = "windows")]
    let mut tray =
        TrayItem::new("vim-esc @luftaquila", IconSource::Resource("tray-default")).unwrap();

    #[cfg(target_os = "macos")]
    let mut tray = TrayItem::new("vim-esc @luftaquila", IconSource::Resource("")).unwrap();

    tray.add_menu_item("vim-esc @luftaquila", || {
        open::that("https://github.com/luftaquila/vim-esc").unwrap();
    })
    .unwrap();

    tray.add_menu_item("Quit", || {
        std::process::exit(0);
    })
    .unwrap();

    let _guard = DeviceState::new().on_key_up(|key| match key {
        Keycode::Escape => platform::to_english(),
        Keycode::Hangul => platform::toggle_lang(),
        _ => (),
    });

    #[cfg(target_os = "macos")]
    tray.inner_mut().display();

    loop {
        std::thread::park();
    }
}
