/// Render a QR code into PNG bytes.
use image::ImageEncoder;
use dioxus::prelude::*;

#[server(GenerateQrCode)]
pub async fn generate_qr_code(text: String, size: u32, transparent: bool) -> Result<String, ServerFnError> {
    let bytes = render_qr_png_bytes(&text, size, transparent).map_err(|e| ServerFnError::new(e))?;
    let base64_image = base64::encode(&bytes);
    let data_url = format!("data:image/png;base64,{}", base64_image);
    Ok(data_url)
}

pub fn render_qr_png_bytes(text: &str, size: u32, transparent: bool) -> Result<Vec<u8>, String> {
    if text.is_empty() {
        return Err("Le texte ne peut pas être vide.".into());
    }

    let image = qrcode::QrCode::new(text.as_bytes()).map_err(|e| e.to_string())?
                   .render()
                   .dark_color(image::Rgba([0, 0, 0, 255]))
                   .light_color(image::Rgba([255, 255, 255, if transparent { 0 } else { 255 }]))
                   .quiet_zone(false)
                   .min_dimensions(size, size)
                   .build();

    let mut buffer = Vec::new();
    let width = image.width();
    let height = image.height();

    // Consume the image and get the raw container. For the qrcode crate the
    // subpixel type is u8 so this should be Vec<u8>, enabling zero-copy encoding.
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

    let color_type = match pixels {
        4 => image::ColorType::Rgba8,
        3 => image::ColorType::Rgb8,
        1 => image::ColorType::L8,
        _ => return Err("unsupported number of channels".into()),
    };

    let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
    encoder
        .write_image(&raw, width, height, color_type.into())
        .map_err(|e| e.to_string())?;

    Ok(buffer)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_render_qr_png_bytes_basic() {
        let bytes = render_qr_png_bytes("hello", 128, false).expect("render failed");
        // PNG magic bytes: 89 50 4E 47 0D 0A 1A 0A
        let png_magic = [0x89u8, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        assert!(bytes.len() >= png_magic.len());
        assert_eq!(&bytes[0..8], &png_magic);
    }

    #[test]
    fn test_render_qr_png_bytes_transparent() {
        let bytes = render_qr_png_bytes("transparent", 128, true).expect("render failed");
        let png_magic = [0x89u8, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(&bytes[0..8], &png_magic);
    }

    #[test]
    fn test_qrcode_build_type_name() {
        let code = qrcode::QrCode::new(b"t").unwrap();
        let image = code.render()
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
