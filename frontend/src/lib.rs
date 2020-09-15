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

async fn fetch_drills() -> Option<Msg> {
    let request = fetch("http://localhost:8000/mangas/").await;
    let payload: Result<JsonApiResponse, FetchError> = request.unwrap().json().await;
    Some(Msg::FetchedMangas(payload))
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(fetch_drills());
    Model { mangas: vec![] }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
