//! Composant principal de l'application

use crate::components::qr_generator::QrGenerator;
use crate::components::saved_qr_list::SavedQrList;
use crate::config::constants;
use crate::models::qr_code::UIQr;
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    let ui = use_signal(|| UIQr {
        text: constants::DEFAULT_QR_TEXT.into(),
        size: 256,
        transparent: false,
        image_data: String::new(),
    });
    let saved = use_signal(|| Vec::<String>::new());

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "flex flex-col min-h-screen bg-gray-800 text-white font-sans",

            header {
                class: "sticky z-50 top-0 p-4 bg-gray-900",
                h1 {
                    class: "text-4xl font-bold text-teal-400",
                    "{constants::APP_NAME}"
                }
            }

            div {
                class: "flex-grow",
                SavedQrList { ui, saved }
                QrGenerator { ui, saved }
            }

            footer { class: "sticky z-50 bottom-0 p-4 bg-gray-900",
                span {
                    class: "text-xs text-gray-400",
                    "v{env!(\"CARGO_PKG_VERSION\")}"
                }
            }
        }
    }
}
