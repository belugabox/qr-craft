use crate::models::qr_code::{MarginEnabled, SavedQr, UIQr};
use crate::services::qr_code::{generate_qr_code, list_saved, save_qr};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use js_sys::Date;
use web_sys::{wasm_bindgen::JsCast, window, HtmlElement};

#[component]
pub fn QrGenerator(
    ui: Signal<UIQr>,
    saved: Signal<Vec<SavedQr>>,
    screen: Signal<super::app::Screen>,
) -> Element {
    tracing::debug!("QrGenerator render: {:?}", ui.read());

    // Signal séparé pour l'image générée afin d'éviter les boucles infinies
    let mut qr_image = use_signal(String::new);

    // Fonction pour télécharger l'image QR
    let h_download_qr = {
        move || async move {
            let image_data = qr_image.read().clone();
            if !image_data.is_empty() {
                if let Some(window) = window() {
                    if let Some(document) = window.document() {
                        if let Ok(anchor) = document.create_element("a") {
                            if let Ok(anchor) = anchor.dyn_into::<HtmlElement>() {
                                let _ = anchor.set_attribute("href", &image_data);
                                let _ = anchor.set_attribute("download", "qr-code.png");
                                anchor.click();
                            }
                        }
                    }
                }
            }
        }
    };

    let h_save_qr = {
        move || async move {
            let ui = ui;
            let saved = saved;
            to_owned![ui, saved];

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

            let id = cur
                .editing_id
                .clone()
                .unwrap_or_else(|| format!("qr-{}", fastrand::u64(..)));

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
                v.editing_id = Some(id);
                ui.set(v);
            }
        }
    };

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
        div { class: "",
            article { class: "",

                div { class: "grid",
                    div { class: "s4",
                        if !qr_image.read().is_empty() {
                            div { class: "center-align padding bg-checkered",
                                img {
                                    class: "no-round min",
                                    style: "max-width: 100%; height: auto;",
                                    src: "{qr_image.read()}",
                                }
                            }
                        }
                    }
                    div { class: "s8 padding",
                        div { class: "row",
                            div { class: "field label border max",
                                input {
                                    r#type: "text",
                                    placeholder: " ",
                                    value: "{ui.read().text}",
                                    oninput: move |e| {
                                        let new_text = e.value();
                                        let mut v = (*ui.read()).clone();
                                        v.text = new_text;
                                        ui.set(v);
                                    },
                                }
                                label { class: "active", "Texte ou URL" }
                            }
                        }
                        div { class: "row",
                            div { class: "field label suffix border",
                                select {
                                    value: "{ui.read().size}",
                                    onchange: move |e| {
                                        if let Ok(s) = e.value().parse::<u32>() {
                                            let mut v = (*ui.read()).clone();
                                            v.size = s;
                                            ui.set(v);
                                        }
                                    },
                                    option { value: "128", "128px" }
                                    option { value: "256", "256px" }
                                    option { value: "512", "512px" }
                                }
                                label { class: "active", "Taille" }
                                i { "arrow_drop_down" }
                            }
                            label { class: "checkbox",
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
                            }
                            label { class: "checkbox",
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
                        }
                        div { class: "row",
                            button { onclick: move |_| { h_download_qr() }, "Télécharger le QR Code" }
                            button {
                                class: "circle secondary",
                                onclick: move |_| { h_save_qr() },
                                i { "bookmark" }
                                div { class: "tooltip", "Mettre en favoris le QR Code" }
                            }
                        }
                    }
                }
            }
        }
    }
}
