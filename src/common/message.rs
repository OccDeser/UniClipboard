extern crate colorful;

use colorful::Colorful;
use colorful::HSL;

pub fn welcome() {
    let s0 = "             _   ___ _ _       _                         _";
    let s1 = " /\\ /\\ _ __ (_) / __\\ (_)_ __ | |__   ___   __ _ _ __ __| |";
    let s2 = "/ / \\ \\ '_ \\| |/ /  | | | '_ \\| '_ \\ / _ \\ / _` | '__/ _` |";
    let s3 = "\\ \\_/ / | | | / /___| | | |_) | |_) | (_) | (_| | | | (_| |";
    let s4 = " \\___/|_| |_|_\\____/|_|_| .__/|_.__/ \\___/ \\__,_|_|  \\__,_|";
    let s5 = "                        |_|                                    ";
    println!("{}", s0.gradient_with_color(HSL::new(0.0, 1.0, 0.5), HSL::new(0.833, 1.0, 0.5)));
    println!("{}", s1.gradient_with_color(HSL::new(0.0, 1.0, 0.5), HSL::new(0.833, 1.0, 0.5)));
    println!("{}", s2.gradient_with_color(HSL::new(0.0, 1.0, 0.5), HSL::new(0.833, 1.0, 0.5)));
    println!("{}", s3.gradient_with_color(HSL::new(0.0, 1.0, 0.5), HSL::new(0.833, 1.0, 0.5)));
    println!("{}", s4.gradient_with_color(HSL::new(0.0, 1.0, 0.5), HSL::new(0.833, 1.0, 0.5)));
    println!("{}", s5.gradient_with_color(HSL::new(0.0, 1.0, 0.5), HSL::new(0.833, 1.0, 0.5)));}

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
