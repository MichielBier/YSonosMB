#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
extern crate sonos;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 750.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Your Sonos,My Bitch",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            allowed_to_close: false,
            show_confirmation_dialog: false,
            devices: sonos::discover().unwrap(),
        }
    }
}

struct MyApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    devices: Vec<sonos::Speaker>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(
                egui::RichText::new("Your Sonos is my Bitch")
                    .monospace()
                    .heading()
                    .color(egui::Color32::from_rgb(0, 143, 17)),
            );
            for device in &self.devices {
                ui.separator();
                ui.heading(&device.name);
                ui.horizontal_top(|ui| {
                    ui.label(format!(
                        "Device: '{}', Volume: {} Muted: {}",
                        &device.name,
                        &device.volume().unwrap(),
                        &device.muted().unwrap()
                    ));
                });

                ui.horizontal(|ui| {
                    ui.add(
                        egui::Slider::new(&mut device.volume().unwrap().to_owned(), 0..=100)
                            .text("Volume"),
                    );
                    if ui.button("-").clicked() {
                        let vol = &device.volume().unwrap() - 1;
                        let _ = &device.set_volume(vol).unwrap();
                    }
                    if ui.button("+").clicked() {
                        let vol = &device.volume().unwrap() + 1;
                        let _ = &device.set_volume(vol).unwrap();
                    }
                });
                ui.horizontal(|ui| {
                    if ui
                        .button(
                            egui::RichText::new("⏮")
                                .heading()
                                .color(egui::Color32::from_rgb(0, 143, 17)),
                        )
                        .clicked()
                    {
                        let _ = &device.previous().unwrap();
                    }
                    if ui
                        .button(
                            egui::RichText::new("⏸")
                                .heading()
                                .color(egui::Color32::from_rgb(0, 143, 17)),
                        )
                        .clicked()
                    {
                        let _ = &device.pause().unwrap();
                    }
                    if ui
                        .button(
                            egui::RichText::new("▶")
                                .heading()
                                .color(egui::Color32::from_rgb(0, 143, 17)),
                        )
                        .clicked()
                    {
                        let _ = &device.play().unwrap();
                    }
                    if ui
                        .button(
                            egui::RichText::new("⏹")
                                .heading()
                                .color(egui::Color32::from_rgb(0, 143, 17)),
                        )
                        .clicked()
                    {
                        let _ = &device.stop().unwrap();
                    }
                    if ui
                        .button(
                            egui::RichText::new("⏭")
                                .heading()
                                .color(egui::Color32::from_rgb(0, 143, 17)),
                        )
                        .clicked()
                    {
                        let _ = &device.next().unwrap();
                    }
                    if ui
                        .button(
                            egui::RichText::new("🔇")
                                .heading()
                                .color(egui::Color32::from_rgb(0, 143, 17)),
                        )
                        .clicked()
                    {
                        let muted = &device.muted().unwrap();
                        if muted == &true {
                            let _ = &device.unmute().unwrap();
                        } else {
                            let _ = &device.mute().unwrap();
                        }
                    }
                });
            }
        });
        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Are you a bitch?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
    }
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }
}
