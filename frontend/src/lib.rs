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

// ------ ------
//      View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    nodes![
        view_nav(),
    ]
}

// ------ Nav ------

fn view_nav() -> Node<Msg> {
    nav![
        C!["navbar", "bg-light"],
        div![
            C!["container"],
            a![
                C!["navbar-brand"],
                img![
                    attrs! {
                        At::Src => "https://via.placeholder.com/60";
                    }
                ]
            ],
            h1![C!["navbar-title"], "Manga Test Site"],
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
