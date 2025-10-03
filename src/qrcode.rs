// We avoid importing the whole dioxus prelude here to keep the pure rendering
// function free of wasm/js bindings so it can be tested natively.

/// Render a QR code into PNG bytes (pure function, no Dioxus types).
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
    let sample_pixel = image.get_pixel(0, 0);
    if sample_pixel.0.len() >= 4 {
        let rgba8: image::RgbaImage = image::ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
            let p = image.get_pixel(x, y);
            let r = (p[0] as i32).clamp(0, 255) as u8;
            let g = (p[1] as i32).clamp(0, 255) as u8;
            let b = (p[2] as i32).clamp(0, 255) as u8;
            let a = (p[3] as i32).clamp(0, 255) as u8;
            image::Rgba([r, g, b, a])
        });
        image::DynamicImage::ImageRgba8(rgba8)
            .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
            .map_err(|e| e.to_string())?;
    } else {
        let rgb8: image::RgbImage = image::ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
            let p = image.get_pixel(x, y);
            let r = (p[0] as i32).clamp(0, 255) as u8;
            let g = (p[1] as i32).clamp(0, 255) as u8;
            let b = (p[2] as i32).clamp(0, 255) as u8;
            image::Rgb([r, g, b])
        });
        image::DynamicImage::ImageRgb8(rgb8)
            .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
            .map_err(|e| e.to_string())?;
    }

    Ok(buffer)
}

// Now provide the server wrapper using Dioxus types. Keep the server wrapper here
// to preserve the existing API, but implement it by calling the pure function.
use dioxus::prelude::*;

#[server(GenerateQrCode)]
pub async fn generate_qr_code(text: String, size: u32, transparent: bool) -> Result<String, ServerFnError> {
    let bytes = render_qr_png_bytes(&text, size, transparent).map_err(|e| ServerFnError::new(e))?;
    let base64_image = base64::encode(&bytes);
    let data_url = format!("data:image/png;base64,{}", base64_image);
    Ok(data_url)
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
}
