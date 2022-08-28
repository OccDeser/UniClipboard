extern crate colorful;

// use colorful::Color;
use colorful::Colorful;

pub fn success(key: String, msg: String) {
    // let key = format!("{: <7}", key);
    println!("{}: {}", key.green().bold(), msg);
}

pub fn warning(msg: String) {
    println!("{}: {}", "warning".yellow().bold(), msg);
}

pub fn error(msg: String) {
    println!("{}: {}", "error".red().bold(), msg);
}
