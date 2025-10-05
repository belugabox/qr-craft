use dioxus::launch;
use dioxus::prelude::*;

mod qr_generator;
mod qrcode;
mod saved_qr_list;
use qr_generator::QrGenerator;
use saved_qr_list::SavedQrList;

static CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Clone, Default)]
pub struct UIQr {
    text: String,
    size: u32,
    transparent: bool,
    image_data: String,
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let ui = use_signal(|| UIQr {
        text: "https://example.com".into(),
        size: 256,
        transparent: false,
        image_data: String::new(),
    });
    let saved = use_signal(|| Vec::<String>::new());

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "min-h-screen flex bg-gray-900 text-white",
            SavedQrList { ui, saved }
            QrGenerator { ui, saved }
        }
    }
}
