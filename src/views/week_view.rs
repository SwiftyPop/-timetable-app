use egui::{pos2, vec2, Color32, Rect, Rounding, Stroke, Ui};
use crate::models::{DAYS_SHORT, EVENTS};
use crate::state::NowInfo;
use crate::theme::{hsl_to_color32, Palette};

pub fn render_week_view(ui: &mut Ui, now: &NowInfo, palette: &Palette) {
    egui::ScrollArea::horizontal()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let total_width = ui.available_width().max(880.0);
            let day_col_w = 68.0;
            let hour_w = (total_width - day_col_w) / 11.0;
            let header_h = 36.0;
            let row_h = 76.0;

            let (grid_rect, _) = ui.allocate_exact_size(
                vec2(total_width, header_h + row_h * 5.0),
                egui::Sense::hover(),
            );

            let painter = ui.painter_at(grid_rect);

            // 1. Background Frame
            painter.rect_filled(grid_rect, Rounding::same(16.0), palette.card);
            painter.rect_stroke(grid_rect, Rounding::same(16.0), Stroke::new(1.0_f32, palette.border));

            // 2. Hour Headers (08:00 .. 19:00)
            for k in 8..19 {
                let x = grid_rect.min.x + day_col_w + (k - 8) as f32 * hour_w;
                let col_rect = Rect::from_min_size(pos2(x, grid_rect.min.y), vec2(hour_w, header_h));

                // Vertical grid lines
                painter.line_segment(
                    [pos2(x, grid_rect.min.y), pos2(x, grid_rect.max.y)],
                    Stroke::new(1.0_f32, palette.border),
                );

                // Hour text
                let hour_str = format!("{:02}:00", k);
                let next_str = format!("{:02}:00", k + 1);

                painter.text(
                    pos2(col_rect.center().x, col_rect.min.y + 11.0),
                    egui::Align2::CENTER_CENTER,
                    &hour_str,
                    egui::FontId::proportional(11.0),
                    palette.ink,
                );
                painter.text(
                    pos2(col_rect.center().x, col_rect.min.y + 24.0),
                    egui::Align2::CENTER_CENTER,
                    &next_str,
                    egui::FontId::proportional(9.0),
                    palette.mute,
                );
            }

            // 3. Day Rows (Mon .. Fri)
            for d in 1..=5 {
                let y = grid_rect.min.y + header_h + (d - 1) as f32 * row_h;

                // Horizontal divider
                painter.line_segment(
                    [pos2(grid_rect.min.x, y), pos2(grid_rect.max.x, y)],
                    Stroke::new(1.0_f32, palette.border),
                );

                // Day Label on Left Column
                let day_name = DAYS_SHORT[d as usize - 1];
                let is_today = now.day_of_week == d as u32;

                let day_color = if is_today { palette.ink } else { palette.mute };
                painter.text(
                    pos2(grid_rect.min.x + 12.0, y + row_h / 2.0),
                    egui::Align2::LEFT_CENTER,
                    day_name,
                    egui::FontId::proportional(13.0),
                    day_color,
                );

                if is_today {
                    // Green dot for today
                    painter.circle_filled(
                        pos2(grid_rect.min.x + 50.0, y + row_h / 2.0),
                        3.0,
                        Color32::from_rgb(34, 197, 94),
                    );
                }

                // Friday Break Block (12:00 – 15:00)
                if d == 5 {
                    let brk_x = grid_rect.min.x + day_col_w + (12.0 - 8.0) * hour_w + 3.0;
                    let brk_w = 3.0 * hour_w - 6.0;
                    let brk_rect = Rect::from_min_size(pos2(brk_x, y + 4.0), vec2(brk_w, row_h - 8.0));

                    painter.rect_filled(brk_rect, Rounding::same(8.0), palette.border);
                    painter.text(
                        brk_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Break · 12:00 – 15:00",
                        egui::FontId::proportional(10.5),
                        palette.mute,
                    );
                }

                // Schedule Blocks
                for ev in EVENTS.iter().filter(|e| e.day == d) {
                    let subj = ev.subject();
                    let bx = grid_rect.min.x + day_col_w + (ev.start as f32 - 8.0) * hour_w + 3.0;
                    let bw = (ev.end - ev.start) as f32 * hour_w - 6.0;
                    let by = y + 4.0;
                    let bh = row_h - 8.0;

                    let block_rect = Rect::from_min_size(pos2(bx, by), vec2(bw, bh));
                    let is_live = is_today
                        && now.minute_of_day >= (ev.start as u32 * 60)
                        && now.minute_of_day < (ev.end as u32 * 60);

                    // Block fill & outline
                    let bg_color = hsl_to_color32(subj.hue, 0.85, 0.55, if is_live { 0.32 } else { 0.20 });
                    let border_color = hsl_to_color32(subj.hue, 0.80, 0.55, if is_live { 0.85 } else { 0.35 });
                    let accent_bar_color = hsl_to_color32(subj.hue, 0.75, 0.52, 1.0);

                    painter.rect_filled(block_rect, Rounding::same(8.0), bg_color);
                    painter.rect_stroke(block_rect, Rounding::same(8.0), Stroke::new(1.0_f32, border_color));

                    // Left 3px accent strip
                    let left_strip = Rect::from_min_max(
                        block_rect.min,
                        pos2(block_rect.min.x + 3.0, block_rect.max.y),
                    );
                    painter.rect_filled(
                        left_strip,
                        Rounding { nw: 8.0, sw: 8.0, ne: 0.0, se: 0.0 },
                        accent_bar_color,
                    );

                    // Block Text Content
                    let tag_color = hsl_to_color32(subj.hue, 0.75, 0.48, 1.0);
                    let code_str = subj.code.split(" / ").next().unwrap_or(subj.code);
                    painter.text(
                        pos2(block_rect.min.x + 7.0, block_rect.min.y + 11.0),
                        egui::Align2::LEFT_CENTER,
                        code_str,
                        egui::FontId::proportional(10.5),
                        tag_color,
                    );

                    let time_str = format!("{:02}:00 – {:02}:00", ev.start, ev.end);
                    painter.text(
                        pos2(block_rect.min.x + 7.0, block_rect.min.y + 24.0),
                        egui::Align2::LEFT_CENTER,
                        &time_str,
                        egui::FontId::proportional(9.0),
                        palette.ink,
                    );

                    // Room & Subject name if space permits
                    if bw > 100.0 {
                        painter.text(
                            pos2(block_rect.min.x + 7.0, block_rect.min.y + 38.0),
                            egui::Align2::LEFT_CENTER,
                            subj.name,
                            egui::FontId::proportional(9.5),
                            palette.mute,
                        );
                        let meta_str = format!("{} · {}", ev.class_type, ev.room_short());
                        painter.text(
                            pos2(block_rect.min.x + 7.0, block_rect.min.y + 51.0),
                            egui::Align2::LEFT_CENTER,
                            &meta_str,
                            egui::FontId::proportional(9.0),
                            palette.mute,
                        );
                    }
                }
            }

            // 4. Live Current Time Indicator Marker (if weekday and within 08:00..19:00)
            if now.day_of_week >= 1 && now.day_of_week <= 5 && now.minute_of_day >= 480 && now.minute_of_day <= 1140 {
                let frac = (now.minute_of_day - 480) as f32 / 660.0;
                let cur_x = grid_rect.min.x + day_col_w + frac * (hour_w * 11.0);
                let row_y = grid_rect.min.y + header_h + (now.day_of_week - 1) as f32 * row_h;

                let line_color = Color32::from_rgb(239, 68, 68);

                // Vertical red marker through the current day's row
                painter.line_segment(
                    [pos2(cur_x, row_y), pos2(cur_x, row_y + row_h)],
                    Stroke::new(2.0_f32, line_color),
                );

                // Pulsing dot at top of marker
                painter.circle_filled(pos2(cur_x, row_y), 4.0, line_color);

                // Time label badge
                painter.text(
                    pos2(cur_x + 6.0, row_y + row_h - 10.0),
                    egui::Align2::LEFT_CENTER,
                    &now.time_str,
                    egui::FontId::proportional(10.0),
                    line_color,
                );
            }
        });
}
