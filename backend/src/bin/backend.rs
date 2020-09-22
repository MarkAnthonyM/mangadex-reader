#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use backend::api;
use backend::db::{ models, create_manga, query_manga };

use mangadex_reader::{ JsonApiResponse, Manga, MangaJsonWrapper };

use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;
use rocket_cors::{ AllowedHeaders, AllowedOrigins, Error };

// Rocket connection pool
#[database("postgres_mangadex")]
struct MangadexDbConn(diesel::PgConnection);

// Route handler returns payload containing manga listings
// Todo: Move from prototype state to finished state
#[get("/mangas")]
fn mangas_get(conn: MangadexDbConn) -> Json<JsonApiResponse<Manga>> {
    let mut response = JsonApiResponse { data: vec![] };

    for db_manga in query_manga(&conn) {
        // Convert database manga model response to api version of manga struct
        let api_manga = Manga {
            alt_names: db_manga.alt_names,
            artists: db_manga.artists,
            authors: db_manga.authors,
            comments: db_manga.comments,
            cover_url: db_manga.cover_url,
            covers: db_manga.covers,
            demographic: db_manga.demographic,
            description: db_manga.manga_description,
            follows: db_manga.follows,
            genres: db_manga.genres,
            hentai: db_manga.hentai,
            lang_flag: db_manga.lang_flag,
            lang_name: db_manga.lang_name,
            status: db_manga.manga_status,
            title: db_manga.title,
            views: db_manga.views,
        };

        // Toss queried manga item into wrapper struct that conforms to json api spec.
        let wrapped_manga = MangaJsonWrapper {
            _type: "mangas".to_string(),
            id: db_manga.manga_id,
            attributes: api_manga,
        };

        response.data.push(wrapped_manga)
    }

    Json(response)
}

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

    // TODO: Toss this logic straight into manga_result match, otherwise may try to unwrap Err()
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

#[post("/testfrontpost", data = "<manga>")]
fn new(conn: MangadexDbConn, manga: Json<MangaJsonWrapper<Manga>>) -> String {
    // Clone recived manga struct as endpoint does not own the state
    // Todo: Study this situation more throughly and find a better solution.
    // Possible starting point: https://stackoverflow.com/questions/56636643/how-to-return-a-state-value-from-a-rocket-endpoint
    // Check also request-local State from rocket docs.
    let cloned_manga = manga.clone();

    let db_manga = models::NewManga {
        alt_names: cloned_manga.attributes.alt_names,
        artists: cloned_manga.attributes.artists,
        authors: cloned_manga.attributes.authors,
        comments: cloned_manga.attributes.comments,
        cover_url: cloned_manga.attributes.cover_url,
        covers: cloned_manga.attributes.covers,
        demographic: manga.attributes.demographic,
        manga_description: cloned_manga.attributes.description,
        follows: cloned_manga.attributes.follows,
        genres: cloned_manga.attributes.genres,
        hentai: cloned_manga.attributes.hentai,
        lang_flag: cloned_manga.attributes.lang_flag,
        lang_name: cloned_manga.attributes.lang_name,
        manga_id: cloned_manga.id,
        manga_status: cloned_manga.attributes.status,
        title: cloned_manga.attributes.title,
        views: cloned_manga.attributes.views,
    };
    
    let result = create_manga(&conn, db_manga);

    format!("Successfully save into database! Amount inserted: {}", result)
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        // figure out what headers need to go here
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()?;
    
    rocket::ignite()
        .attach(MangadexDbConn::fairing())
        .attach(cors)
        .mount("/", routes![front_test, dex_test, mangas_get, new])
        .launch();
    
    Ok(())
}