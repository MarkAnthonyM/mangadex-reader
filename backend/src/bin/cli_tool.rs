use std::env;
use backend::db::{ establish_connection, query_manga };

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
    
    let _conn = establish_connection();
    // Temporary disable.
    // TODO: Reimplement associated method to create mock data
    // create_manga(&conn, &args[0]);
}

fn show_manga(args: &[String]) {
    if args.len() > 0 {
        println!("show: Unexpected argument");
        help();
        return;
    }
    
    let conn = establish_connection();
    println!("Mangas\n------");
    for manga in query_manga(&conn) {
        println!("Manga ID: {}\nManga Name: {}\n", manga.id, manga.title);
    }
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
        "show" => show_manga(&args[2..]),
        _ => help(),
    }
}