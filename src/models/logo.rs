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
    /// Retourne la chaîne de caractères associée au logo
    pub fn as_str(&self) -> &str {
        match self {
            LogoId::None => "none",
            LogoId::Facebook => "facebook",
            LogoId::FacebookColor => "facebook_color",
            LogoId::WhatsApp => "whatsapp",
            LogoId::WhatsAppColor => "whatsapp_color",
            LogoId::InstagramColor => "instagram_color",
        }
    }

    /// Convertit depuis une chaîne de caractères
    pub fn from_str(s: &str) -> Self {
        match s {
            "none" => LogoId::None,
            "facebook" => LogoId::Facebook,
            "whatsapp" => LogoId::WhatsApp,
            "facebook_color" => LogoId::FacebookColor,
            "whatsapp_color" => LogoId::WhatsAppColor,
            "instagram_color" => LogoId::InstagramColor,
            _ => LogoId::default(),
        }
    }

    /// Convertit l'enum en nom de fichier (sans extension)
    pub fn as_filename(&self) -> Option<&str> {
        match self {
            LogoId::None => None,
            _ => Some(self.as_str()),
        }
    }

    /// Convertit depuis un nom de fichier (sans extension)
    pub fn from_filename(filename: &str) -> Self {
        match filename {
            "facebook" => LogoId::Facebook,
            "whatsapp" => LogoId::WhatsApp,
            "facebook_color" => LogoId::FacebookColor,
            "whatsapp_color" => LogoId::WhatsAppColor,
            "instagram_color" => LogoId::InstagramColor,
            _ => LogoId::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum LogoRatio {
    Small,
    #[default]
    Medium,
    Large,
}

impl LogoRatio {
    /// Retourne la chaîne de caractères associée au ratio de logo
    pub fn as_str(&self) -> &str {
        match self {
            LogoRatio::Small => "small",
            LogoRatio::Medium => "medium",
            LogoRatio::Large => "large",
        }
    }

    /// Convertit depuis une chaîne de caractères
    pub fn from_str(s: &str) -> Self {
        match s {
            "small" => LogoRatio::Small,
            "medium" => LogoRatio::Medium,
            "large" => LogoRatio::Large,
            _ => LogoRatio::default(),
        }
    }

    /// Retourne le ratio numérique associé au type de logo
    pub fn as_ratio(&self) -> f64 {
        match self {
            LogoRatio::Small => 0.15,
            LogoRatio::Medium => 0.20,
            LogoRatio::Large => 0.25,
        }
    }
}
