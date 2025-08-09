use dioxus::prelude::*;
use reqwest::*;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

static CSS: Asset = asset!("assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    let skip = move |evt| { };

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button {
                id: "skip",
                onclick: skip, 
                "skip",
            }
            button {
                id: "save",
                onclick: move |_| img_src.restart(),
                "save!" 
            }
        }
    }
}
