extern crate seed;
use seed::{ prelude::*, * };
use mangadex_reader::{ JsonApiResponse, MangaJsonWrapper };

struct Model {
    Mangas: Vec<MangaJsonWrapper>,
}

enum Msg {
    FetchedMangas(Result<JsonApiResponse, FetchError>),
}