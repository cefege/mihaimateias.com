use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Shell() -> Element {
    rsx! {
        header { class: "site-header",
            Link { to: Route::Home {}, class: "site-name", "Mihai Mateias" }
            nav {
                Link { to: Route::Projects {}, "Projects" }
            }
        }
        main { class: "content",
            Outlet::<Route> {}
        }
        footer { class: "site-footer",
            ul { class: "social",
                li { a { href: "https://www.linkedin.com/in/mihai-mateias-487609170/", "LinkedIn" } }
                li { a { href: "https://www.youtube.com/@mihai_mateias", "YouTube" } }
                li { a { href: "https://x.com/cefegezor", "X" } }
                li { a { href: "https://github.com/cefege", "GitHub" } }
            }
        }
    }
}
