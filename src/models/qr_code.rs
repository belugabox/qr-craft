//! Modèles de données pour les QR codes

use serde::{Deserialize, Serialize};

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
    pub image_data: String,
}
