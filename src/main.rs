mod hook;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([200.0, 100.0])
            .with_always_on_top()
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Keyboard Lock",
        options,
        Box::new(|_cc| {
            Box::new(KeyboardLockApp::default())
        }),
    )
}

struct KeyboardLockApp {
    is_locked: bool,
}

impl Default for KeyboardLockApp {
    fn default() -> Self {
        Self {
            is_locked: false,
        }
    }
}

impl eframe::App for KeyboardLockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Keyboard Lock");
                ui.add_space(10.0);

                let button_text = if self.is_locked {
                    "UNLOCK Keyboard"
                } else {
                    "LOCK Keyboard"
                };

                let color = if self.is_locked {
                    egui::Color32::from_rgb(200, 50, 50)
                } else {
                    egui::Color32::from_rgb(50, 150, 50)
                };

                let button = egui::Button::new(
                    egui::RichText::new(button_text)
                        .color(egui::Color32::WHITE)
                        .size(18.0)
                ).fill(color);

                if ui.add_sized([180.0, 40.0], button).clicked() {
                    self.is_locked = !self.is_locked;
                    hook::set_locked(self.is_locked);
                }
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        hook::uninstall_hook();
    }
}
