use dioxus::prelude::*;
/// Render a QR code into PNG bytes.
use image::ImageEncoder;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::path::Path;

use crate::models::qr_code::{MarginEnabled, SavedQr};
use image::GenericImageView;

#[server(GenerateQrCode)]
pub async fn generate_qr_code(
    text: String,
    size: u32,
    transparent: bool,
    margin: MarginEnabled,
    logo_data_url: Option<String>,
    logo_ratio: Option<f64>,
) -> Result<String, ServerFnError> {
    // Si un data URL de logo est fourni, on le décode en bytes (seulement base64 supporté)
    let logo_bytes_opt: Option<Vec<u8>> = if let Some(data_url) = logo_data_url {
        match data_url.split_once(',') {
            Some((meta, payload)) if meta.contains("base64") => match base64::decode(payload) {
                Ok(buf) => Some(buf),
                Err(e) => return Err(ServerFnError::new(format!("invalid logo base64: {}", e))),
            },
            _ => return Err(ServerFnError::new("logo data url must be base64 encoded")),
        }
    } else {
        None
    };

    let logo_slice = logo_bytes_opt.as_deref();

    let ratio = logo_ratio.unwrap_or(0.20);

    let bytes = render_qr_png_bytes(&text, size, transparent, margin, logo_slice, ratio)
        .map_err(|e| ServerFnError::new(e))?;
    let base64_image = base64::encode(&bytes);
    let data_url = format!("data:image/png;base64,{}", base64_image);
    Ok(data_url)
}

#[server(SaveQr)]
pub async fn save_qr(qr: SavedQr) -> Result<String, ServerFnError> {
    // ensure data dir
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    let filename = format!("data/{}.json", qr.id);
    let json = serde_json::to_string_pretty(&qr).map_err(|e| ServerFnError::new(e.to_string()))?;
    fs::write(&filename, json).map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(filename)
}

#[server(ListSaved)]
pub async fn list_saved() -> Result<Vec<SavedQr>, ServerFnError> {
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        return Ok(vec![]);
    }
    let mut res = vec![];
    for entry in fs::read_dir(data_dir).map_err(|e| ServerFnError::new(e.to_string()))? {
        let entry = entry.map_err(|e| ServerFnError::new(e.to_string()))?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "json" {
                match fs::read_to_string(&path) {
                    Ok(s) => match serde_json::from_str::<SavedQr>(&s) {
                        Ok(qr) => res.push(qr),
                        Err(e) => {
                            // Log l'erreur mais continue avec les autres fichiers
                            eprintln!("Erreur de désérialisation du fichier {:?}: {}", path, e);
                        }
                    },
                    Err(e) => {
                        // Log l'erreur mais continue
                        eprintln!("Erreur de lecture du fichier {:?}: {}", path, e);
                    }
                }
            }
        }
    }
    Ok(res)
}

#[server(LoadSaved)]
pub async fn load_saved(filename: String) -> Result<SavedQr, ServerFnError> {
    let path = Path::new("data").join(filename);
    let s = fs::read_to_string(&path).map_err(|e| ServerFnError::new(e.to_string()))?;
    let qr: SavedQr = serde_json::from_str(&s).map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(qr)
}

