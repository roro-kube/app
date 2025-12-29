use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            style: "padding: 20px; font-family: sans-serif;",
            h1 { "Roro Kube" }
            p { "Dioxus desktop application initialized successfully!" }
        }
    }
}

