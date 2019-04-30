use crate::components::connection::handle_connection;
use crate::components::component_creation;
use std::net::TcpListener;
use std::env;

pub fn handle() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 0 {
        start_server(find_port()); // No arguments start using the default procedure
    } else {
        for arg in args {
            if arg.eq("list") {
                println!("Commands:");
                println!("  add");
                println!("  help");
                println!("  list");
                println!("  port=#");
            }
            if arg.starts_with("port=") {
                let port: u32 = arg[5..arg.len()].to_owned().parse().unwrap(); // This line needs better error checking and limiting
                start_server(predefined_port(port));
            }
            if arg.starts_with("help") {
                println!("add: This parameter starts the process of adding a new component to the server.");
                println!("help: This parameter list all of the commands and their descriptions.");
                println!("list: This parameter lists all of the available commands.");
                println!("port: This parameter starts the server on a specific port. Example input 'cargo run port=4321'");
            }
            if arg.starts_with("add") {
                component_creation::initialize();
            }
        }
        // Commands: help, list, port, add application thingy, help {command}
    }
}

pub fn find_port() -> (u32, TcpListener) {
    let mut port = 1234;
    let mut listen = TcpListener::bind(format!("127.0.0.1:{}", port));
    while listen.is_err() {
        println!(
            "port: {} is already in use, trying port: {}",
            port,
            port + 1
        );
        port = port + 1;
        listen = TcpListener::bind(format!("127.0.0.1:{}", port));
    }
    return (port, listen.unwrap());
}

pub fn predefined_port(port: u32) -> (u32, TcpListener) {
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(listener) => listener,
        Err(_e) => panic!("The input port is already in use. Consider launching again without a set port to to be automatically assigned an open port.")
    };
    return (port, listener);
}

pub fn start_server(args: (u32, TcpListener)) {
    println!("Server listening on port: {}", args.0);
    for stream in args.1.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
