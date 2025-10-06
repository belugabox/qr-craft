//! Composants d'icônes Material pour l'application
//! Utilise les Material Icons de Beer CSS

use dioxus::prelude::*;

/// Icône de suppression (corbeille)
#[component]
pub fn DeleteIcon(class: String) -> Element {
    rsx! {
        i { class: "{class}", "delete" }
    }
}

/// Icône de chargement (flèche vers le haut)
#[component]
pub fn LoadIcon(class: String) -> Element {
    rsx! {
        i { class: "{class}", "upload" }
    }
}

/// Icône de sauvegarde
#[component]
pub fn SaveIcon(class: String) -> Element {
    rsx! {
        i { class: "{class}", "save" }
    }
}

/// Icône de thème sombre
#[component]
pub fn MoonIcon(class: String) -> Element {
    rsx! {
        i { class: "{class}", "dark_mode" }
    }
}

/// Icône de thème clair
#[component]
pub fn SunIcon(class: String) -> Element {
    rsx! {
        i { class: "{class}", "light_mode" }
    }
}

/// Icône de génération
#[component]
pub fn GenerateIcon(class: String) -> Element {
    rsx! {
        i { class: "{class}", "add" }
    }
}

/// Icône de palette
#[component]
pub fn PaletteIcon(class: String) -> Element {
    rsx! {
        i { class: "{class}", "palette" }
    }
}

/// Icône d'échantillon (swatch/color picker)
#[component]
pub fn SwatchIcon(class: String) -> Element {
    rsx! {
        i { class: "{class}", "invert_colors" }
    }
}

/// Icône de crayon (édition)
#[component]
pub fn PencilIcon(class: String) -> Element {
    rsx! {
        i { class: "{class}", "edit" }
    }
}
