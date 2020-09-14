#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use backend::db::{ establish_connection, query_manga };

use mangadex_reader::{ JsonApiResponse, Manga, MangaJsonWrapper };

use rocket_contrib::json::Json;

// Route handler returns payload containing manga listings
#[get("/mangas")]
fn mangas_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    let conn = establish_connection();
    for db_manga in query_manga(&conn) {
        // Convert database manga model response to api version of manga struct
        let api_manga = Manga {
            id: db_manga.id,
            title: db_manga.title,
            authors: db_manga.authors,
            artists: db_manga.artists,
            genre_ids: db_manga.genre_ids,
            genre_names: db_manga.genre_names,
            url_link: db_manga.url_link,
        };

        // Toss queried manga item into wrapper struct that conforms to json api spec.
        let wrapped_manga = MangaJsonWrapper {
            _type: "mangas".to_string(),
            id: api_manga.id.to_string(),
            attributes: api_manga,
        };

        response.data.push(wrapped_manga)
    }

    Json(response)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![mangas_get])
        .launch();
}