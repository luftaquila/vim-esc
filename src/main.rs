use device_query::{DeviceEvents, DeviceState, Keycode};
use tray_item::{IconSource, TrayItem};

#[cfg(target_os = "windows")]
mod platform {
    use windows::{
        core::PCWSTR,
        Win32::Globalization::{LoadKeyboardLayoutW, KLF_ACTIVATE},
        Win32::UI::Input::KeyboardAndMouse::ActivateKeyboardLayout,
    };

    pub fn to_english() {
        println!("Esc pressed! Switching IME to English...");

        // 0x0409 = English (United States) Layout
        const ENGLISH_US: &str = "00000409";

        unsafe {
            let wide_layout: Vec<u16> = ENGLISH_US
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

            let hkl = LoadKeyboardLayoutW(PCWSTR(wide_layout.as_ptr()), KLF_ACTIVATE);
            if !hkl.is_invalid() {
                ActivateKeyboardLayout(hkl, KLF_ACTIVATE);
            }
        }
    }

    pub fn toggle_lang() {
        println!("lang key pressed! Toggling IME...");
    }
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
    pub fn to_english() {
        println!("Not implemented for this platform.");
    }

    pub fn toggle_lang() {
        println!("lang key pressed! Toggling IME...");
    }
}

fn main() {
    let _guard = DeviceState::new().on_key_up(|key| match key {
        Keycode::Escape => platform::to_english(),
        Keycode::Kana => platform::toggle_lang(),
        _ => (),
    });

    #[cfg(target_os = "windows")]
    let mut tray =
        TrayItem::new("vimESC @luftaquila", IconSource::Resource("tray-default")).unwrap();

    #[cfg(target_os = "macos")]
    let mut tray = TrayItem::new("vimESC @luftaquila", IconSource::Resource("")).unwrap();

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();

    loop {}
}
