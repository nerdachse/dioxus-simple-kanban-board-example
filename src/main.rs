#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
mod components;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    rsx! {
        div {class: "bg-nord0",
            Router::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    use crate::components::Board;

    rsx! {
        body {class: "bg-nord0"}
        Board { title: "Totally not a surfboard".to_owned() }
    }
}
