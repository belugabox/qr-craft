use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;

static CUSTOM_CSS: Asset = asset!("/assets/custom.css");
static THEME_JS: Asset = asset!("/assets/theme.js");

#[derive(Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Auto, // Suit automatiquement le thème système
    Dark,
    Light,
}

#[component]
pub fn Theme() -> Element {
    let mut theme = use_context::<Signal<ThemeMode>>();

    // Charger le thème sauvegardé au démarrage
    use_effect(move || {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved_theme)) = storage.get_item("app-theme") {
                    let loaded_theme = match saved_theme.as_str() {
                        "dark" => ThemeMode::Dark,
                        "light" => ThemeMode::Light,
                        _ => ThemeMode::Auto,
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
            ThemeMode::Auto => "auto",
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
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

    // ---
    rsx! {
        document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/beercss@3.12.11/dist/cdn/beer.min.css" }
        document::Script { src: "https://cdn.jsdelivr.net/npm/beercss@3.12.11/dist/cdn/beer.min.js" }
        document::Script { src: "https://cdn.jsdelivr.net/npm/material-dynamic-colors@1.1.2/dist/cdn/material-dynamic-colors.min.js" }
        document::Stylesheet { href: CUSTOM_CSS }
        document::Script { src: THEME_JS }
    }
}

#[component]
pub fn ThemeButton() -> Element {
    let mut theme = use_context::<Signal<ThemeMode>>();
    // ---
    rsx! {
        button {
            class: "circle transparent small",
            onclick: move |_| {
                let current_theme = theme();
                let new_theme = match current_theme {
                    ThemeMode::Auto => ThemeMode::Dark,
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Auto,
                };
                theme.set(new_theme);
            },
            match theme() {
                ThemeMode::Auto => rsx! {
                    i { "invert_colors" }
                },
                ThemeMode::Dark => rsx! {
                    i { "dark_mode" }
                },
                ThemeMode::Light => rsx! {
                    i { "light_mode" }
                },
            }
        }
    }
}
