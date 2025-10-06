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
        div { class: "p-4",
            div { class: "pb-4 flex justify-center",
                button {
                    class: "btn-primary flex items-center gap-2 px-4 py-2",
                    onclick: move |_| {
                        screen.set(super::app::Screen::Edit);
                        ui.set(UIQr {
                            text: crate::config::constants::DEFAULT_QR_TEXT.into(),
                            size: 256,
                            transparent: false,
                            margin: crate::models::qr_code::MarginEnabled(true),
                            image_data: String::new(),
                            editing_id: None,
                        });
                    },
                    GenerateIcon { class: "w-4 h-4".to_string() }
                    "Créer nouveau QR code"
                }
            }

            if saved.read().is_empty() {
                div { class: "text-center py-12 text-theme-secondary",
                    p { class: "text-lg mb-4", "Aucun QR code sauvegardé" }
                    p { class: "text-sm", "Cliquez sur \"Créer nouveau QR code\" pour commencer" }
                }
            } else {
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    for qr in saved.read().iter().cloned() {
                        {
                            let qr_for_load = qr.clone();
                            let qr_for_delete = qr.clone();
                            rsx! {
                                div {
                                    key: "{qr.id}",
                                    class: "bg-theme-secondary rounded-lg p-4 shadow-sm border border-theme hover:shadow-md transition-shadow",
                                    div { class: "flex flex-col items-center gap-3",
                                        div { class: "bg-checkered p-2 rounded",
                                            img {
                                                src: "data:image/png;base64,{qr.image_data}",
                                                width: "128",
                                                height: "128",
                                                class: "rounded",
                                            }
                                        }
                                        div { class: "text-center w-full",
                                            p { class: "text-sm font-medium text-theme-primary truncate max-w-full",
                                                "{qr.text}"
                                            }
                                            p { class: "text-xs text-theme-secondary mt-1",
                                                "{qr.size}x{qr.size}px"
                                                if qr.transparent {
                                                    " • Transparent"
                                                } else {
                                                    ""
                                                }
                                            }
                                        }
                                        div { class: "flex gap-2 mt-2",
                                            button {
                                                class: "flex-1 px-3 py-2 btn-primary text-xs flex items-center justify-center gap-1",
                                                onclick: move |_| {
                                                    ui.set(UIQr {
                                                        text: qr_for_load.text.clone(),
                                                        size: qr_for_load.size,
                                                        transparent: qr_for_load.transparent,
                                                        margin: qr_for_load.margin,
                                                        image_data: format!("data:image/png;base64,{}", qr_for_load.image_data),
                                                        editing_id: Some(qr_for_load.id.clone()),
                                                    });
                                                    screen.set(super::app::Screen::Edit);
                                                },
                                                PencilIcon { class: "w-3 h-3".to_string() }
                                                "Modifier"
                                            }
                                            button {
                                                class: "px-3 py-2 bg-red-600 text-white rounded text-xs hover:bg-red-500 transition-colors flex items-center gap-1",
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
                                                DeleteIcon { class: "w-3 h-3".to_string() }
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
