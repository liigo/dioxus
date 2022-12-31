use dioxus::prelude::*;
use dioxus_desktop::{use_window, WindowBuilder};

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let window = use_window(cx);

    cx.render(rsx! {
        div {
            button {
                onclick: move |_| {
                    window.new_window(popup, (), Default::default());
                },
                "New Window"
            }
        }
    })
}

fn popup(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "This is a popup!" }
    })
}
