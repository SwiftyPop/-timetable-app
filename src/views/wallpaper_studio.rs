use egui::{vec2, Color32, Rounding, Stroke, Ui};
use crate::theme::Palette;
use crate::wallpaper::{render_wallpaper, WallpaperTheme, PRESETS};

pub struct WallpaperStudioState {
    pub selected_theme: WallpaperTheme,
    pub selected_preset_idx: usize,
    pub status_msg: String,
    pub last_png_bytes: Option<Vec<u8>>,
}

impl Default for WallpaperStudioState {
    fn default() -> Self {
        Self {
            selected_theme: WallpaperTheme::Olive,
            selected_preset_idx: 0, // Android 1080x2400
            status_msg: String::new(),
            last_png_bytes: None,
        }
    }
}

pub fn render_wallpaper_studio(
    ui: &mut Ui,
    state: &mut WallpaperStudioState,
    palette: &Palette,
    open: &mut bool,
) {
    let frame = egui::Frame::none()
        .fill(palette.card)
        .stroke(Stroke::new(1.0_f32, palette.border))
        .rounding(Rounding::same(20.0))
        .inner_margin(egui::Margin::same(18.0));

    frame.show(ui, |ui| {
        // Header
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("PHONE & DESKTOP WALLPAPER STUDIO")
                    .size(11.0)
                    .strong()
                    .color(palette.mute),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("✕ Close").clicked() {
                    *open = false;
                }
            });
        });

        ui.add_space(10.0);

        ui.horizontal_wrapped(|ui| {
            // Theme selection pills
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Palette:").size(12.5).color(palette.mute));

                let themes = [
                    WallpaperTheme::Olive,
                    WallpaperTheme::Midnight,
                    WallpaperTheme::Cream,
                ];

                for th in themes {
                    let active = state.selected_theme == th;
                    let (bg, fg) = if active {
                        (palette.accent, palette.accent_ink)
                    } else {
                        (palette.card, palette.mute)
                    };

                    let btn = egui::Button::new(egui::RichText::new(th.name()).size(12.0).strong().color(fg))
                        .fill(bg)
                        .stroke(Stroke::new(1.0_f32, if active { palette.accent } else { palette.border }))
                        .rounding(Rounding::same(8.0));

                    if ui.add(btn).clicked() {
                        state.selected_theme = th;
                        state.last_png_bytes = None; // Invalidate cache
                    }
                }
            });

            ui.add_space(16.0);

            // Preset Dropdown
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Size:").size(12.5).color(palette.mute));

                let current_label = PRESETS[state.selected_preset_idx].label;
                egui::ComboBox::from_id_salt("wp_preset_combo")
                    .selected_text(current_label)
                    .show_ui(ui, |ui| {
                        for (i, p) in PRESETS.iter().enumerate() {
                            if ui.selectable_value(&mut state.selected_preset_idx, i, p.label).clicked() {
                                state.last_png_bytes = None; // Invalidate cache
                            }
                        }
                    });
            });
        });

        ui.add_space(12.0);

        // Action Buttons
        ui.horizontal(|ui| {
            let preset = &PRESETS[state.selected_preset_idx];

            #[cfg(not(target_arch = "wasm32"))]
            {
                let set_wp_btn = egui::Button::new(
                    egui::RichText::new("🖥 Set as Windows Wallpaper")
                        .size(13.0)
                        .strong()
                        .color(palette.accent_ink),
                )
                .fill(palette.accent)
                .rounding(Rounding::same(10.0))
                .min_size(vec2(200.0, 34.0));

                if ui.add(set_wp_btn).clicked() {
                    match render_wallpaper(preset, state.selected_theme) {
                        Ok(bytes) => {
                            match crate::platform::set_desktop_wallpaper(&bytes) {
                                Ok(()) => {
                                    state.status_msg = "Desktop wallpaper updated successfully ✓".to_string();
                                }
                                Err(e) => {
                                    state.status_msg = format!("Failed to set wallpaper: {}", e);
                                }
                            }
                            state.last_png_bytes = Some(bytes);
                        }
                        Err(e) => {
                            state.status_msg = format!("Error rendering wallpaper: {}", e);
                        }
                    }
                }
            }

            let save_btn = egui::Button::new(
                egui::RichText::new("⬇ Save PNG")
                    .size(13.0)
                    .strong()
                    .color(palette.ink),
            )
            .fill(palette.card)
            .stroke(Stroke::new(1.0_f32, palette.border))
            .rounding(Rounding::same(10.0))
            .min_size(vec2(120.0, 34.0));

            if ui.add(save_btn).clicked() {
                match render_wallpaper(preset, state.selected_theme) {
                    Ok(bytes) => {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let file_name = format!(
                                "timetable-wallpaper-{}-{}x{}.png",
                                state.selected_theme.name().to_lowercase(),
                                preset.width,
                                preset.height
                            );
                            let user_dirs = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
                            let out_path = std::path::Path::new(&user_dirs).join("Downloads").join(&file_name);
                            if let Ok(mut f) = std::fs::File::create(&out_path) {
                                use std::io::Write;
                                let _ = f.write_all(&bytes);
                                state.status_msg = format!("Saved to {} ✓", out_path.display());
                            } else {
                                state.status_msg = "Could not save to Downloads folder".to_string();
                            }
                        }
                        state.last_png_bytes = Some(bytes);
                    }
                    Err(e) => {
                        state.status_msg = format!("Error: {}", e);
                    }
                }
            }
        });

        if !state.status_msg.is_empty() {
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new(&state.status_msg)
                    .size(12.0)
                    .strong()
                    .color(Color32::from_rgb(34, 197, 94)),
            );
        }
    });

    ui.add_space(10.0);
}
