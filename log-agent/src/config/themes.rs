use crate::config::types::{Color, ColorConfig, ColorStyle, SystemColorConfig, TuiColorConfig};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Type alias for theme information (name, description, author)
pub type ThemeInfo = (String, Option<String>, Option<String>);

/// Configuration complète d'un thème
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Nom du thème
    pub name: String,
    /// Description du thème
    #[serde(default)]
    pub description: Option<String>,
    /// Auteur du thème
    #[serde(default)]
    pub author: Option<String>,
    /// Couleurs des logs
    pub colors: ColorConfig,
    /// Couleurs de l'interface TUI
    pub tui: TuiColorConfig,
}

impl ThemeConfig {
    /// Charge un thème depuis un fichier
    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read theme file: {}", path.display()))?;
        
        let theme: ThemeConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse theme file: {}", path.display()))?;
        
        Ok(theme)
    }

    /// Sauvegarde le thème dans un fichier
    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize theme")?;
        
        // Créer le dossier parent si nécessaire
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create theme directory: {}", parent.display()))?;
        }
        
        fs::write(path, content)
            .with_context(|| format!("Failed to write theme file: {}", path.display()))?;
        
        Ok(())
    }
}

/// Gestionnaire de thèmes
pub struct ThemeManager {
    themes_dir: PathBuf,
}

impl ThemeManager {
    /// Crée un nouveau gestionnaire de thèmes
    pub fn new(config_dir: PathBuf) -> Self {
        let themes_dir = config_dir.join("themes");
        Self { themes_dir }
    }

    /// Retourne le chemin du dossier des thèmes
    #[allow(dead_code)]
    pub fn themes_dir(&self) -> &PathBuf {
        &self.themes_dir
    }

    /// Charge un thème par son nom
    pub fn load_theme(&self, theme_name: &str) -> Result<ThemeConfig> {
        let theme_path = self.themes_dir.join(format!("{}.toml", theme_name));
        ThemeConfig::load_from_file(&theme_path)
    }

    /// Liste tous les thèmes disponibles
    #[allow(dead_code)]
    pub fn list_themes(&self) -> Result<Vec<String>> {
        if !self.themes_dir.exists() {
            return Ok(Vec::new());
        }

        let mut themes = Vec::new();
        
        for entry in fs::read_dir(&self.themes_dir)
            .with_context(|| format!("Failed to read themes directory: {}", self.themes_dir.display()))? 
        {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml")
                && let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    themes.push(name.to_string());
                }
        }
        
        themes.sort();
        Ok(themes)
    }

    /// Liste tous les thèmes avec leurs descriptions
    pub fn list_themes_with_info(&self) -> Result<Vec<ThemeInfo>> {
        let themes = self.list_themes()?;
        let mut result = Vec::new();

        for theme_name in themes {
            if let Ok(theme) = self.load_theme(&theme_name) {
                result.push((theme.name, theme.description, theme.author));
            }
        }

        Ok(result)
    }

    /// Vérifie si un thème existe
    pub fn theme_exists(&self, theme_name: &str) -> bool {
        let theme_path = self.themes_dir.join(format!("{}.toml", theme_name));
        theme_path.exists()
    }

    /// Crée un nouveau thème en copiant un thème existant
    pub fn create_from_template(&self, new_name: &str, template_name: &str) -> Result<ThemeConfig> {
        let template = self.load_theme(template_name)
            .with_context(|| format!("Template theme '{}' not found", template_name))?;

        let mut new_theme = template;
        new_theme.name = new_name.to_string();
        new_theme.description = Some(format!("Custom theme based on {}", template_name));
        new_theme.author = None;

        Ok(new_theme)
    }

    /// Sauvegarde un thème
    pub fn save_theme(&self, theme: &ThemeConfig) -> Result<()> {
        let theme_path = self.themes_dir.join(format!("{}.toml", theme.name));
        theme.save_to_file(&theme_path)
    }

    /// Crée un thème à partir de la configuration actuelle
    pub fn export_from_config(&self, name: &str, colors: &ColorConfig, tui: &TuiColorConfig, description: Option<String>, author: Option<String>) -> ThemeConfig {
        ThemeConfig {
            name: name.to_string(),
            description,
            author,
            colors: colors.clone(),
            tui: tui.clone(),
        }
    }

    /// Initialise les thèmes par défaut
    pub fn initialize_default_themes(&self) -> Result<()> {
        // Créer le dossier themes s'il n'existe pas
        if !self.themes_dir.exists() {
            fs::create_dir_all(&self.themes_dir)
                .with_context(|| format!("Failed to create themes directory: {}", self.themes_dir.display()))?;
        }

        // Liste des thèmes par défaut à créer
        let default_themes = vec![
            ("default", create_default_theme()),
            ("dracula", create_dracula_theme()),
            ("nord", create_nord_theme()),
            ("monokai", create_monokai_theme()),
            ("solarized-dark", create_solarized_dark_theme()),
            ("minimal", create_minimal_theme()),
        ];

        for (name, theme) in default_themes {
            let theme_path = self.themes_dir.join(format!("{}.toml", name));
            
            // Ne pas écraser les thèmes existants
            if !theme_path.exists() {
                theme.save_to_file(&theme_path)?;
            }
        }

        Ok(())
    }
}

