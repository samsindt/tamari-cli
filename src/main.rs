use std::env;
use std::io::{self, Write};
use clap::{Arg, App, SubCommand};

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
                            .help("Sets the server address")
                            .value_name("address")
                            .takes_value(true))
                        .arg(Arg::with_name("port")
                            .short("p")
                            .help("Sets the server port")
                            .value_name("port")
                            .takes_value(true))
                        .arg(Arg::with_name("password")
                            .short("w")
                            .help("Sets the password to be sent to the server")
                            .value_name("password")
                            .takes_value(true))
                        .arg(Arg::with_name("verbose")
                            .short("v")
                            .multiple(true)
                            .help("Sets output to verbose"))
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

    // check for address, port, and password flags
    // open tcp connection
    // run any subcommands
    //   else run input loop
}

   /* let args: Vec<String> = env::args().collect();

    //process args and flags

    //setup tcp connection







    if 1 < args.len() {
        // run command from args
    } else {
        let stdin = io::stdin();

        loop {
            let mut buffer = String::new();
            print!("tamari> ");
            let _ = io::stdout().flush();

            match stdin.read_line(&mut buffer) {
                Ok(_) => process_line(&buffer),
                Err(_) => panic!("There was a proplem reading from stdin"),
            };
        }
    }

}

fn process_line(line: &String) {
    let mut statement_args: Vec<&str> = line.split_whitespace().collect();
    
    match statement_args.get(0) {
        Some(command) => match &(command.to_lowercase())[..] {
            "del" => {
                statement_args.remove(0);
                handle_delete(&statement_args);
            },
            "set" => println!("set"),
            "get" => println!("get"),
            _ => (),
        },
        
        None => println!("No command"),
    }
}

fn handle_delete (args: &Vec<&str>) {

    if args.len() < 1 {
        println!("Insuficient number of arguments: delete requires one argument");
    } else {
        let key = &args[0];

        let command = format!("-{}\t{}\n", key.len(), key);
        println!("{}", command.escape_debug());

    }
}*/