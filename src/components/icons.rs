//! Composants d'icônes SVG pour l'application

use dioxus::prelude::*;

/// Icône de suppression (corbeille)
#[component]
pub fn DeleteIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());
    rsx! {
        svg {
            class: "{class}",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16",
            }
        }
    }
}

/// Icône de chargement (flèche vers le haut)
#[component]
pub fn LoadIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());
    rsx! {
        svg {
            class: "{class}",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M7 16l-4-4m0 0l4-4m-4 4h18",
            }
        }
    }
}

/// Icône de sauvegarde (disquette)
#[component]
pub fn SaveIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());
    rsx! {
        svg {
            class: "{class}",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M8 7H5a2 2 0 00-2 2v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4",
            }
        }
    }
}

/// Icône de thème sombre
#[component]
pub fn MoonIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());
    rsx! {
        svg {
            class: "{class}",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z",
            }
        }
    }
}

/// Icône de thème clair
#[component]
pub fn SunIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());
    rsx! {
        svg {
            class: "{class}",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z",
            }
        }
    }
}

/// Icône de génération (éclair/zap)
#[component]
pub fn GenerateIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());
    rsx! {
        svg {
            class: "{class}",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M13 10V3L4 14h7v7l9-11h-7z",
            }
        }
    }
}

/// Icône de palette (peinture)
#[component]
pub fn PaletteIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());
    rsx! {
        svg {
            class: "{class}",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            path {
                stroke_width: "1.5",
                d: "M20.5096 9.54C20.4243 9.77932 20.2918 9.99909 20.12 10.1863C19.9483 10.3735 19.7407 10.5244 19.5096 10.63C18.2796 11.1806 17.2346 12.0745 16.5002 13.2045C15.7659 14.3345 15.3733 15.6524 15.3696 17C15.3711 17.4701 15.418 17.9389 15.5096 18.4C15.5707 18.6818 15.5747 18.973 15.5215 19.2564C15.4682 19.5397 15.3588 19.8096 15.1996 20.05C15.0649 20.2604 14.8877 20.4403 14.6793 20.5781C14.4709 20.7158 14.2359 20.8085 13.9896 20.85C13.4554 20.9504 12.9131 21.0006 12.3696 21C11.1638 21.0006 9.97011 20.7588 8.85952 20.2891C7.74893 19.8194 6.74405 19.1314 5.90455 18.2657C5.06506 17.4001 4.40807 16.3747 3.97261 15.2502C3.53714 14.1257 3.33208 12.9252 3.36959 11.72C3.4472 9.47279 4.3586 7.33495 5.92622 5.72296C7.49385 4.11097 9.60542 3.14028 11.8496 3H12.3596C14.0353 3.00042 15.6777 3.46869 17.1017 4.35207C18.5257 5.23544 19.6748 6.49885 20.4196 8C20.6488 8.47498 20.6812 9.02129 20.5096 9.52V9.54Z",
            }
            path {
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M8 16.01L8.01 15.9989",
            }
            path {
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6 12.01L6.01 11.9989",
            }
            path {
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M8 8.01L8.01 7.99889",
            }
            path {
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 6.01L12.01 5.99889",
            }
            path {
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M16 8.01L16.01 7.99889",
            }
        }
    }
}

/// Icône d'échantillon (swatch/color picker)
#[component]
pub fn SwatchIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());
    rsx! {
        svg {
            class: "{class}",
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            "xmlns": "http://www.w3.org/2000/svg",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
                d: "M4.098 19.902a3.75 3.75 0 0 0 5.304 0l6.401-6.402M6.75 21A3.75 3.75 0 0 1 3 17.25V4.125C3 3.504 3.504 3 4.125 3h5.25c.621 0 1.125.504 1.125 1.125v4.072M6.75 21a3.75 3.75 0 0 0 3.75-3.75V8.197M6.75 21h13.125c.621 0 1.125-.504 1.125-1.125v-5.25c0-.621-.504-1.125-1.125-1.125h-4.072M10.5 8.197l2.88-2.88c.438-.439 1.15-.439 1.59 0l3.712 3.713c.44.44.44 1.152 0 1.59l-2.879 2.88M6.75 17.25h.008v.008H6.75v-.008Z",
            }
        }
    }
}

/// Icône de crayon (édition)
#[component]
pub fn PencilIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());
    rsx! {
        svg {
            class: "{class}",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "1.5",
            stroke: "currentColor",
            "xmlns": "http://www.w3.org/2000/svg",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L6.832 19.82a4.5 4.5 0 0 1-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 0 1 1.13-1.897L16.863 4.487Zm0 0L19.5 7.125",
            }
        }
    }
}
