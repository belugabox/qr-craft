//! Composant principal de l'application

use crate::components::qr_generator::QrGenerator;
use crate::components::saved_qr_list::SavedQrList;
use crate::config::constants;
use crate::models::qr_code::{SavedQr, UIQr};
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Auto,  // Suit automatiquement le thème système
    Dark,
    Light,
}

#[component]
pub fn App() -> Element {
    let mut theme = use_signal(|| Theme::Auto);
    let ui = use_signal(|| UIQr {
        text: constants::DEFAULT_QR_TEXT.into(),
        size: 256,
        transparent: false,
        image_data: String::new(),
    });
    let saved = use_signal(Vec::<SavedQr>::new);

    // Déterminer la classe de thème à appliquer
    let theme_class = match theme() {
        Theme::Auto => {
            // Pour le mode Auto, on utilise une classe spéciale qui sera gérée par CSS
            "theme-auto"
        }
        Theme::Dark => "theme-dark",
        Theme::Light => "theme-light",
    };

    rsx! {
        document::Stylesheet { href: CSS }
        div { class: theme_class,
            div { class: "flex flex-col min-h-screen bg-theme-primary text-theme-primary font-sans",

                header { class: "sticky z-50 top-0 p-4 bg-theme-header flex justify-between items-center border-b border-theme",
                    h1 { class: "text-4xl font-bold text-primary", "{constants::APP_NAME}" }
                    div { class: "flex items-center gap-4",
                        span { class: "text-xs text-theme-secondary", "v{env!(\"CARGO_PKG_VERSION\")}" }
                        button {
                            class: "p-2 rounded hover:bg-theme-secondary transition-colors text-theme-primary",
                            onclick: move |_| {
                                let current_theme = theme();
                                let new_theme = match current_theme {
                                    Theme::Auto => Theme::Dark,
                                    Theme::Dark => Theme::Light,
                                    Theme::Light => Theme::Auto,
                                };
                                theme.set(new_theme);
                            },
                            match theme() {
                                Theme::Auto => "�",
                                Theme::Dark => "🌙",
                                Theme::Light => "☀️",
                            }
                        }
                    }
                }

                div { class: "flex-grow",
                    SavedQrList { ui, saved }
                    QrGenerator { ui, saved }
                }
            }
        }
    }
}
