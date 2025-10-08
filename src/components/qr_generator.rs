use crate::models::{LogoId, LogoRatio, SavedQr, UIQr};
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
    is_loading: Signal<bool>,
) -> Element {
    tracing::debug!("QrGenerator render: {:?}", ui.read());

    // Signal séparé pour l'image générée afin d'éviter les boucles infinies
    let mut qr_image = use_signal(String::new);

    // Fonction pour télécharger l'image QR
    let h_download_qr = {
        let mut is_loading = is_loading;
        move || async move {
            is_loading.set(true);
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
            is_loading.set(false);
        }
    };

    let h_save_qr = {
        let ui = ui;
        let saved = saved;
        let mut is_loading = is_loading;
        move || async move {
            is_loading.set(true);
            to_owned![ui, saved];

            let cur = (*ui.read()).clone();

            let image_data = qr_image.read().clone();
            if image_data.is_empty() {
                return;
            }

            let saved_q = SavedQr {
                id: cur.id.clone(),
                text: cur.text.clone(),
                size: cur.size,
                transparent: cur.transparent,
                margin: cur.margin,
                created_at: format!("{}", (Date::now() / 1000.0) as u64),
                image_data_url: image_data,
                logo_id: cur.logo_id,
                logo_ratio: cur.logo_ratio,
            };

            if save_qr(saved_q).await.is_ok() {
                if let Ok(list) = list_saved().await {
                    saved.set(list);
                }
                let v = (*ui.read()).clone();
                ui.set(v);
            }
            is_loading.set(false);
        }
    };

    // Effet pour générer automatiquement le QR code quand les paramètres changent
    use_effect(move || {
        let text = ui().text.clone();
        let size = ui().size;
        let transparent = ui().transparent;
        let margin = ui().margin;
        let logo_id = ui().logo_id.clone();
        let logo_ratio = ui().logo_ratio;

        if !text.is_empty() {
            spawn(async move {
                match generate_qr_code(text, size, transparent, margin, logo_id, logo_ratio).await {
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
                            div { class: "field suffix label border",
                                label { class: "active", "Logo" }
                                select {
                                    value: "{ui.read().logo_id.as_str()}",
                                    onchange: move |e| {
                                        let mut v = (*ui.read()).clone();
                                        v.logo_id = LogoId::from_str(&e.value());
                                        ui.set(v);
                                    },
                                    option { value: "none", "Aucun" }
                                    option { value: "facebook", "Facebook" }
                                    option { value: "whatsapp", "WhatsApp" }
                                    option { value: "facebook_color", "Facebook (coloré)" }
                                    option { value: "whatsapp_color", "WhatsApp (coloré)" }
                                    option { value: "instagram_color", "Instagram (coloré)" }
                                }
                                i { "arrow_drop_down" }
                            }
                            div { class: "field suffix label border",
                                label { class: "active", "Taille du logo" }
                                select {
                                    value: "{ui.read().logo_ratio.as_str()}",
                                    onchange: move |e| {
                                        let mut v = (*ui.read()).clone();
                                        v.logo_ratio = LogoRatio::from_str(&e.value());
                                        ui.set(v);
                                    },
                                    option { value: "small", "Petit" }
                                    option { value: "medium", "Moyen" }
                                    option { value: "large", "Grand" }
                                }
                                i { "arrow_drop_down" }
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
                                    checked: "{ui.read().margin}",
                                    onchange: move |e| {
                                        let mut v = (*ui.read()).clone();
                                        v.margin = e.value() == "on" || e.value() == "true";
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
                            button {
                                disabled: "{is_loading()}",
                                onclick: move |_| { h_download_qr() },
                                "Télécharger le QR Code"
                            }
                            button {
                                disabled: "{is_loading()}",
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
