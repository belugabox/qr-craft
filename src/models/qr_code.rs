//! Modèles de données pour les QR codes

use serde::{Deserialize, Serialize};

use crate::models::logo::LogoId;

/// Activer/désactiver la marge autour du QR code
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarginEnabled(pub bool);

impl Default for MarginEnabled {
    fn default() -> Self {
        MarginEnabled(true)
    }
}

/// Configuration d'un QR code pour l'interface utilisateur
#[derive(Clone, Default, Debug)]
pub struct UIQr {
    pub id: String,
    pub text: String,
    pub size: u32,
    pub transparent: bool,
    pub margin: MarginEnabled,
    // Type de logo sélectionné
    pub logo_id: LogoId,
    // Optional logo size ratio (fraction of QR width)
    pub logo_ratio: f64,
}

/// Représentation d'un QR code sauvegardé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedQr {
    pub id: String,
    pub text: String,
    pub size: u32,
    pub transparent: bool,
    pub margin: MarginEnabled,
    pub created_at: String,
    pub image_data_url: String,
    // Nouveau format
    #[serde(default)]
    pub logo_id: LogoId,
    #[serde(default)]
    pub logo_ratio: f64,
}
