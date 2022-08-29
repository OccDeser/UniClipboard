mod common;
mod datatype;

use clap::Parser;
use common::clipboard;
use common::message;

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

    let mut peers = Vec::new();
    if args.remote.ne("nopeer") {
        let words: Vec<&str> = args.remote.split(":").collect();
        if words.len() > 2 {
            message::error("invalid remote host".to_string());
            std::process::exit(-1);
        } else {
            if words.len() == 2 {
                let host = words[0].to_string();
                let port = words[1].parse::<u16>().unwrap();
                peers.push(datatype::RemoteClipboard { host, port });
            } else {
                message::warning("The remote port is not set, use the local port.".to_string());
                let host = words[0].to_string();
                let port = args.port;
                peers.push(datatype::RemoteClipboard { host, port });
            }
        }
    }

    datatype::LocalClipboard {
        port: args.port,
        password: args.password,
        peers: peers,
    }
}

fn main() {
    let args = Args::parse();
    let local_clipboard = init_local_clipboard(args);
    println!("port: {}", local_clipboard.port);
    println!("password: {}", local_clipboard.password);
    println!("peers: {}", local_clipboard.peers.len());
    if local_clipboard.peers.len()>0{
        println!("peer_host: {}", local_clipboard.peers[0].host);
        println!("peer_port: {}", local_clipboard.peers[0].port);
    }

    message::welcome();

    message::success("Finished".to_string(), "success".to_string());
}
