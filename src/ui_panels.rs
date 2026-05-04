use eframe::egui;

use crate::app::R2GlassApp;
use crate::ui_support::{filtered_functions, filtered_lines};
use crate::visual::{FunctionRow, VisualAction, clickable_seek_target};
use crate::workspace::{InspectorTab, NavigatorTab};

impl R2GlassApp {
    pub(crate) fn symbols_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Navigator");
        ui.separator();
        navigator_tabs(ui, &mut self.navigator_tab);
        ui.separator();
        match self.navigator_tab {
            NavigatorTab::Symbols => self.symbols_tab(ui),
            NavigatorTab::Data => self.data_tab(ui),
            NavigatorTab::References => self.references_tab(ui),
        }
    }

    fn symbols_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            ui.label("Functions");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("{}", self.functions.len()));
            });
        });
        let filter = ui.add(
            egui::TextEdit::singleline(&mut self.symbol_filter)
                .hint_text("filter functions...")
                .desired_width(f32::INFINITY),
        );
        let _ = filter.lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter));
        let functions = filtered_functions(&self.functions, &self.symbol_filter);
        self.function_list(ui, &functions);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Flags");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("{}", self.flags.len()));
            });
        });
        ui.add(
            egui::TextEdit::singleline(&mut self.flag_filter)
                .hint_text("filter flags...")
                .desired_width(f32::INFINITY),
        );
        let flags = filtered_lines(&self.flags, &self.flag_filter);
        self.side_list(ui, "Flags", &flags);
    }

    fn references_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(2.0);
        ui.label("Xrefs");
        ui.add(
            egui::TextEdit::singleline(&mut self.xref_filter)
                .hint_text("filter xrefs...")
                .desired_width(f32::INFINITY),
        );
        let xrefs = filtered_lines(&self.xrefs, &self.xref_filter);
        self.side_list(ui, "Xrefs", &xrefs);
    }

    fn data_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(2.0);
        ui.label("Data filter");
        ui.add(
            egui::TextEdit::singleline(&mut self.data_filter)
                .hint_text("filter data...")
                .desired_width(f32::INFINITY),
        );
        let strings = filtered_lines(&self.strings, &self.data_filter);
        let imports = filtered_lines(&self.imports, &self.data_filter);
        let exports = filtered_lines(&self.exports, &self.data_filter);
        let sections = filtered_lines(&self.sections, &self.data_filter);
        self.side_list(ui, "Strings", &strings);
        self.side_list(ui, "Imports", &imports);
        self.side_list(ui, "Exports", &exports);
        self.side_list(ui, "Sections", &sections);
    }

    pub(crate) fn inspector(&mut self, ui: &mut egui::Ui) {
        ui.heading("Inspector");
        inspector_tabs(ui, &mut self.inspector_tab);
        ui.separator();
        if ui.button("Refresh Context").clicked() {
            self.refresh_inspector();
        }
        ui.separator();
        match self.inspector_tab {
            InspectorTab::Context => {
                ui.horizontal_wrapped(|ui| {
                    ui.label(format!("offset {}", self.current_offset.trim()));
                    if ui.button("Xrefs").clicked() {
                        self.show_current_function_xrefs();
                    }
                    if ui.button("Callees").clicked() {
                        self.show_current_function_callees();
                    }
                    if ui.button("Graph").clicked() {
                        self.show_current_function_graph();
                    }
                    if ui.button("Decompile").clicked() {
                        self.view = crate::visual::VisualView::Decompile;
                        self.refresh_view();
                    }
                });
                text_block(ui, "Current Function", &self.function_info, 8);
                text_block(ui, "Callers", &self.callers_output, 8);
                text_block(ui, "Callees", &self.callees_output, 8);
                text_block(ui, "Search Results", &self.search_results.join("\n"), 8);
            }
            InspectorTab::Debug => {
                text_block(ui, "Registers", &self.register_output, 12);
                text_block(ui, "Stack", &self.stack_output, 10);
                text_block(ui, "Breakpoints", &self.breakpoint_output, 6);
                self.breakpoint_tools(ui);
            }
            InspectorTab::Tools => {
                self.search_tools(ui);
                self.editing_tools(ui);
            }
        }
    }

    fn side_list(&mut self, ui: &mut egui::Ui, title: &str, rows: &[String]) {
        egui::CollapsingHeader::new(format!("{} ({})", title, rows.len()))
            .default_open(true)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(180.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for row in rows {
                            let label =
                                egui::Label::new(egui::RichText::new(row).monospace().small())
                                    .sense(egui::Sense::click());
                            if ui.add(label)
                                .on_hover_text("Click to seek to address")
                                .clicked()
                                && let Some(target) = clickable_seek_target(row)
                            {
                                self.apply_action(&VisualAction::Seek(target));
                            }
                        }
                    });
            });
    }

    fn function_list(&mut self, ui: &mut egui::Ui, rows: &[FunctionRow]) {
        egui::CollapsingHeader::new(format!("Functions ({})", rows.len()))
            .default_open(true)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(260.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for row in rows {
                            let label =
                                egui::Label::new(egui::RichText::new(row.label()).monospace())
                                    .sense(egui::Sense::click());
                            if ui.add(label)
                                .on_hover_text("Click to seek to function")
                                .clicked()
                            {
                                self.apply_action(&VisualAction::Seek(row.address.clone()));
                            }
                        }
                    });
            });
    }
}

fn tab_bar<T: Eq + Copy>(
    ui: &mut egui::Ui,
    all: &[T],
    label: impl Fn(T) -> &'static str,
    hover: impl Fn(T) -> &'static str,
    selected: &mut T,
) {
    ui.horizontal(|ui| {
        ui.add_space(2.0);
        for &tab in all {
            if ui.selectable_label(*selected == tab, label(tab))
                .on_hover_text(hover(tab))
                .clicked()
            {
                *selected = tab;
            }
        }
    });
}

fn navigator_tabs(ui: &mut egui::Ui, selected: &mut NavigatorTab) {
    tab_bar(ui, &NavigatorTab::ALL, NavigatorTab::label, NavigatorTab::hover_text, selected);
}

fn inspector_tabs(ui: &mut egui::Ui, selected: &mut InspectorTab) {
    tab_bar(ui, &InspectorTab::ALL, InspectorTab::label, InspectorTab::hover_text, selected);
}

fn text_block(ui: &mut egui::Ui, title: &str, text: &str, rows: usize) {
    ui.collapsing(title, |ui| {
        egui::ScrollArea::vertical()
            .max_height(block_height(rows))
            .show(ui, |ui| {
                if text.trim().is_empty() {
                    ui.label("No data");
                } else {
                    ui.monospace(text);
                }
            });
    });
}

const fn block_height(rows: usize) -> f32 {
    match rows {
        0..=6 => 108.0,
        7..=8 => 144.0,
        9..=10 => 180.0,
        _ => 216.0,
    }
}
