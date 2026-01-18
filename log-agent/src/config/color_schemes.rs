// Allow dead code as these color schemes may be used in the future
#![allow(dead_code)]

use super::types::{Color, ColorConfig, ColorName, ColorStyle, Style, SystemColorConfig};

/// Retourne le schéma de couleurs par défaut
pub fn default() -> ColorConfig {
    ColorConfig::default()
}

/// Schéma Solarized Dark
pub fn solarized_dark() -> ColorConfig {
    ColorConfig {
        error: ColorStyle {
            fg: Some(Color::Named(ColorName::BrightRed)),
            bg: None,
            style: vec![Style::Bold],
        },
        warn: ColorStyle {
            fg: Some(Color::Named(ColorName::BrightYellow)),
            bg: None,
            style: vec![],
        },
        debug: ColorStyle {
            fg: Some(Color::Named(ColorName::Cyan)),
            bg: None,
            style: vec![],
        },
        info: ColorStyle {
            fg: Some(Color::Named(ColorName::BrightWhite)),
            bg: None,
            style: vec![],
        },
        system: SystemColorConfig {
            success: ColorStyle {
                fg: Some(Color::Named(ColorName::Green)),
                bg: None,
                style: vec![],
            },
            error: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightRed)),
                bg: None,
                style: vec![Style::Bold],
            },
            info: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightCyan)),
                bg: None,
                style: vec![],
            },
            dim: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightBlack)),
                bg: None,
                style: vec![Style::Italic],
            },
        },
    }
}

/// Schéma High Contrast
pub fn high_contrast() -> ColorConfig {
    ColorConfig {
        error: ColorStyle {
            fg: Some(Color::Named(ColorName::Red)),
            bg: Some(Color::Named(ColorName::Black)),
            style: vec![Style::Bold, Style::Underline],
        },
        warn: ColorStyle {
            fg: Some(Color::Named(ColorName::Yellow)),
            bg: Some(Color::Named(ColorName::Black)),
            style: vec![Style::Bold],
        },
        debug: ColorStyle {
            fg: Some(Color::Named(ColorName::Cyan)),
            bg: Some(Color::Named(ColorName::Black)),
            style: vec![],
        },
        info: ColorStyle {
            fg: Some(Color::Named(ColorName::White)),
            bg: Some(Color::Named(ColorName::Black)),
            style: vec![],
        },
        system: SystemColorConfig {
            success: ColorStyle {
                fg: Some(Color::Named(ColorName::Green)),
                bg: Some(Color::Named(ColorName::Black)),
                style: vec![Style::Bold],
            },
            error: ColorStyle {
                fg: Some(Color::Named(ColorName::Red)),
                bg: Some(Color::Named(ColorName::Black)),
                style: vec![Style::Bold],
            },
            info: ColorStyle {
                fg: Some(Color::Named(ColorName::Cyan)),
                bg: Some(Color::Named(ColorName::Black)),
                style: vec![],
            },
            dim: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightBlack)),
                bg: Some(Color::Named(ColorName::Black)),
                style: vec![],
            },
        },
    }
}

/// Schéma Minimal (couleurs douces, pas de gras)
pub fn minimal() -> ColorConfig {
    ColorConfig {
        error: ColorStyle {
            fg: Some(Color::Named(ColorName::BrightRed)),
            bg: None,
            style: vec![],
        },
        warn: ColorStyle {
            fg: Some(Color::Named(ColorName::BrightYellow)),
            bg: None,
            style: vec![],
        },
        debug: ColorStyle {
            fg: Some(Color::Named(ColorName::BrightBlue)),
            bg: None,
            style: vec![],
        },
        info: ColorStyle {
            fg: Some(Color::Named(ColorName::White)),
            bg: None,
            style: vec![],
        },
        system: SystemColorConfig {
            success: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightGreen)),
                bg: None,
                style: vec![],
            },
            error: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightRed)),
                bg: None,
                style: vec![],
            },
            info: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightCyan)),
                bg: None,
                style: vec![],
            },
            dim: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightBlack)),
                bg: None,
                style: vec![],
            },
        },
    }
}

/// Schéma Monochrome (nuances de gris)
pub fn monochrome() -> ColorConfig {
    ColorConfig {
        error: ColorStyle {
            fg: Some(Color::Named(ColorName::White)),
            bg: None,
            style: vec![Style::Bold, Style::Underline],
        },
        warn: ColorStyle {
            fg: Some(Color::Named(ColorName::BrightWhite)),
            bg: None,
            style: vec![Style::Bold],
        },
        debug: ColorStyle {
            fg: Some(Color::Named(ColorName::BrightBlack)),
            bg: None,
            style: vec![],
        },
        info: ColorStyle {
            fg: Some(Color::Named(ColorName::White)),
            bg: None,
            style: vec![],
        },
        system: SystemColorConfig {
            success: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightWhite)),
                bg: None,
                style: vec![Style::Bold],
            },
            error: ColorStyle {
                fg: Some(Color::Named(ColorName::White)),
                bg: None,
                style: vec![Style::Bold, Style::Underline],
            },
            info: ColorStyle {
                fg: Some(Color::Named(ColorName::White)),
                bg: None,
                style: vec![],
            },
            dim: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightBlack)),
                bg: None,
                style: vec![Style::Dimmed],
            },
        },
    }
}

/// Retourne un schéma par nom
pub fn get_scheme(name: &str) -> Option<ColorConfig> {
    match name.to_lowercase().as_str() {
        "default" => Some(default()),
        "solarized-dark" | "solarized_dark" => Some(solarized_dark()),
        "high-contrast" | "high_contrast" => Some(high_contrast()),
        "minimal" => Some(minimal()),
        "monochrome" => Some(monochrome()),
        _ => None,
    }
}

/// Liste tous les schémas disponibles
pub fn list_schemes() -> Vec<(&'static str, &'static str)> {
    vec![
        ("default", "Default colors (red errors, yellow warnings)"),
        ("solarized-dark", "Solarized Dark theme"),
        ("high-contrast", "High contrast for accessibility"),
        ("minimal", "Minimal colors, no bold"),
        ("monochrome", "Shades of gray only"),
    ]
}
