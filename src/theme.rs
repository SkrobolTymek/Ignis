use eframe::egui;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub background: egui::Color32,
    pub foreground: egui::Color32,
    pub accent: egui::Color32,
    pub syntax_highlighting: HashMap<String, egui::Color32>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SerializableTheme {
    name: String,
    background: [u8; 4],
    foreground: [u8; 4],
    accent: [u8; 4],
    syntax_highlighting: HashMap<String, [u8; 4]>,
}

impl Theme {
    pub fn to_serializable(&self) -> SerializableTheme {
        SerializableTheme {
            name: self.name.clone(),
            background: self.background.to_array(),
            foreground: self.foreground.to_array(),
            accent: self.accent.to_array(),
            syntax_highlighting: self.syntax_highlighting.iter()
                .map(|(k, v)| (k.clone(), v.to_array()))
                .collect(),
        }
    }

    pub fn from_serializable(serializable: SerializableTheme) -> Self {
        Theme {
            name: serializable.name,
            background: egui::Color32::from_rgba_premultiplied(
                serializable.background[0],
                serializable.background[1],
                serializable.background[2],
                serializable.background[3],
            ),
            foreground: egui::Color32::from_rgba_premultiplied(
                serializable.foreground[0],
                serializable.foreground[1],
                serializable.foreground[2],
                serializable.foreground[3],
            ),
            accent: egui::Color32::from_rgba_premultiplied(
                serializable.accent[0],
                serializable.accent[1],
                serializable.accent[2],
                serializable.accent[3],
            ),
            syntax_highlighting: serializable.syntax_highlighting.into_iter()
                .map(|(k, v)| (k, egui::Color32::from_rgba_premultiplied(v[0], v[1], v[2], v[3])))
                .collect(),
        }
    }
}

pub struct ThemeManager {
    current_theme: Theme,
    themes: Vec<Theme>,
}

impl ThemeManager {
    pub fn load_default() -> Self {
        let mut default_theme = Theme {
            name: "Default Dark".to_string(),
            background: egui::Color32::from_rgb(25, 25, 25),
            foreground: egui::Color32::from_rgb(220, 220, 220),
            accent: egui::Color32::from_rgb(100, 150, 255),
            syntax_highlighting: HashMap::new(),
        };

        // Add syntax highlighting colors
        default_theme.syntax_highlighting.insert("keyword".to_string(), egui::Color32::from_rgb(255, 100, 100));
        default_theme.syntax_highlighting.insert("string".to_string(), egui::Color32::from_rgb(100, 255, 100));

        Self {
            current_theme: default_theme.clone(),
            themes: vec![default_theme],
        }
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = self.current_theme.background;
        visuals.widgets.noninteractive.fg_stroke.color = self.current_theme.foreground;
        visuals.widgets.active.bg_fill = self.current_theme.accent;
        ctx.set_visuals(visuals);
    }

    pub fn get_syntax_color(&self, token_type: &str) -> egui::Color32 {
        self.current_theme.syntax_highlighting
            .get(token_type)
            .copied()
            .unwrap_or(self.current_theme.foreground)
    }
}