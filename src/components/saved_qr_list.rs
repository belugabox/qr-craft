use crate::models::qr_code::{LogoId, SavedQr, UIQr};
use crate::services::qr_code::{delete_saved, list_saved};
use dioxus::prelude::*;

#[component]
pub fn SavedQrList(
    ui: Signal<UIQr>,
    saved: Signal<Vec<SavedQr>>,
    screen: Signal<super::app::Screen>,
) -> Element {
    // Charger automatiquement la liste des QR codes sauvegardés au démarrage
    use_effect(move || {
        let mut saved = saved;
        spawn(async move {
            if let Ok(list) = list_saved().await {
                saved.set(list);
            }
        });
    });

    let h_create_new_qr = {
        move || async move {
            ui.set(UIQr {
                id: format!("qr-{}", fastrand::u64(..)),
                text: crate::config::constants::DEFAULT_QR_TEXT.into(),
                size: 256,
                transparent: false,
                margin: crate::models::qr_code::MarginEnabled(true),
                logo_id: LogoId::None,
                logo_ratio: 0.20,
            });
            screen.set(super::app::Screen::Edit);
        }
    };

    let h_edit_qr = {
        move |qr: SavedQr| async move {
            let mut qr = qr.clone();
            ui.set(UIQr {
                id: qr.id.clone(),
                text: qr.text.clone(),
                size: qr.size,
                transparent: qr.transparent,
                margin: qr.margin,
                logo_id: qr.logo_id,
                logo_ratio: qr.logo_ratio,
            });
            screen.set(super::app::Screen::Edit);
        }
    };

    let h_delete_qr = {
        to_owned![saved];
        move |qr: SavedQr| async move {
            match delete_saved(qr.id.clone()).await {
                Ok(_) => {
                    if let Ok(list) = list_saved().await {
                        saved.set(list);
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Erreur lors de la suppression du QR code {}: {:?}",
                        qr.id, e,
                    );
                    if let Ok(list) = list_saved().await {
                        saved.set(list);
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "",
            if saved.read().is_empty() {
                div { class: "center-align padding",
                    p { class: "large", "Aucun QR code sauvegardé" }
                    p { class: "small", "Cliquez sur \"Créer nouveau QR code\" pour commencer" }
                }
            } else {
                div { class: "grid margin",
                    for qr in saved.read().iter().cloned() {
                        {
                            let qr_for_load = qr.clone();
                            let qr_for_delete = qr.clone();
                            rsx! {
                                div { key: "{qr.id}", class: "s12 m6 l4",
                                    article {
                                        div { class: "row",
                                            div { class: "bg-checkered",
                                                img {
                                                    src: qr.image_data_url.clone(),
                                                    alt: "QR code",
                                                    width: "96",
                                                    height: "96",
                                                }
                                            }
                                            div { class: "max row vertical no-space",
                                                p { class: "truncate-text responsive", "{qr.text}" }
                                                p { class: "small-text",
                                                    "{qr.size}x{qr.size}px"
                                                    br {}
                                                    if qr.transparent {
                                                        "Transparent"
                                                    } else {
                                                        ""
                                                    }
                                                }
                                            }
                                            nav { class: "left-align vertical",
                                                button { class: "transparent circle small",
                                                    i { "more_vert" }
                                                    menu { class: "left no-wrap",
                                                        li { onclick: move |_| { h_delete_qr(qr_for_delete.clone()) },
                                                            i { "delete" }
                                                            "Supprimer"
                                                        }
                                                    }
                                                }
                                                div { class: "max" }
                                                button {
                                                    class: "circle small",
                                                    onclick: move |_| { h_edit_qr(qr_for_load.clone()) },
                                                    i { "edit" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "center-align margin",
                button { onclick: move |_| { h_create_new_qr() },
                    i { "add" }
                    "Créer nouveau QR code"
                }
            }
        }
    }
}
