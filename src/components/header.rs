use crate::config::constants;
use crate::theme::ThemeButton;
use dioxus::prelude::*;

#[component]
pub fn Header(screen: Signal<super::app::Screen>) -> Element {
    // ---
    rsx! {
        header {
            nav {
                if *screen.read() == super::app::Screen::Edit {
                    button {
                        class: "circle transparent",
                        onclick: move |_| {
                            screen.set(super::app::Screen::List);
                        },
                        i { "arrow_back" }
                    }
                }
                nav { class: "max center-align",
                    h5 { class: "primary-text", "{constants::APP_NAME}" }
                    span { class: "small-text", "v{env!(\"CARGO_PKG_VERSION\")}" }
                }
                ThemeButton {}
            }
        }
    }
}
