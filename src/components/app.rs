//! Composant principal de l'application

use crate::components::icons::{MoonIcon, SunIcon, SwatchIcon};
use crate::components::qr_generator::QrGenerator;
use crate::components::saved_qr_list::SavedQrList;
use crate::config::constants;
use crate::models::qr_code::{SavedQr, UIQr};
use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;

static CSS: Asset = asset!("/assets/beer.min.css");
static CUSTOM_CSS: Asset = asset!("/assets/custom.css");
static THEME_JS: Asset = asset!("/assets/theme.js");

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Auto, // Suit automatiquement le thème système
    Dark,
    Light,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    List, // Écran de liste des QR codes
    Edit, // Écran d'édition/création de QR code
}

#[component]
pub fn App() -> Element {
    let mut theme = use_signal(|| Theme::Auto);
    let screen = use_signal(|| Screen::List); // Commencer par l'écran de liste
    let ui = use_signal(|| UIQr {
        text: crate::config::constants::DEFAULT_QR_TEXT.into(),
        size: 256,
        transparent: false,
        margin: crate::models::qr_code::MarginEnabled(true),
        editing_id: None,
    });
    let saved = use_signal(Vec::<SavedQr>::new);

    // Charger le thème sauvegardé au démarrage
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved_theme)) = storage.get_item("app-theme") {
                    let loaded_theme = match saved_theme.as_str() {
                        "dark" => Theme::Dark,
                        "light" => Theme::Light,
                        _ => Theme::Auto,
                    };
                    theme.set(loaded_theme);
                }
            }
        }
    });

    // Effet pour appliquer le thème au niveau HTML
    use_effect(move || {
        let theme_value = theme();
        let theme_str = match theme_value {
            Theme::Auto => "auto",
            Theme::Dark => "dark",
            Theme::Light => "light",
        };

        // Appeler la fonction JavaScript window.setTheme
        if let Some(window) = web_sys::window() {
            if let Ok(set_theme) = js_sys::Reflect::get(&window, &"setTheme".into()) {
                if let Ok(func) = set_theme.dyn_into::<js_sys::Function>() {
                    let _ = func.call1(&window, &theme_str.into());
                }
            }
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" }
        document::Stylesheet { href: CSS }
        document::Stylesheet { href: CUSTOM_CSS }
        document::Script { src: THEME_JS }

        div { class: "responsive",

            nav { class: "top",
                h5 { class: "max", "{constants::APP_NAME}" }
                div { class: "max" }
                span { class: "small-text", "v{env!(\"CARGO_PKG_VERSION\")}" }
                button {
                    class: "circle transparent",
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
                        Theme::Auto => rsx! {
                            SwatchIcon { class: "".to_string() }
                        },
                        Theme::Dark => rsx! {
                            MoonIcon { class: "".to_string() }
                        },
                        Theme::Light => rsx! {
                            SunIcon { class: "".to_string() }
                        },
                    }
                }
            }

            main { class: "center-align",
                match screen() {
                    Screen::List => rsx! {
                        SavedQrList { ui, saved, screen }
                    },
                    Screen::Edit => rsx! {
                        QrGenerator { ui, saved, screen }
                    },
                }
            }
        }
    }
}
