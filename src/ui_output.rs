use eframe::egui;
use egui_code_editor::{CodeEditor, ColorTheme};

use crate::app::R2GlassApp;
use crate::ui_support::output_syntax;
use crate::visual::{VisualAction, clickable_seek_target};

impl R2GlassApp {
    pub(crate) fn output_editor(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.strong(self.view.label());
            ui.separator();
            ui.checkbox(&mut self.output_line_numbers, "lines")
                .on_hover_text("Toggle line number display");
            ui.checkbox(&mut self.output_wrap, "wrap")
                .on_hover_text("Toggle word wrapping");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let offset = self.current_offset.trim();
                if !offset.is_empty() {
                    ui.colored_label(egui::Color32::from_rgb(140, 180, 220), format!("@ {offset}"));
                }
            });
        });

        let lines: Vec<String> = self.output.lines().map(ToOwned::to_owned).collect();
        let line_count = lines.len().max(1);
        let num_width = if self.output_line_numbers {
            line_count.to_string().len()
        } else {
            0
        };

        let theme = ColorTheme::AYU_DARK;
        let editor = CodeEditor::default()
            .id_source("r2_output")
            .with_syntax(output_syntax(self.view))
            .with_theme(theme)
            .with_ui_fontsize(ui);

        let scroll = if self.output_wrap {
            egui::ScrollArea::vertical().id_salt("r2_output_scroll")
        } else {
            egui::ScrollArea::both().id_salt("r2_output_scroll")
        };
        scroll.auto_shrink([false, false]).show(ui, |ui| {
            egui::Frame::new()
                .fill(theme.bg())
                .inner_margin(egui::Margin::symmetric(6, 2))
                .show(ui, |ui| {
                    if lines.is_empty() {
                        ui.monospace("");
                        return;
                    }
                    for (i, line) in lines.iter().enumerate() {
                        let prefix = if self.output_line_numbers {
                            format!("{:>width$}  ", i + 1, width = num_width)
                        } else {
                            String::new()
                        };
                        let display = format!("{prefix}{line}");
                        let target = clickable_seek_target(line);

                        let mut layout_job =
                            egui_code_editor::highlighting::highlight(ui.ctx(), &editor, &display);
                        layout_job.halign = egui::Align::LEFT;

                        let mut label = egui::Label::new(layout_job);
                        if target.is_some() {
                            label = label.sense(egui::Sense::click());
                        }
                        let response = ui.add(label);

                        if let Some(addr) = target {
                            if ui.input(|i| i.modifiers.any()) && response.clicked() {
                                self.apply_action(&VisualAction::Seek(addr));
                            }
                        }
                    }
                });
        });
    }
}
