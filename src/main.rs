use std::env;
use clap::{Arg, App, SubCommand};
use std::process;
use std::str;
use std::io;
use std::io::Write;

use tamari;

fn main() {
    let matches = App::new("tamari-cli")
                        .version("1.0")
                        .author("Sam Sindt")
                        .about("A CLI client for TamariDB")
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Sets a custom config file")
                            .takes_value(true))
                        .arg(Arg::with_name("address")
                            .short("a")
                            .long("address")
                            .help("Sets the server address")
                            .value_name("address")
                            .takes_value(true))
                        .arg(Arg::with_name("port")
                            .short("p")
                            .long("port")
                            .help("Sets the server port")
                            .value_name("port")
                            .takes_value(true))
                        .arg(Arg::with_name("password")
                            .short("w")
                            .long("password")
                            .help("Sets the password to be sent to the server")
                            .value_name("password")
                            .takes_value(true))
                        .arg(Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .multiple(true)
                            .help("Sets output to verbose"))
                        .arg(Arg::with_name("debug")
                            .short("d")
                            .long("debug")
                            .multiple(true)
                            .help("Replaces TCP connection with connection that displays request in protocol format"))
                        .subcommand(SubCommand::with_name("set")
                                    .about("sets the value at the key")
                                    .arg(Arg::with_name("key")
                                        .required(true)
                                        .help("the key to set the value at")
                                        .index(1))
                                    .arg(Arg::with_name("value")
                                        .required(true)
                                        .help("the value to set at the key")
                                    ))
                        .subcommand(SubCommand::with_name("get")
                                    .about("gets the value at the key")
                                    .arg(Arg::with_name("key")
                                        .required(true)
                                        .help("the key to get the value at")
                                    ))
                        .subcommand(SubCommand::with_name("del")
                                    .about("deletes the value at the key")
                                    .arg(Arg::with_name("key")
                                        .required(true)
                                        .help("the key to delete")))       
                        .get_matches();

    // check for verbose flag
    let verbose = matches.is_present("verbose");

    let debug = matches.is_present("debug");

    // check for address
    let address = match matches.value_of("address") {
        Some(adr) => String::from(adr),
        None => match env::var("TAMARI_CLI_ADDRESS") {
            Ok(adr) => adr,
            Err(_) => String::from("127.0.0.1"),
        },
    };

    // check for port
    let port_str = match matches.value_of("port") {
        Some(pt) => String::from(pt),
        None => match env::var("TAMARI_CLI_PORT") {
            Ok(pt) => pt,
            Err(_) => String::from("8080"),
        },
    };

    let port: u16;
    if let Ok(pt) = port_str.parse::<u16>() {
        port = pt;
    } else {
        // standardize this level of error handling
        eprintln!("Invalid port");
        process::exit(-1);
    };

    // check for password 
    /*let password = match matches.value_of("password") {
        Some(pw) => String::from(pw),
        None => match env::var("TAMARI_CLI_PASSWORD") {
            Ok(pw) => pw,
            Err(_) => String::new(),
        },
    };*/

    if verbose && !debug {
        println!("Connecting to server at {}:{} ...", address, port);
    } else if verbose {
        println!("Setting up debug connection ...");
    }

    let connection: Box<dyn tamari::Connection>;

    if !debug {
        match tamari::TcpConnection::new(&address, port) {
            Ok(c) => connection = Box::new(c),
            Err(e) => {
                eprintln!("Failed to connect to server with error: {}", e);
                process::exit(-1);
            }
        }
    } else {
        connection = Box::new(DebugConnection{});
    }

    

    let mut client = tamari::Client::new(connection);

    if let Some(get_matches) = matches.subcommand_matches("get") {
        let key = get_matches.value_of("key").unwrap();
        match client.get(key) {
            Ok(res) => println!("{}", res),
            Err(e) => {
                eprintln!("Get request failed with error: {}", e);
                process::exit(-1);
            }
        }
    }

   if let Some(set_matches) = matches.subcommand_matches("set") {
        let key = set_matches.value_of("key").unwrap();
        let value = set_matches.value_of("value").unwrap();
        match client.set(key, value) {
            Ok(res) => println!("{}", res),
            Err(e) => {
                eprintln!("Set request failed with error: {}", e);
                process::exit(-1);
            }
        }
    }

    if let Some(del_matches) = matches.subcommand_matches("del") {
        let key = del_matches.value_of("key").unwrap();
        match client.delete(key) {
            Ok(res) =>println!("{}", res),
            Err(e) => {
                eprintln!("Delete request failed with error: {}", e);
                process::exit(-1);
            }
        }
    }

    if let None = matches.subcommand_name() {
        let stdin = io::stdin();

        loop {
            let mut buffer = String::new();
            print!("tamari> ");
            let _ = io::stdout().flush();

            match stdin.read_line(&mut buffer) {
                Ok(_) => process_line(&buffer, &mut client),
                Err(_) => panic!("There was a proplem reading from stdin"),
            };
        }
    }
}

fn process_line(line: &String, client: &mut tamari::Client) {
    let mut statement_args: Vec<&str> = line.split_whitespace().collect();
    
    match statement_args.get(0) {
        Some(command) => match &(command.to_lowercase())[..] {
            "del" => {
                statement_args.remove(0);
                if 1 > statement_args.len() {
                    eprintln!("Insufficient number of arguments: delete requires one argument");
                    return
                }

                match client.delete(statement_args[0]) {
                    Ok(res) => println!("{}", res),
                    Err(e) => {
                        eprintln!("Delete request failed with error: {}", e);
                        process::exit(-1);
                    }
                }
            },
            "set" => {
                statement_args.remove(0);
                if 2 > statement_args.len() {
                    eprintln!("Insufficient number of arguments: set requires one argument");
                    return
                }

                match client.set(statement_args[0], statement_args[1]) {
                    Ok(res) => println!("{}", res),
                    Err(e) => {
                        eprintln!("Set request failed with error: {}", e);
                        process::exit(-1);
                    }
                }
            },
            "get" => {
                statement_args.remove(0);
                if 1 > statement_args.len() {
                    eprintln!("Insufficient number of arguments: get requires one argument");
                    return
                }

                match client.get(statement_args[0]) {
                    Ok(res) => println!("{}", res),
                    Err(e) => {
                        eprintln!("Get request failed with error: {}", e);
                        return
                    }
                }
            },
            _ => (),
        },
        
        None => (),
    }
}

struct DebugConnection {}

impl tamari::Connection for DebugConnection {
    fn read(&mut self) -> Result<Vec<u8>, tamari::TamariError> {
        let s = String::from("$\n");
        Ok(s.into_bytes())
    }

    fn write(&mut self, buffer: &[u8]) -> Result<(), tamari::TamariError> {
        let s = String::from(str::from_utf8(buffer).unwrap());
        println!("Debug request: {}", s.escape_debug());
        Ok(())
    }
}