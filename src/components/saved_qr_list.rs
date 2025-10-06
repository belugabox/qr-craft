use crate::models::qr_code::{SavedQr, UIQr};
use crate::services::qr_code::{delete_saved, list_saved};
use dioxus::prelude::*;

#[component]
pub fn SavedQrList(ui: Signal<UIQr>, saved: Signal<Vec<SavedQr>>) -> Element {
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
        nav { class: "p-4",
            button {
                class: "p-2 bg-teal-600 rounded hover:bg-teal-500 transition-colors",
                onclick: move |_| {
                    let saved = saved;
                    to_owned![saved];
                    async move {
                        if let Ok(list) = list_saved().await {
                            saved.set(list);
                        }
                    }
                },
                "Refresh"
            }

            div { class: "mt-4 space-y-2",
                for qr in saved.read().iter().cloned() {
                    {
                        let qr_for_load = qr.clone();
                        let qr_for_delete = qr.clone();
                        rsx! {
                            div { key: "{qr.id}", class: "flex items-center gap-2",
                                img {
                                    src: "data:image/png;base64,{qr.image_data}",
                                    width: "64",
                                    height: "64",
                                }
                                span { class: "flex-1 truncate text-sm", "{qr.text}" }
                                button {
                                    class: "px-2 py-1 bg-teal-600 rounded text-xs hover:bg-teal-500 transition-colors",
                                    onclick: move |_| {
                                        let ui = ui;
                                        let qr_for_load = qr_for_load.clone();
                                        to_owned![ui];
                                        async move {
                                            ui.set(UIQr {
                                                text: qr_for_load.text,
                                                size: qr_for_load.size,
                                                transparent: qr_for_load.transparent,
                                                image_data: format!("data:image/png;base64,{}", qr_for_load.image_data),
                                            });
                                        }
                                    },
                                    "Load"
                                }
                                button {
                                    class: "px-2 py-1 bg-red-600 rounded text-xs hover:bg-red-500 transition-colors",
                                    onclick: move |_| {
                                        let qr_for_delete = qr_for_delete.clone();
                                        to_owned![saved];
                                        async move {
                                            if delete_saved(qr_for_delete.id).await.is_ok() {
                                                if let Ok(list) = list_saved().await {
                                                    saved.set(list);
                                                }
                                            }
                                        }
                                    },
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
