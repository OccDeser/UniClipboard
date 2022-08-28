use super::message;
use arboard::Clipboard;

pub fn get() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    let res = clipboard.get_text();
    match res {
        Ok(s) => s,
        Err(error) => {
            message::error(format!("{}", error));
            "".to_string()
        }
    }
}

pub fn set(s: String) {
    let mut clipboard = Clipboard::new().unwrap();
    let res =  clipboard.set_text(s);
    match res {
        Ok(_) => (),
        Err(error) => {
            message::error(format!("{}", error));
        }
    }
}
