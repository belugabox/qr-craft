use dioxus::prelude::*;

mod qrcode;
use qrcode::generate_qr_code;

// Assets
static CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        div {
            class: "bg-gray-800 text-white min-h-screen flex flex-col items-center justify-center font-sans",
            
            div {
                class: "bg-gray-900 p-8 rounded-lg shadow-2xl w-full max-w-md",

                // Titre
                h1 {
                    class: "text-4xl font-bold mb-6 text-center text-teal-400",
                    "QR Craft"
                }

                // Intégrer le composant générateur
                QrGenerator {}
            }
        }
    }
}

#[component]
fn QrGenerator() -> Element {
    // Signal pour le texte à encoder
    let mut text_to_encode = use_signal(|| "https://example.com".to_string());
    // Signal pour la taille du QR code
    let mut qr_size = use_signal(|| 256u32);
    // Signal pour activer la transparence
    let mut transparent = use_signal(|| true);
    // Signal pour le chemin de l'image du QR code généré
    let mut qr_code_path = use_signal(String::new);
    // Signal pour gérer les erreurs
    let mut error_message = use_signal(String::new);

    rsx! {
        div {
            class: "flex flex-col gap-4",

            // Champ de saisie
            input {
                class: "bg-gray-700 border border-gray-600 rounded-md p-3 text-lg focus:outline-none focus:ring-2 focus:ring-teal-500",
                placeholder: "Entrez du texte ou une URL...",
                value: "{text_to_encode}",
                oninput: move |event| text_to_encode.set(event.value())
            }

            // Sélecteur de taille
            div {
                class: "flex flex-col gap-2",
                label {
                    class: "text-sm font-medium text-gray-300",
                    "Taille du QR Code:"
                }
                select {
                    class: "bg-gray-700 border border-gray-600 rounded-md p-3 text-lg focus:outline-none focus:ring-2 focus:ring-teal-500",
                    value: "{qr_size}",
                    onchange: move |event| {
                        if let Ok(size) = event.value().parse::<u32>() {
                            qr_size.set(size);
                        }
                    },
                    option { value: "128", "Petit (128x128)" }
                    option { value: "256", selected: true, "Moyen (256x256)" }
                    option { value: "512", "Grand (512x512)" }
                    option { value: "1024", "Très grand (1024x1024)" }
                }
            }

            // Bouton de génération
            button {
                class: "bg-teal-600 hover:bg-teal-700 text-white font-bold py-3 px-4 rounded-md text-lg transition-colors duration-300 disabled:bg-gray-500",
                disabled: text_to_encode.read().is_empty(),
                onclick: move |_| async move {
                    // Nettoyer les anciens messages d'erreur
                    error_message.set("".to_string());
                    qr_code_path.set("".to_string());

                    match generate_qr_code(text_to_encode.read().clone(), *qr_size.read(), *transparent.read()).await {
                        Ok(path) => qr_code_path.set(path),
                        Err(e) => error_message.set(format!("Erreur du serveur: {}", e)),
                    }
                },
                "Générer le QR Code"
            }

            // Option transparence
            div {
                class: "flex items-center gap-2",
                input {
                    r#type: "checkbox",
                    checked: "{transparent}",
                    onchange: move |event| {
                        let v = event.value();
                        // event.value() retourne "on" ou "" selon l'input; on peut parser
                        transparent.set(v == "on" || v == "true");
                    }
                }
                label { class: "text-sm text-gray-300", "Fond transparent" }
            }

            // Affichage de l'erreur
            if !error_message.read().is_empty() {
                p {
                    class: "text-red-500 text-center",
                    "{error_message}"
                }
            }

            // Zone d'affichage du QR Code
            if !qr_code_path.read().is_empty() {
                div {
                    class: "bg-white p-4 rounded-lg mt-4 flex flex-col items-center gap-3",
                    img {
                        class: match *qr_size.read() {
                            128 => "w-32 h-32",
                            256 => "w-64 h-64", 
                            512 => "w-80 h-80",
                            1024 => "w-96 h-96",
                            _ => "w-64 h-64"
                        },
                        src: "{qr_code_path}"
                    }
                    a {
                        class: "text-sm text-gray-600 hover:text-teal-600",
                        href: "{qr_code_path}",
                        download: "qr_code.png",
                        "Télécharger l\'image"
                    }
                }
            }
        }
    }
}
