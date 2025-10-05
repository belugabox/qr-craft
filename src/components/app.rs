//! Composant principal de l'application

use crate::components::qr_generator::QrGenerator;
use crate::components::saved_qr_list::SavedQrList;
use crate::config::constants;
use crate::models::qr_code::{SavedQr, UIQr};
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
    let saved = use_signal(|| Vec::<SavedQr>::new());

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            class: "flex flex-col min-h-screen bg-gray-900 text-white font-sans",

            header {
                class: "sticky z-50 top-0 p-4 bg-gray-900 flex justify-between items-center",
                h1 {
                    class: "text-4xl font-bold text-teal-400",
                    "{constants::APP_NAME}"
                }
                span {
                    class: "text-xs text-gray-400",
                    "v{env!(\"CARGO_PKG_VERSION\")}"
                }
            }

            div {
                class: "flex-grow",
                SavedQrList { ui, saved }
                QrGenerator { ui, saved }
            }
        }
    }
}
