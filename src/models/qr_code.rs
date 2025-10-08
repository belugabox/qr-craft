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

/// Type de logo disponible
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum LogoId {
    #[default]
    None,
    Facebook,
    FacebookColor,
    WhatsApp,
    WhatsAppColor,
    InstagramColor,
}

impl LogoId {
    /// Convertit l'enum en nom de fichier (sans extension)
    pub fn as_filename(&self) -> Option<&str> {
        match self {
            LogoId::None => None,
            LogoId::Facebook => Some("facebook"),
            LogoId::FacebookColor => Some("facebook_color"),
            LogoId::WhatsApp => Some("whatsapp"),
            LogoId::WhatsAppColor => Some("whatsapp_color"),
            LogoId::InstagramColor => Some("instagram_color"),
        }
    }

    /// Convertit depuis un nom de fichier (sans extension)
    pub fn from_filename(filename: &str) -> Option<Self> {
        match filename {
            "facebook" => Some(LogoId::Facebook),
            "whatsapp" => Some(LogoId::WhatsApp),
            "facebook_color" => Some(LogoId::FacebookColor),
            "whatsapp_color" => Some(LogoId::WhatsAppColor),
            "instagram_color" => Some(LogoId::InstagramColor),
            _ => None,
        }
    }

    /// Convertit en valeur pour le select HTML
    pub fn as_select_value(&self) -> &str {
        match self {
            LogoId::None => "",
            LogoId::Facebook => "facebook",
            LogoId::WhatsApp => "whatsapp",
            LogoId::FacebookColor => "facebook_color",
            LogoId::WhatsAppColor => "whatsapp_color",
            LogoId::InstagramColor => "instagram_color",
        }
    }

    /// Convertit depuis une valeur de select HTML
    pub fn from_select_value(value: &str) -> Self {
        match value {
            "facebook" => LogoId::Facebook,
            "whatsapp" => LogoId::WhatsApp,
            "facebook_color" => LogoId::FacebookColor,
            "whatsapp_color" => LogoId::WhatsAppColor,
            "instagram_color" => LogoId::InstagramColor,
            _ => LogoId::None,
        }
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
