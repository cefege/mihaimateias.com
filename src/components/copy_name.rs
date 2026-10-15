use dioxus::prelude::*;

/// A monospace name with a button that copies it to the clipboard.
#[component]
pub fn CopyName(text: &'static str) -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        span { class: "copy-name",
            code { "{text}" }
            button {
                r#type: "button",
                class: "copy-button",
                onclick: move |_| {
                    // The text is sent as a value, so no string escaping is needed.
                    // The script resolves after 2 s, which resets the label.
                    let e = document::eval(
                        "await navigator.clipboard.writeText(await dioxus.recv()); \
                         await new Promise(r => setTimeout(r, 2000)); return true;",
                    );
                    let _ = e.send(text);
                    copied.set(true);
                    spawn(async move {
                        let _ = e.await;
                        copied.set(false);
                    });
                },
                if copied() { "Copied" } else { "Copy" }
            }
        }
    }
}
