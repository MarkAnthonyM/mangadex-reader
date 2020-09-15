extern crate seed;
use seed::{ prelude::*, * };
use mangadex_reader::{ JsonApiResponse, MangaJsonWrapper };

struct Model {
    Mangas: Vec<MangaJsonWrapper>,
}

enum Msg {
    FetchedMangas(Result<JsonApiResponse, FetchError>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedMangas(Ok(mut result)) => {
            model.Mangas.clear();
            model.Mangas.append(&mut result.data);
        },
        Msg::FetchedMangas(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        },
    }
}