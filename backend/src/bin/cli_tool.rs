use std::env;
use backend::db::{ create_manga, establish_connection };

fn help() {
    println!("subcommand:");
    println!("    new<manga>: create new manga listing");
}

fn new_manga(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <manga>");
        help();
        return;
    }
    
    let conn = establish_connection();
    create_manga(&conn, &args[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_manga(&args[2..]),
        _ => help(),
    }
}