#[server(DeleteSaved)]
pub async fn delete_saved(filename: String) -> Result<(), ServerFnError> {
    let path = Path::new("data").join(format!("{}.json", filename));
    if !path.exists() {
        return Err(ServerFnError::new(format!(
            "Le fichier {} n'existe pas",
            filename
        )));
    }
    fs::remove_file(&path).map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[allow(dead_code)]
pub fn render_qr_png_bytes(
    text: &str,
    size: u32,
    transparent: bool,
    margin: MarginEnabled,
    logo_png: Option<&[u8]>, // nouvel argument optionnel : bytes PNG/JPEG du logo
    logo_ratio: f64,         // fraction de la largeur du QR (0.2 = 20%)
) -> Result<Vec<u8>, String> {
    if text.is_empty() {
        return Err("Le texte ne peut pas être vide.".into());
    }

    let image = qrcode::QrCode::new(text.as_bytes())
        .map_err(|e| e.to_string())?
        .render()
        .dark_color(image::Rgba([0, 0, 0, 255]))
        .light_color(image::Rgba([
            255,
            255,
            255,
            if transparent { 0 } else { 255 },
        ]))
        .quiet_zone(margin.0) // Utilise directement le boolean
        .min_dimensions(size, size)
        .build();

    let width = image.width();
    let height = image.height();

    // Consume the image and get the raw container.
    let raw = image.into_raw();

    // Determine channels per pixel (must divide evenly)
    let total = raw.len() as u32;
    if width == 0 || height == 0 {
        return Err("invalid image dimensions".into());
    }
    let pixels = total / (width * height);
    if pixels == 0 || (width * height * pixels) != total {
        return Err("unexpected raw buffer length".into());
    }

    // Rebuild une image RGBA mut (pour overlay facile)
    let mut base_rgba: image::RgbaImage = match pixels {
        4 => image::RgbaImage::from_raw(width, height, raw)
            .ok_or("failed to construct RGBA image from raw")?,
        3 => {
            let rgb = image::RgbImage::from_raw(width, height, raw)
                .ok_or("failed to construct RGB image from raw")?;
            image::DynamicImage::ImageRgb8(rgb).to_rgba8()
        }
        1 => {
            let gray = image::GrayImage::from_raw(width, height, raw)
                .ok_or("failed to construct Gray image from raw")?;
            image::DynamicImage::ImageLuma8(gray).to_rgba8()
        }
        _ => return Err("unsupported number of channels".into()),
    };

    // Si on a un logo, le charger, redimensionner et le dessiner centré
    if let Some(logo_bytes) = logo_png {
        // Protéger logo_ratio et éviter dimension 0
        let ratio = if logo_ratio > 0.0 && logo_ratio < 1.0 {
            logo_ratio
        } else {
            0.2
        };

        // Charger le logo depuis les bytes (PNG/JPEG supportés)
        let logo_img = image::load_from_memory(logo_bytes).map_err(|e| e.to_string())?;
        let (lw, lh) = logo_img.dimensions();
        if lw == 0 || lh == 0 {
            return Err("logo invalid dimensions".into());
        }

        // Calculer taille du logo en maintenant le ratio et l'aspect
        let logo_w = ((width as f64) * ratio).max(1.0).round() as u32;
        let logo_h = ((logo_w as f64) * (lh as f64) / (lw as f64))
            .max(1.0)
            .round() as u32;

        // Redimensionner le logo avec un filtre de qualité
        let logo_resized = logo_img.resize(logo_w, logo_h, image::imageops::FilterType::Lanczos3);
        let logo_rgba = logo_resized.to_rgba8();

        // Positionner centré
        let x = (width.saturating_sub(logo_w)) / 2;
        let y = (height.saturating_sub(logo_h)) / 2;

        // Optionnel : dessiner un fond blanc arrondi derrière le logo pour garantir contraste
        {
            // Créer une petite ellipse blanche légèrement plus grande que le logo
            let pad = (logo_w.max(logo_h) as f32 * 0.12).round() as i32;
            let bg_w = (logo_w as i32 + pad).max(1) as u32;
            let bg_h = (logo_h as i32 + pad).max(1) as u32;
            let bg_x = x.saturating_sub((pad / 2) as u32);
            let bg_y = y.saturating_sub((pad / 2) as u32);

            // Dessiner un rectangle arrondi simple (approx par ellipse) en blanc
            // Ici on utilise un remplissage d'ellipse centré
            let bg = image::RgbaImage::from_pixel(bg_w, bg_h, image::Rgba([255, 255, 255, 255]));
            // Optionnel : adoucir bords, mais pour simplicité on colle un rectangle blanc
            image::imageops::overlay(&mut base_rgba, &bg, bg_x as i64, bg_y as i64);
        }

        // Enfin overlay du logo (respecte l'alpha)
        image::imageops::overlay(&mut base_rgba, &logo_rgba, x as i64, y as i64);
    }

    // Encoder en PNG (RGBA8)
    let mut buffer = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
    // consommer base_rgba pour récupérer les octets
    let raw_out = base_rgba.into_raw();
    encoder
        .write_image(&raw_out, width, height, image::ColorType::Rgba8.into())
        .map_err(|e| e.to_string())?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_qr_png_bytes_basic() {
        let bytes = render_qr_png_bytes("hello", 128, false, MarginEnabled(true), None, 0.20)
            .expect("render failed");
        // PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
        let png_magic = [0x89u8, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        assert!(bytes.len() >= png_magic.len());
        assert_eq!(&bytes[0..8], &png_magic);
    }

    #[test]
    fn test_render_qr_png_bytes_transparent() {
        let bytes = render_qr_png_bytes("transparent", 128, true, MarginEnabled(true), None, 0.20)
            .expect("render failed");
        let png_magic = [0x89u8, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(&bytes[0..8], &png_magic);
    }

    #[test]
    fn test_qrcode_build_type_name() {
        let code = qrcode::QrCode::new(b"t").unwrap();
        let image = code
            .render()
            .dark_color(image::Rgba([0u8, 0u8, 0u8, 255u8]))
            .light_color(image::Rgba([255u8, 255u8, 255u8, 255u8]))
            .quiet_zone(false)
            .min_dimensions(16, 16)
            .build();
        let t = std::any::type_name_of_val(&image);
        // Print the type name to help decide fast-path
        println!("qrcode build type: {}", t);
        // At minimum ensure we got something
        assert!(!t.is_empty());
    }

    // bench test removed
}