// ==================== Thèmes par défaut ====================

use crate::config::types::ColorName;

fn create_default_theme() -> ThemeConfig {
    ThemeConfig {
        name: "default".to_string(),
        description: Some("Default color theme with vibrant colors".to_string()),
        author: Some("mcp-log-agent".to_string()),
        colors: ColorConfig {
            error: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightRed)),
                bg: None,
                style: vec![crate::config::types::Style::Bold],
            },
            warn: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightYellow)),
                bg: None,
                style: vec![],
            },
            info: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightCyan)),
                bg: None,
                style: vec![],
            },
            debug: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightBlue)),
                bg: None,
                style: vec![],
            },
            system: SystemColorConfig {
                success: ColorStyle {
                    fg: Some(Color::Named(ColorName::BrightGreen)),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                error: ColorStyle {
                    fg: Some(Color::Named(ColorName::BrightRed)),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                info: ColorStyle {
                    fg: Some(Color::Named(ColorName::BrightCyan)),
                    bg: None,
                    style: vec![],
                },
                dim: ColorStyle {
                    fg: Some(Color::Named(ColorName::BrightBlack)),
                    bg: None,
                    style: vec![crate::config::types::Style::Dimmed],
                },
            },
        },
        tui: TuiColorConfig {
            header_bg: Color::Named(ColorName::Blue),
            header_fg: Color::Named(ColorName::White),
            status_bg: Color::Named(ColorName::Black),
            status_fg: Color::Named(ColorName::Green),
            border: Color::Named(ColorName::Cyan),
            selected_bg: Color::Named(ColorName::BrightBlack),
            selected_fg: Color::Named(ColorName::White),
            search_match: Color::Named(ColorName::Yellow),
            search_dimmed: Color::Named(ColorName::BrightBlack),
            help_bg: Color::Named(ColorName::Black),
            help_fg: Color::Named(ColorName::White),
        },
    }
}

