use dioxus::prelude::*;

mod components;
mod pages;

use components::Shell;
use pages::{HarryPotter2, Home, NotFound, Projects};

/// The only place the game URL appears in the site.
pub const HP2_PLAY_URL: &str = "https://hp2.mihaimateias.com/";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Shell)]
        #[route("/")]
        Home {},
        #[route("/projects")]
        Projects {},
        #[route("/projects/harry-potter-2")]
        HarryPotter2 {},
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
