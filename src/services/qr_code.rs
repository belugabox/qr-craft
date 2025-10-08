use dioxus::prelude::*;
/// Render a QR code into PNG bytes.
use image::ImageEncoder;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::path::Path;

use crate::models::qr_code::{LogoId, MarginEnabled, SavedQr};
use image::GenericImageView;

// Helpers for SVG rasterization
fn is_svg_bytes(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }
    if let Ok(s) = std::str::from_utf8(&bytes[..bytes.len().min(256)]) {
        let s = s.trim_start();
        return s.starts_with('<')
            && (s.contains("<svg") || s.starts_with("<?xml") || s.starts_with("<svg"));
    }
    false
}

fn rasterize_svg_to_rgba(
    svg_bytes: &[u8],
    target_size: u32,
    logo_id: &LogoId,
) -> Result<image::RgbaImage, String> {
    use resvg::tiny_skia::Pixmap;
    use resvg::usvg::{FitTo, ImageRendering, Options, ShapeRendering, TextRendering, Tree};

    let svg_str = std::str::from_utf8(svg_bytes).map_err(|e| e.to_string())?;

    // Remplacer currentColor par la couleur appropriée selon le logo
    let processed_svg = match logo_id {
        LogoId::Facebook => svg_str.replace("currentColor", "#000000"), // Noir pour Facebook monochrome
        LogoId::FacebookColor => svg_str.replace("currentColor", "#1877F2"), // Bleu Facebook officiel
        LogoId::WhatsApp => svg_str.replace("currentColor", "#000000"), // Noir pour WhatsApp monochrome
        LogoId::WhatsAppColor => svg_str.replace("currentColor", "#25D366"), // Vert WhatsApp officiel
        LogoId::InstagramColor => svg_str.replace("currentColor", "#E4405F"), // Rose Instagram officiel
        LogoId::None => svg_str.replace("currentColor", "#000000"),           // Noir par défaut
    };

    // Configurer les options pour une qualité maximale
    let mut opt = Options::default();
    opt.shape_rendering = ShapeRendering::CrispEdges;
    opt.text_rendering = TextRendering::OptimizeSpeed; // ou OptimizeLegibility
    opt.image_rendering = ImageRendering::OptimizeQuality;

    let tree = Tree::from_str(&processed_svg, &opt).map_err(|e| e.to_string())?;

    // Utiliser un supersampling pour une meilleure qualité
    // Rendre à 2x la taille puis réduire pour un meilleur anti-aliasing
    let render_size = target_size * 2;

    // Créer un Pixmap aux dimensions de rendu (supersampled)
    let mut pixmap = Pixmap::new(render_size, render_size).ok_or("failed to create pixmap")?;

    // Rendre le SVG en l'ajustant aux dimensions de rendu
    resvg::render(
        &tree,
        FitTo::Width(render_size),
        Default::default(),
        pixmap.as_mut(),
    )
    .ok_or("failed to render svg")?;

    // Convertir tiny-skia Pixmap (BGRA) en image::RgbaImage (RGBA)
    let data = pixmap.data();
    let mut high_res = Vec::with_capacity((render_size * render_size * 4) as usize);
    for chunk in data.chunks_exact(4) {
        let b = chunk[0];
        let g = chunk[1];
        let r = chunk[2];
        let a = chunk[3];
        high_res.push(r);
        high_res.push(g);
        high_res.push(b);
        high_res.push(a);
    }

    // Créer l'image haute résolution
    let high_res_img = image::RgbaImage::from_raw(render_size, render_size, high_res)
        .ok_or("failed to create high-res RgbaImage from svg raster")?;

    // Redimensionner vers la taille cible avec un excellent filtre
    let final_img = image::DynamicImage::ImageRgba8(high_res_img).resize(
        target_size,
        target_size,
        image::imageops::FilterType::Lanczos3,
    );

    Ok(final_img.to_rgba8())
}

