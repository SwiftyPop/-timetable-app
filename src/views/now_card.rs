use egui::{vec2, Color32, Rounding, Stroke, Ui};
use crate::models::{DAYS_FULL};
use crate::state::{format_duration_min, NowInfo};
use crate::theme::{hsl_to_color32, Palette};

pub fn render_now_card(ui: &mut Ui, now: &NowInfo, palette: &Palette, time: f64) {
    let frame = egui::Frame::none()
        .fill(palette.card)
        .stroke(Stroke::new(1.0_f32, palette.border))
        .rounding(Rounding::same(16.0))
        .inner_margin(egui::Margin::symmetric(16.0, 12.0));

    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            // Left: Time clock & date
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(&now.time_str)
                        .size(26.0)
                        .strong()
                        .color(palette.ink),
                );
                ui.label(
                    egui::RichText::new(&now.date_str)
                        .size(11.5)
                        .color(palette.mute),
                );
            });

            ui.add_space(16.0);

            // Right: Class status
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    // Pulsing Dot
                    let (dot_rect, _) = ui.allocate_exact_size(vec2(10.0, 10.0), egui::Sense::hover());
                    let dot_color = if now.current_class.is_some() {
                        let pulse = ((time * 3.0).sin() * 0.5 + 0.5) as f32;
                        let alpha = (150.0 + pulse * 105.0) as u8;
                        Color32::from_rgba_premultiplied(34, 197, 94, alpha)
                    } else {
                        Color32::from_rgb(239, 68, 68)
                    };
                    ui.painter().circle_filled(dot_rect.center(), 4.0, dot_color);

                    let status_badge = if now.current_class.is_some() {
                        "IN CLASS NOW"
                    } else if now.day_of_week > 0 {
                        "FREE NOW · UP NEXT"
                    } else {
                        "NO CLASS TODAY · UP NEXT"
                    };

                    ui.label(
                        egui::RichText::new(status_badge)
                            .size(10.5)
                            .strong()
                            .color(if now.current_class.is_some() {
                                Color32::from_rgb(34, 197, 94)
                            } else {
                                palette.mute
                            }),
                    );
                });

                if let Some(cur) = now.current_class {
                    let subj = cur.subject();
                    ui.label(
                        egui::RichText::new(subj.name)
                            .size(16.0)
                            .strong()
                            .color(palette.ink),
                    );
                    let meta_text = format!(
                        "{} · {} | {:02}:00 – {:02}:00 · {} left",
                        cur.class_type,
                        cur.location,
                        cur.start,
                        cur.end,
                        format_duration_min(now.current_remaining_min)
                    );
                    ui.label(egui::RichText::new(meta_text).size(12.0).color(palette.mute));

                    // Animated Progress Bar
                    let (bar_rect, _) = ui.allocate_exact_size(vec2(ui.available_width().min(360.0), 6.0), egui::Sense::hover());
                    ui.painter().rect_filled(bar_rect, Rounding::same(3.0), palette.border);
                    let mut fill_rect = bar_rect;
                    fill_rect.set_width(bar_rect.width() * now.current_progress);
                    let subj_color = hsl_to_color32(subj.hue, 0.75, 0.52, 1.0);
                    ui.painter().rect_filled(fill_rect, Rounding::same(3.0), subj_color);
                } else if let Some(nxt) = now.next_class {
                    let subj = nxt.subject();
                    ui.label(
                        egui::RichText::new(subj.name)
                            .size(15.5)
                            .strong()
                            .color(palette.ink),
                    );
                    let day_name = if now.next_day_offset == 0 {
                        "Today".to_string()
                    } else if now.next_day_offset == 1 {
                        "Tomorrow".to_string()
                    } else {
                        DAYS_FULL[nxt.day as usize - 1].to_string()
                    };

                    let meta_text = format!(
                        "{} · {:02}:00 – {:02}:00 | {} · starts in {}",
                        day_name,
                        nxt.start,
                        nxt.end,
                        nxt.location,
                        format_duration_min(now.next_starts_in_min)
                    );
                    ui.label(egui::RichText::new(meta_text).size(12.0).color(palette.mute));
                } else {
                    ui.label(
                        egui::RichText::new("No upcoming classes scheduled")
                            .size(14.0)
                            .color(palette.mute),
                    );
                }
            });
        });
    });
}
