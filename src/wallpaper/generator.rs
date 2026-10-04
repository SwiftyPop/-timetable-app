// Pure Rust Wallpaper Generator using tiny-skia

use tiny_skia::*;
use crate::models::EVENTS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WallpaperTheme {
    Olive,
    Midnight,
    Cream,
}

impl Default for WallpaperTheme {
    fn default() -> Self {
        Self::Olive
    }
}

impl WallpaperTheme {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Olive => "Olive",
            Self::Midnight => "Midnight",
            Self::Cream => "Cream",
        }
    }

    pub fn colors(&self) -> (Color, Color, Color, Color, Color) {
        // (top, bottom, ink, mute, line)
        match self {
            Self::Olive => (
                Color::from_rgba8(93, 103, 67, 255),
                Color::from_rgba8(34, 40, 21, 255),
                Color::from_rgba8(241, 239, 224, 255),
                Color::from_rgba8(241, 239, 224, 165),
                Color::from_rgba8(241, 239, 224, 120),
            ),
            Self::Midnight => (
                Color::from_rgba8(38, 46, 80, 255),
                Color::from_rgba8(8, 10, 21, 255),
                Color::from_rgba8(238, 240, 255, 255),
                Color::from_rgba8(238, 240, 255, 165),
                Color::from_rgba8(238, 240, 255, 120),
            ),
            Self::Cream => (
                Color::from_rgba8(247, 241, 229, 255),
                Color::from_rgba8(226, 214, 191, 255),
                Color::from_rgba8(42, 36, 24, 255),
                Color::from_rgba8(42, 36, 24, 165),
                Color::from_rgba8(42, 36, 24, 120),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ResolutionPreset {
    pub label: &'static str,
    pub width: u32,
    pub height: u32,
    pub is_phone: bool,
}

pub static PRESETS: &[ResolutionPreset] = &[
    ResolutionPreset {
        label: "Android · 1080×2400",
        width: 1080,
        height: 2400,
        is_phone: true,
    },
    ResolutionPreset {
        label: "iPhone · 1179×2556",
        width: 1179,
        height: 2556,
        is_phone: true,
    },
    ResolutionPreset {
        label: "Large phone · 1440×3120",
        width: 1440,
        height: 3120,
        is_phone: true,
    },
    ResolutionPreset {
        label: "Full HD · 1920×1080",
        width: 1920,
        height: 1080,
        is_phone: false,
    },
    ResolutionPreset {
        label: "QHD · 2560×1440",
        width: 2560,
        height: 1440,
        is_phone: false,
    },
    ResolutionPreset {
        label: "4K · 3840×2160",
        width: 3840,
        height: 2160,
        is_phone: false,
    },
    ResolutionPreset {
        label: "Ultrawide · 3440×1440",
        width: 3440,
        height: 1440,
        is_phone: false,
    },
];

fn rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32) -> Path {
    let mut pb = PathBuilder::new();
    let r = r.min(w / 2.0).min(h / 2.0);
    pb.move_to(x + r, y);
    pb.line_to(x + w - r, y);
    pb.quad_to(x + w, y, x + w, y + r);
    pb.line_to(x + w, y + h - r);
    pb.quad_to(x + w, y + h, x + w - r, y + h);
    pb.line_to(x + r, y + h);
    pb.quad_to(x, y + h, x, y + h - r);
    pb.line_to(x, y + r);
    pb.quad_to(x, y, x + r, y);
    pb.close();
    pb.finish().unwrap_or_else(|| PathBuilder::new().finish().unwrap())
}

pub fn render_wallpaper(preset: &ResolutionPreset, theme: WallpaperTheme) -> Result<Vec<u8>, String> {
    let mut pixmap = Pixmap::new(preset.width, preset.height)
        .ok_or_else(|| "Failed to allocate pixmap".to_string())?;

    let (c_top, c_bot, c_ink, c_mute, c_line) = theme.colors();

    // 1. Draw Linear Gradient Background
    let grad = LinearGradient::new(
        Point::from_xy(0.0, 0.0),
        Point::from_xy(preset.width as f32 * 0.35, preset.height as f32),
        vec![
            GradientStop::new(0.0, c_top),
            GradientStop::new(1.0, c_bot),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    ).ok_or_else(|| "Failed to create gradient".to_string())?;

    let mut bg_paint = Paint::default();
    bg_paint.shader = grad;
    let bg_rect = Rect::from_xywh(0.0, 0.0, preset.width as f32, preset.height as f32)
        .ok_or_else(|| "Failed to create bg rect".to_string())?;
    pixmap.fill_rect(bg_rect, &bg_paint, Transform::identity(), None);

    // 2. Render Schedule Layout
    let is_phone = preset.is_phone;
    let base_w = if is_phone { 1080.0 } else { 1920.0 };
    let base_h = if is_phone { 2400.0 } else { 1080.0 };
    let scale = (preset.width as f32 / base_w).min(preset.height as f32 / base_h);
    let offset_x = (preset.width as f32 - base_w * scale) / 2.0;
    let offset_y = (preset.height as f32 - base_h * scale) / 2.0;

    let trans = Transform::from_translate(offset_x, offset_y).post_scale(scale, scale);

    if is_phone {
        render_phone_layout(&mut pixmap, trans, c_ink, c_mute, c_line);
    } else {
        render_desktop_layout(&mut pixmap, trans, c_ink, c_mute, c_line);
    }

    pixmap.encode_png().map_err(|e| e.to_string())
}

fn render_phone_layout(pixmap: &mut Pixmap, trans: Transform, _c_ink: Color, c_mute: Color, c_line: Color) {
    let mut stroke = Stroke::default();
    stroke.width = 2.0;

    let mut line_paint = Paint::default();
    line_paint.set_color(c_line);

    let mut mute_paint = Paint::default();
    mute_paint.set_color(c_mute);

    // Draw events as clean rounded cards
    for ev in EVENTS {
        let cx = 164.0 + (ev.day as f32 - 1.0) * 188.0;
        let by = 1140.0 + (ev.start as f32 - 8.0) * 88.0 + 3.0;
        let bh = (ev.end as f32 - ev.start as f32) * 88.0 - 6.0;

        let path = rounded_rect(cx - 88.0, by, 176.0, bh, 30.0);
        let subj = ev.subject();

        // Fill with subject hue tint
        let mut fill_paint = Paint::default();
        let (r, g, b) = hsl_to_rgb(subj.hue, 0.70, 0.60);
        fill_paint.set_color(Color::from_rgba8(r, g, b, 50));
        pixmap.fill_path(&path, &fill_paint, FillRule::Winding, trans, None);
        pixmap.stroke_path(&path, &line_paint, &stroke, trans, None);
    }
}

fn render_desktop_layout(pixmap: &mut Pixmap, trans: Transform, _c_ink: Color, _c_mute: Color, c_line: Color) {
    let x0 = 270.0;
    let cw = 136.0;
    let y0 = 300.0;
    let rh = 118.0;

    let mut stroke = Stroke::default();
    stroke.width = 2.0;

    let mut line_paint = Paint::default();
    line_paint.set_color(c_line);

    // Grid row divider lines
    for i in 0..=5 {
        let y = y0 + i as f32 * rh;
        let line_rect = Rect::from_xywh(130.0, y, 1636.0, 1.0).unwrap();
        pixmap.fill_rect(line_rect, &line_paint, trans, None);
    }

    // Draw Friday break box
    let brk_path = rounded_rect(x0 + 4.0 * cw + 4.0, y0 + 4.0 * rh + 8.0, 3.0 * cw - 8.0, rh - 16.0, 24.0);
    pixmap.stroke_path(&brk_path, &line_paint, &stroke, trans, None);

    // Draw class cards
    for ev in EVENTS {
        let bx = x0 + (ev.start as f32 - 8.0) * cw + 4.0;
        let bw = (ev.end as f32 - ev.start as f32) * cw - 8.0;
        let by = y0 + (ev.day as f32 - 1.0) * rh + 8.0;
        let bh = rh - 16.0;

        let path = rounded_rect(bx, by, bw, bh, 24.0);
        let subj = ev.subject();

        let mut fill_paint = Paint::default();
        let (r, g, b) = hsl_to_rgb(subj.hue, 0.70, 0.60);
        fill_paint.set_color(Color::from_rgba8(r, g, b, 50));
        pixmap.fill_path(&path, &fill_paint, FillRule::Winding, trans, None);
        pixmap.stroke_path(&path, &line_paint, &stroke, trans, None);
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
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

    (
        ((r + m) * 255.0).clamp(0.0, 255.0) as u8,
        ((g + m) * 255.0).clamp(0.0, 255.0) as u8,
        ((b + m) * 255.0).clamp(0.0, 255.0) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_wallpaper_render() {
        let preset = &PRESETS[0]; // Android 1080x2400
        let bytes = render_wallpaper(preset, WallpaperTheme::Olive).expect("Should render phone wallpaper");
        assert!(bytes.len() > 5000);
        // Verify PNG magic header
        assert_eq!(&bytes[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    }

    #[test]
    fn test_desktop_wallpaper_render() {
        let preset = &PRESETS[3]; // FHD 1920x1080
        let bytes = render_wallpaper(preset, WallpaperTheme::Midnight).expect("Should render desktop wallpaper");
        assert!(bytes.len() > 5000);
        assert_eq!(&bytes[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
    }
}
