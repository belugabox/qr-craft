use dioxus::prelude::*;

// Assets
const TAILWIND_CSS: Asset = asset!("./assets/tailwind.css");

/// La fonction principale qui s'exécute sur le serveur pour générer le QR code.
#[server(GenerateQrCode)]
async fn generate_qr_code(text: String) -> Result<String, ServerFnError> {
    // Vérifier que le texte n'est pas vide
    if text.is_empty() {
        return Err(ServerFnError::new("Le texte ne peut pas être vide."));
    }

    // Générer les données du QR code
    let code = qrcode::QrCode::new(text.as_bytes()).map_err(|e| ServerFnError::new(e.to_string()))?;

    // Rendre le QR code en une image
    let image = code.into_image();

    // Définir le chemin de sauvegarde dans le dossier des assets
    // Le préfixe "." est important pour que Dioxus le reconnaisse comme un chemin local au projet
    let file_path = "./assets/qr_code.png";

    // Sauvegarder l'image
    image.save(file_path).map_err(|e| ServerFnError::new(e.to_string()))?;

    // Renvoyer le chemin public de l'image. 
    // On ajoute un paramètre aléatoire pour forcer le navigateur à recharger l'image.
    let public_path = format!("/assets/qr_code.png?v={}", fastrand::u32(..));

    Ok(public_path)
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

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
    let mut text_to_encode = use_signal(String::new);
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

            // Bouton de génération
            button {
                class: "bg-teal-600 hover:bg-teal-700 text-white font-bold py-3 px-4 rounded-md text-lg transition-colors duration-300 disabled:bg-gray-500",
                disabled: text_to_encode.read().is_empty(),
                onclick: move |_| async move {
                    // Nettoyer les anciens messages d'erreur
                    error_message.set("".to_string());
                    qr_code_path.set("".to_string());

                    match generate_qr_code(text_to_encode.read().clone()).await {
                        Ok(path) => qr_code_path.set(path),
                        Err(e) => error_message.set(format!("Erreur du serveur: {}", e)),
                    }
                },
                "Générer le QR Code"
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
                        class: "w-64 h-64",
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
