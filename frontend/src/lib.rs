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

fn view(model: &Model) -> impl IntoNodes<Msg> {
    nodes![
        view_nav(),
        view_main(model),
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
fn view_main(model: &Model) -> Node<Msg> {
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
                            At::Src => if model.mangas.len() > 0 {
                                format!("https://mangadex.org/{}", model.mangas[0].attributes.url_link)
                            } else {
                                "https://via.placeholder.com/200".into()
                            };
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
    let request = fetch("http://localhost:8000/testapi/").await;
    let payload: Result<JsonApiResponse<Manga>, FetchError> = request.unwrap().json().await;
    Some(Msg::FetchedMangas(payload))
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let model = Model { mangas: vec![] };
    orders.perform_cmd(fetch_drills());
    model
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
