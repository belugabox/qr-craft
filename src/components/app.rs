//! Composant principal de l'application
use crate::components::header::Header;
use crate::components::qr_generator::QrGenerator;
use crate::components::saved_qr_list::SavedQrList;
use crate::config::constants;
use crate::models::qr_code::{LogoId, MarginEnabled, SavedQr, UIQr};
use crate::theme::{Theme, ThemeMode};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    List, // Écran de liste des QR codes
    Edit, // Écran d'édition/création de QR code
}

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(ThemeMode::Auto));

    let screen = use_signal(|| Screen::List); // Commencer par l'écran de liste
    let ui = use_signal(|| UIQr {
        id: format!("qr-{}", fastrand::u64(..)),
        text: constants::DEFAULT_QR_TEXT.into(),
        size: 256,
        transparent: false,
        margin: MarginEnabled(true),
        logo_id: LogoId::None,
        logo_ratio: 0.20,
    });
    let saved = use_signal(Vec::<SavedQr>::new);

    rsx! {
        Theme {}
        Header { screen }
        main { class: "responsive",
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
