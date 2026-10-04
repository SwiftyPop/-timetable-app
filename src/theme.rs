use egui::{Color32, FontData, FontDefinitions, FontFamily, Rounding, Stroke, Visuals};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ThemeMode {
    Auto,
    Light,
    Dark,
    Oled,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl ThemeMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Oled => "OLED",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Auto => "🌓",
            Self::Light => "☀️",
            Self::Dark => "🌙",
            Self::Oled => "🌑",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Auto => Self::Light,
            Self::Light => Self::Dark,
            Self::Dark => Self::Oled,
            Self::Oled => Self::Auto,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub bg: Color32,
    pub card: Color32,
    pub border: Color32,
    pub ink: Color32,
    pub mute: Color32,
    pub accent: Color32,
    pub accent_ink: Color32,
}

pub fn get_palette(mode: ThemeMode, system_dark: bool) -> Palette {
    let effective_dark = match mode {
        ThemeMode::Auto => system_dark,
        ThemeMode::Light => false,
        ThemeMode::Dark | ThemeMode::Oled => true,
    };

    if mode == ThemeMode::Oled {
        Palette {
            bg: Color32::from_rgb(0, 0, 0),
            card: Color32::from_rgb(9, 10, 15),
            border: Color32::from_rgb(28, 30, 39),
            ink: Color32::from_rgb(244, 245, 248),
            mute: Color32::from_rgb(124, 131, 150),
            accent: Color32::from_rgb(244, 245, 248),
            accent_ink: Color32::from_rgb(0, 0, 0),
        }
    } else if effective_dark {
        Palette {
            bg: Color32::from_rgb(13, 15, 20),
            card: Color32::from_rgb(22, 25, 34),
            border: Color32::from_rgb(37, 42, 55),
            ink: Color32::from_rgb(238, 240, 245),
            mute: Color32::from_rgb(140, 146, 164),
            accent: Color32::from_rgb(238, 240, 245),
            accent_ink: Color32::from_rgb(13, 15, 20),
        }
    } else {
        Palette {
            bg: Color32::from_rgb(245, 244, 240),
            card: Color32::from_rgb(255, 255, 255),
            border: Color32::from_rgb(231, 229, 223),
            ink: Color32::from_rgb(22, 24, 29),
            mute: Color32::from_rgb(107, 112, 128),
            accent: Color32::from_rgb(22, 24, 29),
            accent_ink: Color32::from_rgb(255, 255, 255),
        }
    }
}

pub fn hsl_to_color32(h: f32, s: f32, l: f32, a: f32) -> Color32 {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h_prime as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color32::from_rgba_premultiplied(
        ((r + m) * 255.0).clamp(0.0, 255.0) as u8,
        ((g + m) * 255.0).clamp(0.0, 255.0) as u8,
        ((b + m) * 255.0).clamp(0.0, 255.0) as u8,
        (a * 255.0).clamp(0.0, 255.0) as u8,
    )
}

pub fn configure_visuals(ctx: &egui::Context, palette: &Palette, is_dark: bool) {
    let mut visuals = if is_dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    visuals.panel_fill = palette.bg;
    visuals.window_fill = palette.card;
    visuals.extreme_bg_color = palette.bg;

    // Non-interactive widgets
    visuals.widgets.noninteractive.bg_fill = palette.card;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, palette.border);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, palette.ink);
    visuals.widgets.noninteractive.rounding = Rounding::same(12.0);

    // Inactive buttons / pills
    visuals.widgets.inactive.bg_fill = palette.card;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, palette.border);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, palette.mute);
    visuals.widgets.inactive.rounding = Rounding::same(12.0);

    // Hovered buttons
    visuals.widgets.hovered.bg_fill = palette.card;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.5_f32, palette.ink);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, palette.ink);
    visuals.widgets.hovered.rounding = Rounding::same(12.0);

    // Active pressed buttons
    visuals.widgets.active.bg_fill = palette.accent;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, palette.accent);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, palette.accent_ink);
    visuals.widgets.active.rounding = Rounding::same(12.0);

    // Selection color
    visuals.selection.bg_fill = palette.accent;
    visuals.selection.stroke = Stroke::new(1.0_f32, palette.accent_ink);

    ctx.set_visuals(visuals);
}

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Embed Plus Jakarta Sans variable font
    fonts.font_data.insert(
        "PlusJakartaSans".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/PlusJakartaSans.ttf")),
    );

    // Put Plus Jakarta Sans at highest priority for Proportional
    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        family.insert(0, "PlusJakartaSans".to_owned());
    }

    ctx.set_fonts(fonts);
}
