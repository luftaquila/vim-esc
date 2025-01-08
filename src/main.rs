// #![windows_subsystem = "windows"]

use device_query::{DeviceEvents, DeviceState, Keycode};
use tray_item::{IconSource, TrayItem};

#[cfg(target_os = "windows")]
mod platform {
    pub fn to_english() {
        println!("Esc pressed! Switching IME to English...");
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
    #[cfg(target_os = "windows")]
    let mut tray = TrayItem::new("vimESC @luftaquila", IconSource::Resource("tray-default")).unwrap();

    #[cfg(target_os = "macos")]
    let mut tray = TrayItem::new("vimESC @luftaquila", IconSource::Resource("")).unwrap();

    tray.add_menu_item("vimESC @luftaquila", || { open::that("https://github.com/luftaquila/vim-esc").unwrap(); }).unwrap();
    tray.add_menu_item("Quit", || { std::process::exit(0); }).unwrap();

    let _guard = DeviceState::new().on_key_up(|key| match key {
        Keycode::Escape => platform::to_english(),
        Keycode::Kana => platform::toggle_lang(),
        _ => (),
    });

    #[cfg(target_os = "macos")]
    tray.inner_mut().display();

    loop {
        std::thread::park();
    }
}
