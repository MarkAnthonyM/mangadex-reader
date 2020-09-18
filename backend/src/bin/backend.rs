#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use backend::db::query_manga;

use mangadex_reader::{ JsonApiResponse, Manga, MangaJsonWrapper };
use mangadex_reader::api;

use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error };

// Rocket connection pool
#[database("postgres_mangadex")]
struct MangadexDbConn(diesel::PgConnection);

// Route handler returns payload containing manga listings
#[get("/mangas")]
fn mangas_get(conn: MangadexDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

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

#[get("/testapi")]
fn api_test() -> Json<api::manga::Manga> {
    let test_manga = api::manga::Manga::populate();
    let data = match test_manga {
        Ok(result) => Some(result),
        Err(e) => {
            println!("Error with request: {:?}", e);
            None
        },
    };

    let response = data.unwrap();

    Json(response)
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
        .mount("/", routes![mangas_get, api_test])
        .launch();
    
    Ok(())
}