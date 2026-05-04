use std::path::PathBuf;

use eframe::egui;

use crate::app::R2GlassApp;
use crate::ui_support::apply_theme;
use crate::visual::{
    AnalysisAction, DebugAction, QuickViewAction, SeekStep, VisualAction, VisualView,
};

impl R2GlassApp {
    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.strong("r2glass");
            ui.separator();
            for view in [
                VisualView::Disassembly,
                VisualView::Hex,
                VisualView::Graph,
                VisualView::Decompile,
                VisualView::Info,
            ] {
                if ui
                    .selectable_label(self.view == view, view.label())
                    .on_hover_text(format!("Switch to {} view [P]", view.label()))
                    .clicked()
                {
                    self.view = view;
                    self.refresh_view();
                }
            }
            ui.separator();
            for (label, action) in [
                ("Analyze", AnalysisAction::Full),
                ("Deep", AnalysisAction::Deep),
                ("Types", AnalysisAction::Types),
            ] {
                if ui.button(label).on_hover_text(action.hover_text()).clicked() {
                    self.analysis_action(action);
                }
            }
            if ui.button("⟳").on_hover_text("Refresh current view").clicked() {
                self.refresh_view();
            }
            if ui.button("◀").on_hover_text("Page up [PageUp]").clicked() {
                self.apply_action(&VisualAction::Step(SeekStep::PageUp));
            }
            if ui.button("▶").on_hover_text("Page down [PageDown]").clicked() {
                self.apply_action(&VisualAction::Step(SeekStep::PageDown));
            }
            if ui.button("All").on_hover_text("Decompile all functions in background").clicked() {
                self.decompile_all();
            }
            ui.separator();
            for (label, action) in [
                ("Headers", QuickViewAction::Headers),
                ("Symbols", QuickViewAction::Symbols),
                ("Relocs", QuickViewAction::Relocations),
                ("Entries", QuickViewAction::Entrypoints),
                ("Comments", QuickViewAction::Comments),
            ] {
                if ui.button(label).on_hover_text(action.hover_text()).clicked() {
                    self.quick_view_action(action);
                }
            }
        });
    }

    fn debug_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.strong("Debug");
            ui.separator();
            for (label, action) in [
                ("Start", DebugAction::Start),
                ("Continue", DebugAction::Continue),
                ("Step Into", DebugAction::StepInto),
                ("Step Over", DebugAction::StepOver),
                ("Break Here", DebugAction::BreakHere),
                ("Registers", DebugAction::Registers),
                ("Backtrace", DebugAction::Backtrace),
            ] {
                if ui.button(label).on_hover_text(action.hover_text()).clicked() {
                    self.debug_action(action);
                }
            }
            ui.separator();
            if ui.button("Help").on_hover_text("Show debug setup help").clicked() {
                crate::visual::debug_error_hint("possibly unsigned r2")
                    .unwrap_or("No debug help available")
                    .clone_into(&mut self.output);
            }
        });
    }

    fn command_bar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            ui.label("Seek:");
            let seek = ui.add(
                egui::TextEdit::singleline(&mut self.seek_input)
                    .desired_width(180.0)
                    .hint_text("address or symbol"),
            );
            if ui.button("Go").on_hover_text("Seek to address/symbol [Enter]").clicked()
                || (seek.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)))
            {
                self.apply_action(&VisualAction::Seek(self.seek_input.clone()));
            }
            ui.separator();
            ui.label("Cmd:");
            let command = ui.add(
                egui::TextEdit::singleline(&mut self.command_input)
                    .desired_width(240.0)
                    .hint_text("r2 command"),
            );
            if ui.button("Run").on_hover_text("Execute r2 command [Enter]").clicked()
                || (command.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)))
            {
                let command = self.command_input.clone();
                self.run_command(&command);
            }
            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(4.0);
                if self.background_job.is_some() {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 196, 0),
                        "⏳ Running...",
                    );
                }
            });
            ui.colored_label(egui::Color32::GRAY, &self.status);
        });
    }
}

impl eframe::App for R2GlassApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        apply_theme(&ctx);
        self.poll_background_job(&ctx);
        self.handle_keys(&ctx);

        let title = self
            .target_path
            .as_ref()
            .map_or_else(|| "r2glass".to_owned(), |p| format!("r2glass - {}", p.display()));
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(title));

        if self.session.is_none() {
            // Welcome screen — no target loaded
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.heading("r2glass");
                    ui.label("radare2 GUI frontend");
                    ui.separator();
                    ui.add_space(16.0);
                    egui::Frame::new()
                        .inner_margin(egui::Margin::symmetric(20, 14))
                        .fill(ctx.global_style().visuals.panel_fill)
                        .stroke(ui.style().visuals.window_stroke())
                        .corner_radius(6.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.set_min_width(400.0);
                                ui.label("Target:");
                                let response = ui.add(
                                    egui::TextEdit::singleline(&mut self.target_input)
                                        .desired_width(280.0)
                                        .hint_text("/path/to/binary"),
                                );
                                let opened = ui.button("Open").clicked()
                                    || (response.lost_focus()
                                        && ui.input(|i| i.key_pressed(egui::Key::Enter)));
                                if opened {
                                    self.open_target(&PathBuf::from(self.target_input.clone()));
                                }
                            });
                        });
                    ui.add_space(6.0);
                    ui.small("Or drag-and-drop a binary onto this window");
                    ui.add_space(4.0);
                    ui.small("Pass a binary path as a CLI argument:");
                    ui.monospace("r2glass ./binary");
                });
            });
            return;
        }

        egui::Panel::top("top").show_inside(ui, |ui| {
            self.top_bar(ui);
            self.debug_bar(ui);
            self.command_bar(ui);
        });
        egui::Panel::left("symbols")
            .resizable(true)
            .min_size(180.0)
            .default_size(280.0)
            .show_inside(ui, |ui| self.symbols_panel(ui));
        egui::Panel::right("inspector")
            .resizable(true)
            .min_size(180.0)
            .default_size(320.0)
            .show_inside(ui, |ui| self.inspector(ui));
        egui::Panel::bottom("console")
            .resizable(true)
            .min_size(80.0)
            .default_size(120.0)
            .show_inside(ui, |ui| self.console(ui));
        egui::CentralPanel::default().show_inside(ui, |ui| self.output_editor(ui));
    }
}
