use crate::components::icons::{DeleteIcon, GenerateIcon, PencilIcon};
use crate::models::qr_code::{SavedQr, UIQr};
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

    rsx! {
        div { class: "padding",
            div { class: "center-align small-margin",
                button {
                    class: "fill primary",
                    onclick: move |_| {
                        screen.set(super::app::Screen::Edit);
                        ui.set(UIQr {
                            text: crate::config::constants::DEFAULT_QR_TEXT.into(),
                            size: 256,
                            transparent: false,
                            margin: crate::models::qr_code::MarginEnabled(true),
                            editing_id: None,
                        });
                    },
                    GenerateIcon { class: "".to_string() }
                    "Créer nouveau QR code"
                }
            }

            if saved.read().is_empty() {
                div { class: "center-align padding",
                    p { class: "large", "Aucun QR code sauvegardé" }
                    p { class: "small", "Cliquez sur \"Créer nouveau QR code\" pour commencer" }
                }
            } else {
                div { class: "grid",
                    for qr in saved.read().iter().cloned() {
                        {
                            let qr_for_load = qr.clone();
                            let qr_for_delete = qr.clone();
                            rsx! {
                                div {
                                    key: "{qr.id}",
                                    class: "s12 m6 l4",
                                    article { class: "surface-variant round padding",
                                        div { class: "center-align",
                                            div { class: "bg-checkered padding round",
                                                img {
                                                    src: "data:image/png;base64,{qr.image_data}",
                                                    width: "128",
                                                    height: "128",
                                                    class: "round",
                                                }
                                            }
                                            div { class: "small-margin",
                                                p { class: "bold",
                                                    "{qr.text}"
                                                }
                                                p { class: "small",
                                                    "{qr.size}x{qr.size}px"
                                                    if qr.transparent {
                                                        " • Transparent"
                                                    } else {
                                                        ""
                                                    }
                                                }
                                            }
                                            div { class: "row",
                                                button {
                                                    class: "small primary",
                                                    onclick: move |_| {
                                                        ui.set(UIQr {
                                                            text: qr_for_load.text.clone(),
                                                            size: qr_for_load.size,
                                                            transparent: qr_for_load.transparent,
                                                            margin: qr_for_load.margin,
                                                            editing_id: Some(qr_for_load.id.clone()),
                                                        });
                                                        screen.set(super::app::Screen::Edit);
                                                    },
                                                    PencilIcon { class: "".to_string() }
                                                    "Modifier"
                                                }
                                                button {
                                                    class: "small red",
                                                    onclick: move |_| {
                                                        let qr_for_delete = qr_for_delete.clone();
                                                        to_owned![saved];
                                                        async move {
                                                            match delete_saved(qr_for_delete.id.clone()).await {
                                                                Ok(_) => {
                                                                    if let Ok(list) = list_saved().await {
                                                                        saved.set(list);
                                                                    }
                                                                }
                                                                Err(e) => {
                                                                    eprintln!(
                                                                        "Erreur lors de la suppression du QR code {}: {:?}",
                                                                        qr_for_delete.id,
                                                                        e,
                                                                    );
                                                                    if let Ok(list) = list_saved().await {
                                                                        saved.set(list);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    DeleteIcon { class: "".to_string() }
                                                    "Supprimer"
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
        }
    }
}
