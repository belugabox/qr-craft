use crate::models::qr_code::UIQr;
use crate::services::qr_code::{delete_saved, list_saved, load_saved};
use dioxus::prelude::*;

#[component]
pub fn SavedQrList(ui: Signal<UIQr>, saved: Signal<Vec<String>>) -> Element {
    // Charger automatiquement la liste des QR codes sauvegardés au démarrage
    use_effect(move || {
        let mut saved = saved.clone();
        spawn(async move {
            if let Ok(list) = list_saved().await {
                saved.set(list);
            }
        });
    });

    rsx! {
        nav { class: "w-64 p-4 bg-gray-800",
            h2 { class: "mb-2 text-lg", "Saved QRs" }
            button { class: "p-2 bg-teal-600 rounded hover:bg-teal-500 transition-colors", onclick: move |_| {
                let saved = saved.clone(); to_owned![saved];
                async move {
                    if let Ok(list) = list_saved().await {
                        saved.set(list);
                    }
                }
            }, "Refresh" }

            div { class: "mt-4 space-y-2",
                for name in saved.read().iter().cloned() {
                    {
                        let name_for_load = name.clone();
                        let name_for_delete = name.clone();
                        rsx! {
                            div {
                                key: "{name}",
                                class: "flex items-center gap-2",
                                span { class: "flex-1 truncate text-sm", "{name}" }
                                button {
                                    class: "px-2 py-1 bg-teal-600 rounded text-xs hover:bg-teal-500 transition-colors",
                                    onclick: move |_| {
                                        let ui = ui.clone(); let name_for_load = name_for_load.clone(); to_owned![ui];
                                        async move {
                                            if let Ok(s) = load_saved(name_for_load).await {
                                                ui.set(UIQr { text: s.text, size: s.size, transparent: s.transparent, image_data: format!("data:image/png;base64,{}", s.image_data) });
                                            }
                                        }
                                    },
                                    "Load"
                                }
                                button {
                                    class: "px-2 py-1 bg-red-600 rounded text-xs hover:bg-red-500 transition-colors",
                                    onclick: move |_| {
                                        let saved = saved.clone(); let name_for_delete = name_for_delete.clone(); to_owned![saved];
                                        async move {
                                            if let Ok(_) = delete_saved(name_for_delete).await {
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
