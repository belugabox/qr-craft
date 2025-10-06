use crate::components::icons::{GenerateIcon, SaveIcon};
use crate::models::qr_code::{MarginEnabled, SavedQr, UIQr};
use crate::services::qr_code::{generate_qr_code, list_saved, save_qr};
use dioxus::prelude::*;
use js_sys::Date;

#[component]
pub fn QrGenerator(
    ui: Signal<UIQr>,
    saved: Signal<Vec<SavedQr>>,
    screen: Signal<super::app::Screen>,
) -> Element {
    rsx! {
        div { class: "flex-1 p-8",
            div { class: "mb-6",
                button {
                    class: "btn-secondary flex items-center gap-2 px-4 py-2",
                    onclick: move |_| {
                        screen.set(super::app::Screen::List);
                    },
                    "← Retour à la liste"
                }
            }

            div { class: "space-y-3",
                input {
                    class: "p-2 w-full",
                    placeholder: "Text or URL...",
                    value: "{ui.read().text}",
                    oninput: move |e| {
                        let mut v = (*ui.read()).clone();
                        v.text = e.value();
                        ui.set(v);
                    },
                }

                div { class: "flex items-center gap-4",
                    select {
                        class: "p-2",
                        value: "{ui.read().size}",
                        onchange: move |e| {
                            if let Ok(s) = e.value().parse::<u32>() {
                                let mut v = (*ui.read()).clone();
                                v.size = s;
                                ui.set(v);
                            }
                        },
                        option { value: "128", "128" }
                        option { value: "256", "256" }
                        option { value: "512", "512" }
                    }
                    input {
                        r#type: "checkbox",
                        checked: "{ui.read().margin.0}",
                        onchange: move |e| {
                            let mut v = (*ui.read()).clone();
                            v.margin = MarginEnabled(e.value() == "on" || e.value() == "true");
                            ui.set(v);
                        },
                    }
                    span { "Marge" }
                    input {
                        r#type: "checkbox",
                        checked: "{ui.read().transparent}",
                        onchange: move |e| {
                            let mut v = (*ui.read()).clone();
                            v.transparent = e.value() == "on" || e.value() == "true";
                            ui.set(v);
                        },
                    }
                    span { "Transparent" }
                }

                div { class: "flex gap-2",
                    button {
                        class: "p-2 btn-primary",
                        onclick: move |_| {
                            let ui = ui;
                            to_owned![ui];
                            async move {
                                let cur = (*ui.read()).clone();
                                match generate_qr_code(
                                        cur.text.clone(),
                                        cur.size,
                                        cur.transparent,
                                        cur.margin,
                                    )
                                    .await
                                {
                                    Ok(data_url) => {
                                        let mut v = (*ui.read()).clone();
                                        v.image_data = data_url;
                                        ui.set(v);
                                    }
                                    Err(e) => {
                                        eprintln!("generate error: {}", e);
                                    }
                                }
                            }
                        },
                        GenerateIcon { class: "w-4 h-4".to_string() }
                    }

                    button {
                        class: "p-2 btn-secondary",
                        onclick: move |_| {
                            let ui = ui;
                            let saved = saved;
                            to_owned![ui, saved];
                            async move {
                                let cur = (*ui.read()).clone();
                                if cur.image_data.is_empty() {
                                    return;
                                }
                                let base64 = cur
                                    .image_data
                                    .split_once(',')
                                    .map(|(_, b64)| b64)
                                    .unwrap_or(&cur.image_data)
                                    .to_string();

                                let id = if let Some(editing_id) = &cur.editing_id {
                                    // Mode édition : utiliser l'ID existant
                                    editing_id.clone()
                                } else {
                                    // Mode création : générer un nouvel ID
                                    format!("qr-{}", fastrand::u64(..))
                                };

                                let saved_q = SavedQr {
                                    id: id.clone(),
                                    text: cur.text.clone(),
                                    size: cur.size,
                                    transparent: cur.transparent,
                                    margin: cur.margin,
                                    created_at: format!("{}", (Date::now() / 1000.0) as u64),
                                    image_data: base64,
                                };
                                if save_qr(saved_q).await.is_ok() {
                                    if let Ok(list) = list_saved().await {
                                        saved.set(list);
                                    }
                                    // Réinitialiser l'état d'édition après sauvegarde
                                    let mut v = (*ui.read()).clone();
                                    v.editing_id = None;
                                    ui.set(v);
                                }
                            }
                        },
                        SaveIcon { class: "w-4 h-4".to_string() }
                    }
                }

                if !ui.read().image_data.is_empty() {
                    div { class: "mt-4 p-3 rounded text-black bg-checkered",
                        img { src: "{ui.read().image_data}" }
                    }
                }
            }
        }
    }
}
