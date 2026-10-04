use egui::{vec2, Rect, Rounding, Stroke, Ui};
use crate::models::{ClassEvent, DAYS_FULL, EVENTS};
use crate::state::NowInfo;
use crate::theme::{hsl_to_color32, Palette};

pub fn render_day_view(ui: &mut Ui, day: u8, now: &NowInfo, palette: &Palette) {
    let day_idx = (day as usize).saturating_sub(1);
    let day_name = DAYS_FULL.get(day_idx).unwrap_or(&"Monday");
    let is_today = now.day_of_week == day as u32;

    ui.horizontal(|ui| {
        let title = if is_today {
            format!("{} · Today", day_name)
        } else {
            day_name.to_string()
        };
        ui.label(
            egui::RichText::new(title)
                .size(13.0)
                .strong()
                .color(palette.mute),
        );
    });

    ui.add_space(8.0);

    let day_events: Vec<&'static ClassEvent> = EVENTS.iter().filter(|e| e.day == day).collect();

    for (i, &ev) in day_events.iter().enumerate() {
        // Free period gap check
        if i > 0 {
            let prev = day_events[i - 1];
            if ev.start > prev.end {
                render_gap(ui, prev.end, ev.start, palette);
            }
        }

        render_class_card(ui, ev, now, is_today, palette);
    }

    if day_events.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(32.0);
            ui.label(
                egui::RichText::new("No classes scheduled for this day")
                    .size(15.0)
                    .color(palette.mute),
            );
        });
    }
}

fn render_gap(ui: &mut Ui, start: u8, end: u8, palette: &Palette) {
    ui.horizontal(|ui| {
        ui.add_space(80.0);
        let gap_text = format!("— Free · {:02}:00 – {:02}:00 ({} h) —", start, end, end - start);
        ui.label(egui::RichText::new(gap_text).size(11.5).color(palette.mute));
    });
    ui.add_space(6.0);
}

fn render_class_card(ui: &mut Ui, ev: &'static ClassEvent, now: &NowInfo, is_today: bool, palette: &Palette) {
    let subj = ev.subject();
    let is_live = is_today && now.minute_of_day >= (ev.start as u32 * 60) && now.minute_of_day < (ev.end as u32 * 60);
    let is_past = is_today && now.minute_of_day >= (ev.end as u32 * 60);

    ui.horizontal_top(|ui| {
        // Left Column: Time & Duration
        ui.allocate_ui_with_layout(
            vec2(72.0, 70.0),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.label(
                    egui::RichText::new(format!("{:02}:00", ev.start))
                        .size(16.0)
                        .strong()
                        .color(if is_past { palette.mute } else { palette.ink }),
                );
                ui.label(
                    egui::RichText::new(format!("{:02}:00", ev.end))
                        .size(12.5)
                        .color(palette.mute),
                );
                let dur = ev.duration();
                let dur_str = if dur == 1 { "1 hour".to_string() } else { format!("{} hours", dur) };
                ui.label(egui::RichText::new(dur_str).size(10.5).color(palette.mute));
            },
        );

        ui.add_space(4.0);

        // Right Column: Event Card
        let card_bg = hsl_to_color32(subj.hue, 0.85, 0.55, if is_live { 0.22 } else { 0.12 });
        let border_color = hsl_to_color32(subj.hue, 0.80, 0.55, if is_live { 0.70 } else { 0.28 });
        let left_strip_color = hsl_to_color32(subj.hue, 0.75, 0.52, 1.0);

        let frame = egui::Frame::none()
            .fill(card_bg)
            .stroke(Stroke::new(1.0_f32, border_color))
            .rounding(Rounding::same(14.0))
            .inner_margin(egui::Margin::symmetric(14.0, 12.0));

        let response = frame.show(ui, |ui| {
            ui.vertical(|ui| {
                // Tag badge
                let tag_text = format!(
                    "{} · {}{}",
                    subj.code,
                    ev.class_type.to_uppercase(),
                    if is_live { " · LIVE" } else { "" }
                );
                let tag_color = hsl_to_color32(subj.hue, 0.75, 0.50, 1.0);
                ui.label(egui::RichText::new(tag_text).size(11.0).strong().color(tag_color));

                ui.add_space(2.0);

                // Subject title
                ui.label(
                    egui::RichText::new(subj.name)
                        .size(16.0)
                        .strong()
                        .color(palette.ink),
                );

                ui.add_space(3.0);

                // Location and Lecturers
                let loc_text = format!("📍 {}", ev.location);
                ui.label(egui::RichText::new(loc_text).size(12.0).color(palette.mute));
                ui.label(egui::RichText::new(ev.lecturers).size(11.5).color(palette.mute));

                ui.add_space(6.0);

                // Slot pills
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(4.0, 4.0);
                    for s in ev.start..ev.end {
                        let pill_text = format!("{:02}:00 – {:02}:00", s, s + 1);
                        let pill_frame = egui::Frame::none()
                            .fill(hsl_to_color32(subj.hue, 0.80, 0.55, 0.16))
                            .rounding(Rounding::same(999.0))
                            .inner_margin(egui::Margin::symmetric(7.0, 3.0));

                        pill_frame.show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(pill_text)
                                    .size(10.5)
                                    .strong()
                                    .color(tag_color),
                            );
                        });
                    }
                });

                // Live progress bar if active
                if is_live {
                    ui.add_space(6.0);
                    let (bar_rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 5.0), egui::Sense::hover());
                    ui.painter().rect_filled(bar_rect, Rounding::same(2.5), palette.border);
                    let mut fill_rect = bar_rect;
                    fill_rect.set_width(bar_rect.width() * now.current_progress);
                    ui.painter().rect_filled(fill_rect, Rounding::same(2.5), left_strip_color);
                }
            });
        });

        // Draw 4px thick accent bar on the left of the card
        let r = response.response.rect;
        let left_strip_rect = Rect::from_min_max(
            r.min,
            egui::pos2(r.min.x + 4.0, r.max.y),
        );
        ui.painter().rect_filled(
            left_strip_rect,
            Rounding { nw: 14.0, sw: 14.0, ne: 0.0, se: 0.0 },
            left_strip_color,
        );
    });

    ui.add_space(10.0);
}
