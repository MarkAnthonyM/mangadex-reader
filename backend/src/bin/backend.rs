#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use backend::api;
// use backend::db::query_manga;

use mangadex_reader::{ JsonApiResponse, Manga, MangaJsonWrapper };

use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error };

// Rocket connection pool
#[database("postgres_mangadex")]
struct MangadexDbConn(diesel::PgConnection);

// TODO: Rework backend postgres db to accept new manga struct version
// Route handler returns payload containing manga listings
// #[get("/mangas")]
// fn mangas_get(conn: MangadexDbConn) -> Json<JsonApiResponse<Manga>> {
//     let mut response = JsonApiResponse { data: vec![] };

//     for db_manga in query_manga(&conn) {
//         // Convert database manga model response to api version of manga struct
//         let api_manga = Manga {
//             id: db_manga.id,
//             title: db_manga.title,
//             authors: db_manga.authors,
//             artists: db_manga.artists,
//             genre_ids: db_manga.genre_ids,
//             genre_names: db_manga.genre_names,
//             url_link: db_manga.url_link,
//         };

//         // Toss queried manga item into wrapper struct that conforms to json api spec.
//         let wrapped_manga = MangaJsonWrapper {
//             _type: "mangas".to_string(),
//             id: api_manga.id.to_string(),
//             attributes: api_manga,
//         };

//         response.data.push(wrapped_manga)
//     }

//     Json(response)
// }

#[get("/testfront/<id>")]
fn front_test(id: String) -> Json<JsonApiResponse<Manga>> {
    let mut response = JsonApiResponse { data: vec![] };
    let manga_result = api::manga::Manga::populate(&id);
    let data = match manga_result {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error with request: {:?}", e);
            None
        },
    };

    let fetched_manga = data.unwrap();

    let api_manga = Manga {
        alt_names: fetched_manga.manga.alt_names,
        artists: Some(vec![fetched_manga.manga.artist]),
        authors: Some(vec![fetched_manga.manga.author]),
        comments: fetched_manga.manga.comments,
        cover_url: fetched_manga.manga.cover_url,
        covers: fetched_manga.manga.covers,
        demographic: fetched_manga.manga.demographic,
        description: fetched_manga.manga.description,
        follows: fetched_manga.manga.follows,
        genres: fetched_manga.manga.genres,
        hentai: fetched_manga.manga.hentai,
        lang_flag: fetched_manga.manga.lang_flag,
        lang_name: fetched_manga.manga.lang_name,
        status: fetched_manga.manga.status,
        title: fetched_manga.manga.title,
        views: fetched_manga.manga.views,
    };

    let wrapped_manga = MangaJsonWrapper {
        _type: "mangas".to_string(),
        id,
        attributes: api_manga,
    };

    response.data.push(wrapped_manga);

    Json(response)
}

#[get("/testdexapi/<id>")]
fn dex_test(id: String) -> Json<api::manga::Manga> {
    let response = api::manga::Manga::populate(&id);
    let data = match response {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error with response: {:?}", e);
            None
        },
    };

    Json(data.unwrap())
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()?;
    
    rocket::ignite()
        .attach(MangadexDbConn::fairing())
        .attach(cors)
        .mount("/", routes![front_test, dex_test])
        .launch();
    
    Ok(())
}