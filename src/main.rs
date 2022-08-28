mod common;

use common::clipboard;
use common::message;

fn main() {
    println!("get clipboard: {}", clipboard::get());
    let s = "123 Test";
    clipboard::set(s.to_string());
    println!("set clipboard: {}", s);

    message::success("Finished".to_string(), "success".to_string());
}
