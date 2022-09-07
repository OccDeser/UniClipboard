mod common;
mod datatype;

use clap::Parser;
use common::hotkey::Keycode;
use common::{message, uniclip};
use datatype::RemoteClipboard;

#[derive(Parser, Debug)]
#[clap(author, version, about = None, long_about = None)]
struct Args {
    /// Listen port
    #[clap(short = 'p', long, value_parser, default_value_t = 10500)]
    port: u16,

    /// Password
    #[clap(short='P', long, value_parser, default_value_t = String::from("nopassword"))]
    password: String,

    /// Remote host
    #[clap(short, long, value_parser, default_value_t = String::from("nopeer"))]
    remote: String,
}

fn init_local_clipboard(args: Args) -> datatype::LocalClipboard {
    if args.password == String::from("nopassword") {
        message::warning("Use the default password, which may be a security risk.".to_string());
    }

    let mut peer = datatype::RemoteClipboard {
        host: "".to_string(),
        port: 0,
    };
    if args.remote.ne("nopeer") {
        let words: Vec<&str> = args.remote.split(":").collect();
        if words.len() > 2 {
            message::error(format!("Invalid remote host \"{}\"", args.remote));
            std::process::exit(-1);
        } else {
            if words.len() == 2 {
                peer.host = words[0].to_string();
                peer.port = match words[1].parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => {
                        message::error(format!("Invalid remote port \"{}\"", words[1]));
                        std::process::exit(-1);
                    }
                };
            } else {
                message::warning("The remote port is not set, use the local port.".to_string());
                peer.host = words[0].to_string();
                peer.port = args.port;
            }
        }
    }

    datatype::LocalClipboard {
        port: args.port,
        password: args.password,
        peer: RemoteClipboard {
            host: peer.host,
            port: peer.port,
        },
    }
}

fn main() {
    let args = Args::parse();
    let local_clipboard = init_local_clipboard(args);
    println!("port: {}", local_clipboard.port);
    println!("password: {}", local_clipboard.password);
    println!("peer_host: {}", local_clipboard.peer.host);
    println!("peer_port: {}", local_clipboard.peer.port);

    uniclip::init();
    let mut uniclip = uniclip::Uniclip::new(
        &local_clipboard,
        vec![Keycode::LControl, Keycode::LShift, Keycode::C],
    );
    uniclip.start();

    message::welcome();

    // message::success("Finished".to_string(), "success".to_string());
    loop {}
}
