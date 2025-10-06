use crate::components::icons::SaveIcon;
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
    // Signal séparé pour l'image générée afin d'éviter les boucles infinies
    let mut qr_image = use_signal(String::new);

    // Effet pour générer automatiquement le QR code quand les paramètres changent
    use_effect(move || {
        let text = ui().text.clone();
        let size = ui().size;
        let transparent = ui().transparent;
        let margin = ui().margin;

        if !text.is_empty() {
            spawn(async move {
                match generate_qr_code(text, size, transparent, margin).await {
                    Ok(data_url) => qr_image.set(data_url),
                    Err(e) => eprintln!("generate error: {}", e),
                }
            });
        } else {
            qr_image.set(String::new());
        }
    });

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
                        let new_text = e.value();
                        let mut v = (*ui.read()).clone();
                        v.text = new_text;
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
                        class: "p-2 btn-secondary",
                        onclick: move |_| {
                            let ui = ui;
                            let saved = saved;
                            to_owned![ui, saved];
                            async move {
                                let cur = (*ui.read()).clone();
                                let image_data = qr_image.read().clone();
                                if image_data.is_empty() {
                                    return;
                                }
                                let base64 = image_data
                                    .split_once(',')
                                    .map(|(_, b64)| b64)
                                    .unwrap_or(&image_data)
                                    .to_string();
                                let id = if let Some(editing_id) = &cur.editing_id {
                                    editing_id.clone()
                                } else {
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
                                    let mut v = (*ui.read()).clone();
                                    v.editing_id = None;
                                    ui.set(v);
                                }
                            }
                        },
                        SaveIcon { class: "w-4 h-4".to_string() }
                    }
                }

                if !qr_image.read().is_empty() {
                    div { class: "mt-4 p-3 rounded text-black bg-checkered",
                        img { src: "{qr_image.read()}" }
                    }
                }
            }
        }
    }
}
