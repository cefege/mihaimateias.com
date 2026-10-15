use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let _ = segments;
    rsx! {
        document::Title { "Page not found · Mihai Mateias" }
        document::Meta { name: "robots", content: "noindex" }
        h1 { "Page not found" }
        p { Link { to: Route::Home {}, "Back to the home page" } }
    }
}
