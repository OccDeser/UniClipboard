mod common;

use common::clipboard;

fn main() {
	println!("get clipboard: {}", clipboard::get());
	let s = "123 Test";
    clipboard::set(s.to_string());
	println!("set clipboard: {}", s);
}