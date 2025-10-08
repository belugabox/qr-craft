//! Modèles de données pour les QR codes

use serde::{Deserialize, Serialize};

use crate::config::constants;
use crate::models::logo::{LogoId, LogoRatio};

/// Configuration d'un QR code pour l'interface utilisateur
#[derive(Clone, Debug)]
pub struct UIQr {
    pub id: String,
    pub text: String,
    pub size: u32,
    pub transparent: bool,
    pub margin: bool,
    // Type de logo sélectionné
    pub logo_id: LogoId,
    // Optional logo size ratio (fraction of QR width)
    pub logo_ratio: LogoRatio,
}

impl Default for UIQr {
    fn default() -> Self {
        UIQr {
            id: format!("qr-{}", fastrand::u64(..)),
            text: constants::DEFAULT_QR_TEXT.into(),
            size: 256,
            transparent: false,
            margin: true,
            logo_id: LogoId::default(),
            logo_ratio: LogoRatio::default(),
        }
    }
}

/// Représentation d'un QR code sauvegardé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedQr {
    /// Identifiant unique du QR code
    pub id: String,
    /// Texte encodé dans le QR code
    pub text: String,
    /// Taille du QR code en pixels
    pub size: u32,
    /// Si le fond est transparent
    pub transparent: bool,
    /// Si une marge est activée
    pub margin: bool,
    /// Date de création (format ISO 8601)
    pub created_at: String,
    /// URL des données de l'image (souvent base64)
    pub image_data_url: String,
    /// Type de logo intégré
    pub logo_id: LogoId,
    /// Ratio de taille du logo
    pub logo_ratio: LogoRatio,
}