fn create_dracula_theme() -> ThemeConfig {
    ThemeConfig {
        name: "dracula".to_string(),
        description: Some("Dracula dark theme with purple accents".to_string()),
        author: Some("mcp-log-agent".to_string()),
        colors: ColorConfig {
            error: ColorStyle {
                fg: Some(Color::Hex("FF5555".to_string())),
                bg: None,
                style: vec![crate::config::types::Style::Bold],
            },
            warn: ColorStyle {
                fg: Some(Color::Hex("FFB86C".to_string())),
                bg: None,
                style: vec![],
            },
            info: ColorStyle {
                fg: Some(Color::Hex("8BE9FD".to_string())),
                bg: None,
                style: vec![],
            },
            debug: ColorStyle {
                fg: Some(Color::Hex("BD93F9".to_string())),
                bg: None,
                style: vec![],
            },
            system: SystemColorConfig {
                success: ColorStyle {
                    fg: Some(Color::Hex("50FA7B".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                error: ColorStyle {
                    fg: Some(Color::Hex("FF5555".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                info: ColorStyle {
                    fg: Some(Color::Hex("8BE9FD".to_string())),
                    bg: None,
                    style: vec![],
                },
                dim: ColorStyle {
                    fg: Some(Color::Hex("6272A4".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Dimmed],
                },
            },
        },
        tui: TuiColorConfig {
            header_bg: Color::Hex("282A36".to_string()),
            header_fg: Color::Hex("F8F8F2".to_string()),
            status_bg: Color::Hex("282A36".to_string()),
            status_fg: Color::Hex("50FA7B".to_string()),
            border: Color::Hex("BD93F9".to_string()),
            selected_bg: Color::Hex("44475A".to_string()),
            selected_fg: Color::Hex("F8F8F2".to_string()),
            search_match: Color::Hex("F1FA8C".to_string()),
            search_dimmed: Color::Hex("6272A4".to_string()),
            help_bg: Color::Hex("282A36".to_string()),
            help_fg: Color::Hex("F8F8F2".to_string()),
        },
    }
}

fn create_nord_theme() -> ThemeConfig {
    ThemeConfig {
        name: "nord".to_string(),
        description: Some("Nord arctic, north-bluish color palette".to_string()),
        author: Some("mcp-log-agent".to_string()),
        colors: ColorConfig {
            error: ColorStyle {
                fg: Some(Color::Hex("BF616A".to_string())),
                bg: None,
                style: vec![crate::config::types::Style::Bold],
            },
            warn: ColorStyle {
                fg: Some(Color::Hex("EBCB8B".to_string())),
                bg: None,
                style: vec![],
            },
            info: ColorStyle {
                fg: Some(Color::Hex("88C0D0".to_string())),
                bg: None,
                style: vec![],
            },
            debug: ColorStyle {
                fg: Some(Color::Hex("81A1C1".to_string())),
                bg: None,
                style: vec![],
            },
            system: SystemColorConfig {
                success: ColorStyle {
                    fg: Some(Color::Hex("A3BE8C".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                error: ColorStyle {
                    fg: Some(Color::Hex("BF616A".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                info: ColorStyle {
                    fg: Some(Color::Hex("88C0D0".to_string())),
                    bg: None,
                    style: vec![],
                },
                dim: ColorStyle {
                    fg: Some(Color::Hex("4C566A".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Dimmed],
                },
            },
        },
        tui: TuiColorConfig {
            header_bg: Color::Hex("2E3440".to_string()),
            header_fg: Color::Hex("ECEFF4".to_string()),
            status_bg: Color::Hex("2E3440".to_string()),
            status_fg: Color::Hex("A3BE8C".to_string()),
            border: Color::Hex("88C0D0".to_string()),
            selected_bg: Color::Hex("3B4252".to_string()),
            selected_fg: Color::Hex("ECEFF4".to_string()),
            search_match: Color::Hex("EBCB8B".to_string()),
            search_dimmed: Color::Hex("4C566A".to_string()),
            help_bg: Color::Hex("2E3440".to_string()),
            help_fg: Color::Hex("ECEFF4".to_string()),
        },
    }
}

fn create_monokai_theme() -> ThemeConfig {
    ThemeConfig {
        name: "monokai".to_string(),
        description: Some("Monokai Pro color scheme".to_string()),
        author: Some("mcp-log-agent".to_string()),
        colors: ColorConfig {
            error: ColorStyle {
                fg: Some(Color::Hex("F92672".to_string())),
                bg: None,
                style: vec![crate::config::types::Style::Bold],
            },
            warn: ColorStyle {
                fg: Some(Color::Hex("E6DB74".to_string())),
                bg: None,
                style: vec![],
            },
            info: ColorStyle {
                fg: Some(Color::Hex("66D9EF".to_string())),
                bg: None,
                style: vec![],
            },
            debug: ColorStyle {
                fg: Some(Color::Hex("AE81FF".to_string())),
                bg: None,
                style: vec![],
            },
            system: SystemColorConfig {
                success: ColorStyle {
                    fg: Some(Color::Hex("A6E22E".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                error: ColorStyle {
                    fg: Some(Color::Hex("F92672".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                info: ColorStyle {
                    fg: Some(Color::Hex("66D9EF".to_string())),
                    bg: None,
                    style: vec![],
                },
                dim: ColorStyle {
                    fg: Some(Color::Hex("75715E".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Dimmed],
                },
            },
        },
        tui: TuiColorConfig {
            header_bg: Color::Hex("272822".to_string()),
            header_fg: Color::Hex("F8F8F2".to_string()),
            status_bg: Color::Hex("272822".to_string()),
            status_fg: Color::Hex("A6E22E".to_string()),
            border: Color::Hex("66D9EF".to_string()),
            selected_bg: Color::Hex("49483E".to_string()),
            selected_fg: Color::Hex("F8F8F2".to_string()),
            search_match: Color::Hex("E6DB74".to_string()),
            search_dimmed: Color::Hex("75715E".to_string()),
            help_bg: Color::Hex("272822".to_string()),
            help_fg: Color::Hex("F8F8F2".to_string()),
        },
    }
}

fn create_solarized_dark_theme() -> ThemeConfig {
    ThemeConfig {
        name: "solarized-dark".to_string(),
        description: Some("Solarized Dark color scheme".to_string()),
        author: Some("mcp-log-agent".to_string()),
        colors: ColorConfig {
            error: ColorStyle {
                fg: Some(Color::Hex("DC322F".to_string())),
                bg: None,
                style: vec![crate::config::types::Style::Bold],
            },
            warn: ColorStyle {
                fg: Some(Color::Hex("B58900".to_string())),
                bg: None,
                style: vec![],
            },
            info: ColorStyle {
                fg: Some(Color::Hex("2AA198".to_string())),
                bg: None,
                style: vec![],
            },
            debug: ColorStyle {
                fg: Some(Color::Hex("268BD2".to_string())),
                bg: None,
                style: vec![],
            },
            system: SystemColorConfig {
                success: ColorStyle {
                    fg: Some(Color::Hex("859900".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                error: ColorStyle {
                    fg: Some(Color::Hex("DC322F".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                info: ColorStyle {
                    fg: Some(Color::Hex("2AA198".to_string())),
                    bg: None,
                    style: vec![],
                },
                dim: ColorStyle {
                    fg: Some(Color::Hex("586E75".to_string())),
                    bg: None,
                    style: vec![crate::config::types::Style::Dimmed],
                },
            },
        },
        tui: TuiColorConfig {
            header_bg: Color::Hex("002B36".to_string()),
            header_fg: Color::Hex("839496".to_string()),
            status_bg: Color::Hex("002B36".to_string()),
            status_fg: Color::Hex("859900".to_string()),
            border: Color::Hex("586E75".to_string()),
            selected_bg: Color::Hex("073642".to_string()),
            selected_fg: Color::Hex("93A1A1".to_string()),
            search_match: Color::Hex("B58900".to_string()),
            search_dimmed: Color::Hex("586E75".to_string()),
            help_bg: Color::Hex("002B36".to_string()),
            help_fg: Color::Hex("839496".to_string()),
        },
    }
}

fn create_minimal_theme() -> ThemeConfig {
    ThemeConfig {
        name: "minimal".to_string(),
        description: Some("Minimal monochrome theme".to_string()),
        author: Some("mcp-log-agent".to_string()),
        colors: ColorConfig {
            error: ColorStyle {
                fg: Some(Color::Named(ColorName::White)),
                bg: None,
                style: vec![crate::config::types::Style::Bold],
            },
            warn: ColorStyle {
                fg: Some(Color::Named(ColorName::White)),
                bg: None,
                style: vec![],
            },
            info: ColorStyle {
                fg: Some(Color::Named(ColorName::White)),
                bg: None,
                style: vec![],
            },
            debug: ColorStyle {
                fg: Some(Color::Named(ColorName::BrightBlack)),
                bg: None,
                style: vec![],
            },
            system: SystemColorConfig {
                success: ColorStyle {
                    fg: Some(Color::Named(ColorName::White)),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                error: ColorStyle {
                    fg: Some(Color::Named(ColorName::White)),
                    bg: None,
                    style: vec![crate::config::types::Style::Bold],
                },
                info: ColorStyle {
                    fg: Some(Color::Named(ColorName::White)),
                    bg: None,
                    style: vec![],
                },
                dim: ColorStyle {
                    fg: Some(Color::Named(ColorName::BrightBlack)),
                    bg: None,
                    style: vec![crate::config::types::Style::Dimmed],
                },
            },
        },
        tui: TuiColorConfig {
            header_bg: Color::Named(ColorName::Black),
            header_fg: Color::Named(ColorName::White),
            status_bg: Color::Named(ColorName::Black),
            status_fg: Color::Named(ColorName::White),
            border: Color::Named(ColorName::BrightBlack),
            selected_bg: Color::Named(ColorName::BrightBlack),
            selected_fg: Color::Named(ColorName::White),
            search_match: Color::Named(ColorName::White),
            search_dimmed: Color::Named(ColorName::BrightBlack),
            help_bg: Color::Named(ColorName::Black),
            help_fg: Color::Named(ColorName::White),
        },
    }
}
