//! Modèles de données pour les QR codes

use serde::{Deserialize, Serialize};

/// Configuration d'un QR code pour l'interface utilisateur
#[derive(Clone, Default, Debug)]
pub struct UIQr {
    pub text: String,
    pub size: u32,
    pub transparent: bool,
    pub image_data: String,
}

/// Représentation d'un QR code sauvegardé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedQr {
    pub id: String,
    pub text: String,
    pub size: u32,
    pub transparent: bool,
    pub created_at: String,
    pub image_data: String,
}
