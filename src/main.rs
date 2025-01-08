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

    pub fn to_english() {
        println!("Esc pressed! Switching IME to English...");
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
