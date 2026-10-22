use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Mihai Mateias" }
        document::Meta {
            name: "description",
            content: "Mihai Mateias: developer, data scientist.",
        }
        section { class: "hero",
            h1 { "Mihai Mateias" }
            p { class: "subtitle", "Developer, data scientist" }
            p { "This site is a placeholder while the real pages are being written." }
            Link { to: Route::Projects {}, class: "button", "See projects" }
        }
    }
}
