use eframe::App;
use egui::{vec2, Key, Rounding, Stroke};

use crate::models::{total_class_hours, SUBJECTS};
use crate::state::{calculate_now_info, AppView, NowInfo};
use crate::theme::{configure_visuals, get_palette, setup_fonts, ThemeMode};
use crate::views::{
    render_day_view, render_now_card, render_toolbar, render_wallpaper_studio, render_week_view,
    WallpaperStudioState,
};

pub struct TimetableApp {
    pub view: AppView,
    pub selected_day: u8,
    pub theme_mode: ThemeMode,
    pub wallpaper_open: bool,
    pub wallpaper_state: WallpaperStudioState,
    pub now_info: NowInfo,
    pub start_time: std::time::Instant,
}

impl TimetableApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        let now_info = calculate_now_info();
        let default_day = if now_info.day_of_week >= 1 && now_info.day_of_week <= 5 {
            now_info.day_of_week as u8
        } else {
            1 // Monday
        };

        Self {
            view: AppView::Day,
            selected_day: default_day,
            theme_mode: ThemeMode::Auto,
            wallpaper_open: false,
            wallpaper_state: WallpaperStudioState::default(),
            now_info,
            start_time: std::time::Instant::now(),
        }
    }
}

impl App for TimetableApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request periodic repaint every 1 second for live clock & countdown updates
        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        // Update now info every frame
        self.now_info = calculate_now_info();

        // Keyboard navigation
        ctx.input(|i| {
            if i.key_pressed(Key::Escape) && self.wallpaper_open {
                self.wallpaper_open = false;
            }
            if self.view == AppView::Day {
                if i.key_pressed(Key::ArrowLeft) && self.selected_day > 1 {
                    self.selected_day -= 1;
                } else if i.key_pressed(Key::ArrowRight) && self.selected_day < 5 {
                    self.selected_day += 1;
                }
            }
        });

        // Theme configuration
        let system_dark = ctx.style().visuals.dark_mode;
        let palette = get_palette(self.theme_mode, system_dark);
        let effective_dark = match self.theme_mode {
            ThemeMode::Auto => system_dark,
            ThemeMode::Light => false,
            ThemeMode::Dark | ThemeMode::Oled => true,
        };
        configure_visuals(ctx, &palette, effective_dark);

        let elapsed = self.start_time.elapsed().as_secs_f64();

        // Main Application Window Frame
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = vec2(0.0, 10.0);

                    // 1. Eyebrow Header
                    ui.label(
                        egui::RichText::new("UniMAP · UR6522002 · Y2G3 · Semester 1 2026/27")
                            .size(11.5)
                            .strong()
                            .color(palette.mute),
                    );

                    // 2. Title
                    ui.label(
                        egui::RichText::new("My Timetable")
                            .size(24.0)
                            .strong()
                            .color(palette.ink),
                    );

                    // 3. Stats Badge Carousel
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = vec2(6.0, 6.0);

                        let stats = [
                            format!("{} class hours", total_class_hours()),
                            format!("{} subjects", SUBJECTS.len()),
                            "First class Mon 08:00".to_string(),
                            "Last class Wed 19:00".to_string(),
                        ];

                        for s in &stats {
                            let frame = egui::Frame::none()
                                .fill(palette.card)
                                .stroke(Stroke::new(1.0_f32, palette.border))
                                .rounding(Rounding::same(999.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 4.0));

                            frame.show(ui, |ui| {
                                ui.label(egui::RichText::new(s).size(11.5).strong().color(palette.ink));
                            });
                        }
                    });

                    // 4. Live "Now" Status Banner
                    render_now_card(ui, &self.now_info, &palette, elapsed);

                    // 5. Toolbar & View Navigation
                    render_toolbar(
                        ui,
                        &palette,
                        &mut self.view,
                        &mut self.selected_day,
                        self.now_info.day_of_week,
                        &mut self.theme_mode,
                        &mut self.wallpaper_open,
                    );

                    // 6. Wallpaper Studio Drawer (if open)
                    if self.wallpaper_open {
                        render_wallpaper_studio(
                            ui,
                            &mut self.wallpaper_state,
                            &palette,
                            &mut self.wallpaper_open,
                        );
                    }

                    // 7. Active View Content
                    match self.view {
                        AppView::Day => {
                            render_day_view(ui, self.selected_day, &self.now_info, &palette);
                        }
                        AppView::Week => {
                            render_week_view(ui, &self.now_info, &palette);
                        }
                    }

                    // 8. Legend & Note
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = vec2(14.0, 6.0);
                        for s in SUBJECTS {
                            ui.horizontal(|ui| {
                                let (dot_rect, _) = ui.allocate_exact_size(vec2(8.0, 8.0), egui::Sense::hover());
                                let c = crate::theme::hsl_to_color32(s.hue, 0.75, 0.55, 1.0);
                                ui.painter().rect_filled(dot_rect, Rounding::same(2.0), c);
                                let label_str = format!("{} · {}", s.code, s.name);
                                ui.label(egui::RichText::new(label_str).size(11.5).color(palette.mute));
                            });
                        }
                    });

                    ui.label(
                        egui::RichText::new("English, Malay and Philosophy classes left out.")
                            .size(11.0)
                            .color(palette.mute),
                    );

                    ui.add_space(16.0);
                });
        });
    }
}
