use eframe::egui;

use crate::app::R2GlassApp;

impl R2GlassApp {
    pub(crate) fn console(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.strong("Console");
            ui.separator();
            ui.label(format!("{} commands", self.history.len()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Clear").on_hover_text("Clear command history").clicked() {
                    self.history.clear();
                }
            });
        });
        egui::ScrollArea::vertical()
            .id_salt("console_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let recent = self.history.iter().rev().cloned().collect::<Vec<_>>();
                if recent.is_empty() {
                    ui.add_space(4.0);
                    ui.label("No commands yet");
                }
                for entry in &recent {
                    let label = egui::Label::new(
                        egui::RichText::new(format!(":{}", entry.label())).monospace(),
                    )
                    .sense(egui::Sense::click());
                    if ui.add(label)
                        .on_hover_text("Click to re-run")
                        .clicked()
                        && let Some(command) = entry.replay_command()
                    {
                        self.run_command(command);
                    }
                }
            });
    }
}
