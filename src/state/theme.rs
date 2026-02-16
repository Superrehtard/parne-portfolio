use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ColorScheme {
    Claude,
    Green,
    Amber,
}

impl ColorScheme {
    pub fn css_class(&self) -> &'static str {
        match self {
            ColorScheme::Claude => "theme-claude",
            ColorScheme::Green => "theme-green",
            ColorScheme::Amber => "theme-amber",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ColorScheme::Claude => "claude",
            ColorScheme::Green => "green",
            ColorScheme::Amber => "amber",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "claude" => Some(ColorScheme::Claude),
            "green" => Some(ColorScheme::Green),
            "amber" => Some(ColorScheme::Amber),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeState {
    pub color_scheme: RwSignal<ColorScheme>,
    pub crt_enabled: RwSignal<bool>,
    pub matrix_rain: RwSignal<bool>,
}

impl ThemeState {
    pub fn new() -> Self {
        Self {
            color_scheme: RwSignal::new(ColorScheme::Claude),
            crt_enabled: RwSignal::new(false),
            matrix_rain: RwSignal::new(false),
        }
    }
}