#[server(GenerateQrCode)]
pub async fn generate_qr_code(
    text: String,
    size: u32,
    transparent: bool,
    margin: MarginEnabled,
    logo_id: LogoId,
    logo_ratio: f64,
) -> Result<String, ServerFnError> {
    // Charger le fichier SVG depuis assets/logo si un logo est sélectionné
    let logo_bytes_opt: Option<Vec<u8>> = if let Some(filename) = logo_id.as_filename() {
        let logo_path = format!("assets/logo/{}.svg", filename);
        match fs::read(&logo_path) {
            Ok(bytes) => Some(bytes),
            Err(e) => {
                return Err(ServerFnError::new(format!(
                    "failed to read logo file {}: {}",
                    logo_path, e
                )))
            }
        }
    } else {
        None
    };
    let logo_slice = logo_bytes_opt.as_deref();

    let bytes = render_qr_png_bytes(
        &text,
        size,
        transparent,
        margin,
        logo_slice,
        Some(logo_id),
        logo_ratio,
    )
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
                        Ok(mut qr) => {
                            res.push(qr);
                        }
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

pub fn render_qr_png_bytes(
    text: &str,
    size: u32,
    transparent: bool,
    margin: MarginEnabled,
    logo_bytes: Option<&[u8]>, // optionnel : bytes du logo (SVG ou raster)
    logo_id: Option<LogoId>,   // optionnel : type de logo pour la couleur SVG
    logo_ratio: f64,           // fraction de la largeur du QR (0.2 = 20%)
) -> Result<Vec<u8>, String> {
    if text.is_empty() {
        return Err("Le texte ne peut pas être vide.".into());
    }

    let image = qrcode::QrCode::with_error_correction_level(text.as_bytes(), qrcode::EcLevel::H)
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

    // Si on a un logo, le charger (PNG/JPEG) ou rasteriser l'SVG, redimensionner et le dessiner centré
    if let Some(logo_bytes) = logo_bytes {
        // Protéger logo_ratio et éviter dimension 0
        let ratio = if logo_ratio > 0.0 && logo_ratio < 1.0 {
            logo_ratio
        } else {
            0.2
        };

        // Charger le logo depuis les bytes (PNG/JPEG ou SVG)
        let logo_img: image::DynamicImage = if is_svg_bytes(logo_bytes) {
            // Calculer la taille cible du logo pour rasteriser le SVG directement
            let logo_w = ((width as f64) * ratio).max(1.0).round() as u32;
            // Rasteriser l'SVG directement à la taille cible pour éviter le redimensionnement
            let rgba_img = rasterize_svg_to_rgba(
                logo_bytes,
                logo_w,
                logo_id.as_ref().unwrap_or(&LogoId::None),
            )?;
            image::DynamicImage::ImageRgba8(rgba_img)
        } else {
            // Charger PNG/JPEG normalement
            image::load_from_memory(logo_bytes).map_err(|e| e.to_string())?
        };
        let (lw, lh) = logo_img.dimensions();
        if lw == 0 || lh == 0 {
            return Err("logo invalid dimensions".into());
        }

        // Calculer taille du logo en maintenant le ratio et l'aspect
        let logo_w = ((width as f64) * ratio).max(1.0).round() as u32;
        let logo_h = ((logo_w as f64) * (lh as f64) / (lw as f64))
            .max(1.0)
            .round() as u32;

        // Pour les SVGs déjà rasterisés à la bonne taille, pas besoin de redimensionner
        // Pour les PNG/JPEG, redimensionner si nécessaire avec le meilleur filtre
        let logo_rgba = if is_svg_bytes(logo_bytes) && lw == logo_w && lh == logo_h {
            // SVG déjà exactement à la bonne taille
            logo_img.to_rgba8()
        } else {
            // Redimensionner avec le filtre de meilleure qualité
            logo_img
                .resize_exact(logo_w, logo_h, image::imageops::FilterType::Lanczos3)
                .to_rgba8()
        };

        // Positionner centré
        let x = (width.saturating_sub(logo_w)) / 2;
        let y = (height.saturating_sub(logo_h)) / 2;

        // Améliorer le fond blanc : plus subtil et mieux adapté
        {
            // Créer un fond légèrement plus grand que le logo avec des bords arrondis
            let pad = (logo_w.max(logo_h) as f32 * 0.08).max(2.0).round() as i32;
            let bg_w = (logo_w as i32 + pad).max(1) as u32;
            let bg_h = (logo_h as i32 + pad).max(1) as u32;
            let bg_x = x.saturating_sub((pad / 2) as u32);
            let bg_y = y.saturating_sub((pad / 2) as u32);

            // Utiliser un blanc légèrement transparent pour un meilleur contraste
            let bg_color = image::Rgba([255, 255, 255, 240]); // Légèrement transparent
            let bg = image::RgbaImage::from_pixel(bg_w, bg_h, bg_color);
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
        let bytes = render_qr_png_bytes("hello", 128, false, MarginEnabled(true), None, None, 0.20)
            .expect("render failed");
        // PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
        let png_magic = [0x89u8, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        assert!(bytes.len() >= png_magic.len());
        assert_eq!(&bytes[0..8], &png_magic);
    }

    #[test]
    fn test_render_qr_png_bytes_transparent() {
        let bytes = render_qr_png_bytes(
            "transparent",
            128,
            true,
            MarginEnabled(true),
            None,
            None,
            0.20,
        )
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

    #[test]
    fn test_rasterize_svg_to_rgba() {
        let svg_content = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12.001 2C6.47813 2 2.00098 6.47715 2.00098 12C2.00098 16.9913 5.65783 21.1283 10.4385 21.8785V14.8906H7.89941V12H10.4385V9.79688C10.4385 7.29063 11.9314 5.90625 14.2156 5.90625C15.3097 5.90625 16.4541 6.10156 16.4541 6.10156V8.5625H15.1931C13.9509 8.5625 13.5635 9.33334 13.5635 10.1242V12H16.3369L15.8936 14.8906H13.5635V21.8785C18.3441 21.1283 22.001 16.9913 22.001 12C22.001 6.47715 17.5238 2 12.001 2Z"></path></svg>"#;
        let svg_bytes = svg_content.as_bytes();
        let result = rasterize_svg_to_rgba(svg_bytes, 64, &LogoId::None); // Test avec une taille de 64px
        assert!(
            result.is_ok(),
            "Failed to rasterize SVG: {:?}",
            result.err()
        );
        let img = result.unwrap();
        assert_eq!(img.width(), 64);
        assert_eq!(img.height(), 64);
        // Vérifier que l'image n'est pas vide (au moins quelques pixels non transparents)
        let has_content = img.pixels().any(|p| p[3] > 0);
        assert!(has_content, "SVG rasterization produced empty image");
    }

    #[test]
    fn test_render_qr_with_svg_logo() {
        use std::fs;
        // Charger un SVG réel depuis assets/logo
        let svg_path = "assets/logo/facebook.svg";
        let svg_bytes = fs::read(svg_path).expect("Failed to read SVG file");

        // Générer un QR avec ce logo SVG
        let result = render_qr_png_bytes(
            "test with SVG logo",
            256,
            false,
            MarginEnabled(true),
            Some(&svg_bytes),
            None, // Pas de logo_id connu pour ce test
            0.15,
        );
        assert!(
            result.is_ok(),
            "Failed to render QR with SVG logo: {:?}",
            result.err()
        );

        let png_bytes = result.unwrap();
        // Vérifier que c'est un PNG valide
        let png_magic = [0x89u8, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        assert!(png_bytes.len() >= png_magic.len());
        assert_eq!(&png_bytes[0..8], &png_magic);

        // Le PNG devrait être plus grand qu'un QR sans logo (car il contient le logo)
        let empty_qr =
            render_qr_png_bytes("test", 256, false, MarginEnabled(true), None, None, 0.15).unwrap();
        assert!(
            png_bytes.len() > empty_qr.len(),
            "QR with logo should be larger than without"
        );
    }

    #[test]
    fn test_generate_qr_code_with_logo_id() {
        // Test de la fonction generate_qr_code avec logo_id
        // Note: Cette fonction est async et serveur, donc on teste seulement la logique de base
        // Le test complet nécessiterait un environnement serveur

        // Vérifier que les fichiers existent
        assert!(std::path::Path::new("assets/logo/facebook.svg").exists());
        assert!(std::path::Path::new("assets/logo/whatsapp.svg").exists());
    }

    #[test]
    fn test_models_serialization() {
        use crate::models::qr_code::{LogoId, MarginEnabled, SavedQr, UIQr};

        // Test UIQr avec logo_id
        let _ui_qr = UIQr {
            id: "test-id".to_string(),
            text: "test text".to_string(),
            size: 256,
            transparent: false,
            margin: MarginEnabled(true),
            logo_id: LogoId::Facebook,
            logo_ratio: 0.2,
        };

        // Test SavedQr avec logo_id
        let saved_qr = SavedQr {
            id: "test-id".to_string(),
            text: "test text".to_string(),
            size: 256,
            transparent: false,
            margin: MarginEnabled(true),
            created_at: "1234567890".to_string(),
            image_data_url: "data:image/png;base64,test".to_string(),
            logo_id: LogoId::WhatsApp,
            logo_ratio: 0.15,
        };

        // Test sérialisation/désérialisation
        let serialized = serde_json::to_string(&saved_qr).unwrap();
        let deserialized: SavedQr = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.logo_id, LogoId::WhatsApp);
        assert_eq!(deserialized.logo_ratio, 0.15);
    }
}
