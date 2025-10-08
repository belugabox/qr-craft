use serde::{Deserialize, Serialize};

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
