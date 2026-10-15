use dioxus::prelude::*;

use crate::HP2_PLAY_URL;
use crate::components::CopyName;

#[component]
pub fn HarryPotter2() -> Element {
    rsx! {
        document::Title { "Harry Potter and the Chamber of Secrets in your browser · Mihai Mateias" }
        document::Meta {
            name: "description",
            content: "Play Harry Potter and the Chamber of Secrets (2002 PC) in your browser with your own copy of the game.",
        }
        article { class: "hp2",
            h1 { "Harry Potter and the Chamber of Secrets (2002 PC)" }

            section { class: "play",
                a { class: "button big", href: HP2_PLAY_URL, "Play now" }
                p { "Free. You bring your own copy of the PC game. Nothing is uploaded; your files and saves stay on your computer." }
            }

            section {
                h2 { "What you need" }
                ul {
                    li { "The PC game (2002) installed, or a copy of its installed folder." }
                    li { "A supported browser: Chrome, Edge or Firefox on a computer, or Chrome/Firefox on Android." }
                    li { "Not supported: iPhone and iPad." }
                }
            }

            section {
                h2 { "Find your game folder" }
                p { "Look for a folder with this name:" }
                p { CopyName { text: "Harry Potter and the Chamber of Secrets" } }
                p { "It contains these folders: " }
                p {
                    for name in ["Maps", "System", "Textures", "Sounds", "Music", "Help"] {
                        CopyName { text: name }
                        " "
                    }
                }
                p { "Files to search for:" }
                ul {
                    li {
                        CopyName { text: "HP2_Master.utx" }
                        " inside "
                        code { "Textures" }
                        " (the sure one)"
                    }
                    li {
                        CopyName { text: "HGame.u" }
                        " inside "
                        code { "System" }
                        ". HP1 also has an "
                        code { "HGame.u" }
                        ", so "
                        code { "HP2_Master.utx" }
                        " is the one to rely on."
                    }
                }
                p { "How to search:" }
                ul {
                    li { "Windows: open File Explorer → This PC → type the name in the search box." }
                    li { "Mac: press ⌘ Space and type the name, or use Finder's search." }
                }
                p { "Then choose the folder one level above " code { "Textures" } "." }
            }

            section {
                h2 { "Play" }
                ol {
                    li { "Click " a { href: HP2_PLAY_URL, "Play now" } "." }
                    li { "In the game choose " strong { "Game Data → Import retail game…" } "." }
                    li { "Pick the game folder you found above." }
                    li { "If the browser asks to “upload” files, say yes. They stay on your device." }
                    li { strong { "New Game" } "." }
                }
            }

            section {
                h2 { "Your saves" }
                p { "Saves are stored in this browser on this computer. Click " strong { "Save backup" } " in the game to download a file, and " strong { "Restore backup" } " to load it again, for example after clearing browser data or on another computer." }
            }

            section {
                h2 { "Problems" }
                ul {
                    li { "“No System/Default.ini found”: you picked the wrong folder. Pick the one that contains " code { "System" } "." }
                    li { "“This browser cannot run the game”: switch to Chrome, Edge or Firefox." }
                }
            }

            p { class: "legal", "Not affiliated with EA or Warner Bros. Game files are not provided here." }
        }
    }
}
