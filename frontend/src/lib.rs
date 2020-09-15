extern crate seed;
use seed::{ prelude::*, * };
use mangadex_reader::{ JsonApiResponse, MangaJsonWrapper };

struct Model {
    mangas: Vec<MangaJsonWrapper>,
}

enum Msg {
    FetchedMangas(Result<JsonApiResponse, FetchError>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedMangas(Ok(mut result)) => {
            model.mangas.clear();
            model.mangas.append(&mut result.data);
        },
        Msg::FetchedMangas(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        },
    }
}

fn view(model: &Model) -> Node<Msg> {
    h1![
        "Mangas",
        ul![
            model
                .mangas
                .iter()
                .map(|t| li![ t.attributes.title.clone() ])
        ],
    ]
}
