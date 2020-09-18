extern crate seed;
use seed::{ prelude::*, * };
use mangadex_reader::{ JsonApiResponse, Manga, MangaJsonWrapper };

struct Model {
    mangas: Vec<MangaJsonWrapper<Manga>>,
}

enum Msg {
    FetchedMangas(Result<JsonApiResponse<Manga>, FetchError>),
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

fn view(_model: &Model) -> impl IntoNodes<Msg> {
    nodes![
        view_nav(),
        view_main(),
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

// ------ Main ------
fn view_main() -> Node<Msg> {
    div![
        C!["container", "card-row"],
        div![
            C!["card-main"],
            h6![
                C!["card-header"],
                "Featured Daily Manga",
            ],
            div![
                C!["card-body"],
                div![
                    C!["card-body-title"],
                    a!["Mock Manga"],
                ],
                div![
                    C!["card-body-img"],
                    img![
                        attrs! {
                            At::Src => "https://via.placeholder.com/200";
                        }
                    ],
                ],
                div![
                    C!["card-body-info"],
                    div![
                        a!["Chapter mock"],
                    ],
                    div![
                        C!["group"],
                        a!["Mock Scans"],
                    ],
                    div![
                        span!["Mock mins ago"],
                    ],
                ],
            ],
        ],
    ]
}

async fn fetch_drills() -> Option<Msg> {
    let request = fetch("http://localhost:8000/mangas/").await;
    let payload: Result<JsonApiResponse<Manga>, FetchError> = request.unwrap().json().await;
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
