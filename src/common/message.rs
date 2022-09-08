use term_painter::{Color, ToStyle};

pub fn welcome() {
    let s0 = "             _   ___ _ _       _                         _";
    let s1 = " /\\ /\\ _ __ (_) / __\\ (_)_ __ | |__   ___   __ _ _ __ __| |";
    let s2 = "/ / \\ \\ '_ \\| |/ /  | | | '_ \\| '_ \\ / _ \\ / _` | '__/ _` |";
    let s3 = "\\ \\_/ / | | | / /___| | | |_) | |_) | (_) | (_| | | | (_| |";
    let s4 = " \\___/|_| |_|_\\____/|_|_| .__/|_.__/ \\___/ \\__,_|_|  \\__,_|";
    let s5 = "                        |_|                                    ";
    println!("{}", s0);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s4);
    println!("{}", s5);
}

pub fn success(key: String, msg: String) {
    println!("{}: {}", Color::Green.bold().paint(key), msg);    
}

pub fn warning(msg: String) {
    println!("{}: {}", Color::Yellow.bold().paint("warning"), msg);
}

pub fn error(msg: String) {
    println!("{}: {}", Color::Red.bold().paint("error"), msg);
}

pub fn info(msg: String) {
    println!("{}", msg);
}
