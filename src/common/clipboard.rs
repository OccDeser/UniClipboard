use arboard::Clipboard;

pub fn get() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.get_text().unwrap()
}

pub fn set(s: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(s).unwrap();
}
