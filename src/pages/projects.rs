use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Projects() -> Element {
    rsx! {
        document::Title { "Projects · Mihai Mateias" }
        document::Meta {
            name: "description",
            content: "Projects by Mihai Mateias, including Harry Potter and the Chamber of Secrets in your browser.",
        }
        h1 { "Projects" }
        ul { class: "cards",
            li { class: "card",
                Link { to: Route::HarryPotter2 {},
                    h2 { "Harry Potter and the Chamber of Secrets (2002 PC)" }
                    p { "play it in your browser" }
                }
            }
        }
    }
}
