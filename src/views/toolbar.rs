use egui::{vec2, Response, Rounding, Stroke, Ui};
use crate::models::DAYS_SHORT;
use crate::state::AppView;
use crate::theme::{Palette, ThemeMode};

pub fn render_toolbar(
    ui: &mut Ui,
    palette: &Palette,
    view: &mut AppView,
    selected_day: &mut u8,
    today: u32,
    theme_mode: &mut ThemeMode,
    wallpaper_open: &mut bool,
) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(8.0, 8.0);

        // Day / Week Seg Switch
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
            let frame = egui::Frame::none()
                .fill(palette.card)
                .stroke(Stroke::new(1.0_f32, palette.border))
                .rounding(Rounding::same(12.0))
                .inner_margin(egui::Margin::same(3.0));

            frame.show(ui, |ui| {
                let is_day = *view == AppView::Day;
                let day_btn = custom_pill(ui, "Day", is_day, palette);
                if day_btn.clicked() && !is_day {
                    *view = AppView::Day;
                }

                let is_week = *view == AppView::Week;
                let week_btn = custom_pill(ui, "Week", is_week, palette);
                if week_btn.clicked() && !is_week {
                    *view = AppView::Week;
                }
            });
        });

        // Day tabs (only visible when in Day view)
        if *view == AppView::Day {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                for (i, &day_name) in DAYS_SHORT.iter().enumerate() {
                    let d = (i + 1) as u8;
                    let is_active = *selected_day == d;
                    let is_today = today == d as u32;

                    let label = if is_today {
                        format!("{} •", day_name)
                    } else {
                        day_name.to_string()
                    };

                    let btn = custom_pill(ui, &label, is_active, palette);
                    if btn.clicked() {
                        *selected_day = d;
                    }
                }
            });
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Theme toggle pill
            let theme_label = format!("{} {}", theme_mode.icon(), theme_mode.label());
            let theme_btn = custom_pill(ui, &theme_label, false, palette);
            if theme_btn.clicked() {
                *theme_mode = theme_mode.next();
            }

            // Wallpaper studio toggle button
            let wp_label = if *wallpaper_open { "🖼 Close Studio" } else { "🖼 Wallpaper" };
            let wp_btn = custom_pill(ui, wp_label, *wallpaper_open, palette);
            if wp_btn.clicked() {
                *wallpaper_open = !*wallpaper_open;
            }
        });
    });
}

pub fn custom_pill(ui: &mut Ui, text: &str, active: bool, palette: &Palette) -> Response {
    let (bg, fg, stroke) = if active {
        (palette.accent, palette.accent_ink, Stroke::new(1.0_f32, palette.accent))
    } else {
        (palette.card, palette.mute, Stroke::new(1.0_f32, palette.border))
    };

    let btn = egui::Button::new(egui::RichText::new(text).size(13.0).strong().color(fg))
        .fill(bg)
        .stroke(stroke)
        .rounding(Rounding::same(10.0))
        .min_size(vec2(36.0, 32.0));

    ui.add(btn)
}
