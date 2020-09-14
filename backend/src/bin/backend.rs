#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use backend::db::{ establish_connection, query_manga };

#[get("/mangas")]
fn mangas_get() -> String {
    "This is a response\n".into()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![mangas_get])
        .launch();
}