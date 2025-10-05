use crate::qrcode::{generate_qr_code, list_saved, save_qr, SavedQr};
use crate::UIQr;
use dioxus::prelude::*;

#[component]
pub fn QrGenerator(ui: Signal<UIQr>, saved: Signal<Vec<String>>) -> Element {
    rsx! {
        div { class: "flex-1 p-8",
            h1 { class: "text-3xl mb-4", "QR Craft" }
            div { class: "space-y-3",
                input { class: "p-2 w-full bg-gray-800 rounded",
                    placeholder: "Text or URL...",
                    value: "{ui.read().text}",
                    oninput: move |e| {
                        let mut v = (*ui.read()).clone();
                        v.text = e.value();
                        ui.set(v);
                    }
                }

                div { class: "flex items-center gap-4",
                    select { class: "p-2 bg-gray-800 rounded",
                        value: "{ui.read().size}",
                        onchange: move |e| {
                            if let Ok(s) = e.value().parse::<u32>() {
                                let mut v = (*ui.read()).clone(); v.size = s; ui.set(v);
                            }
                        },
                        option { value: "128", "128" }
                        option { value: "256", "256" }
                        option { value: "512", "512" }
                    }

                    label { class: "flex items-center gap-2",
                        input { r#type: "checkbox", checked: "{ui.read().transparent}", onchange: move |e| {
                            let mut v = (*ui.read()).clone();
                            v.transparent = e.value() == "on" || e.value() == "true";
                            ui.set(v);
                        } }
                        span { "Transparent" }
                    }
                }

                div { class: "flex gap-2",
                    button { class: "p-2 bg-teal-600 rounded", onclick: move |_| {
                        let ui = ui.clone(); to_owned![ui];
                        async move {
                            let cur = (*ui.read()).clone();
                            match generate_qr_code(cur.text.clone(), cur.size, cur.transparent).await {
                                Ok(data_url) => {
                                    let mut v = (*ui.read()).clone(); v.image_data = data_url; ui.set(v);
                                }
                                Err(e) => {
                                    eprintln!("generate error: {}", e);
                                }
                            }
                        }
                    }, "Generate" }

                    button { class: "p-2 bg-gray-200 text-black rounded", onclick: move |_| {
                        let ui = ui.clone(); let saved = saved.clone(); to_owned![ui, saved];
                        async move {
                            let cur = (*ui.read()).clone();
                            if cur.image_data.is_empty() { return; }
                            let base64 = cur.image_data.splitn(2, ',').nth(1).unwrap_or(&cur.image_data).to_string();
                            let id = format!("qr-{}", fastrand::u64(..));
                            let saved_q = SavedQr { id: id.clone(), text: cur.text.clone(), size: cur.size, transparent: cur.transparent, image_base64: base64 };
                            if let Ok(_) = save_qr(saved_q).await {
                                if let Ok(list) = list_saved().await { saved.set(list); }
                            }
                        }
                    }, "Save" }
                }

                if !ui.read().image_data.is_empty() {
                    div { class: "mt-4 bg-white p-3 rounded text-black",
                        img { src: "{ui.read().image_data}" }
                    }
                }
            }
        }
    }
}
