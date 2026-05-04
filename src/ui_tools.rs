use eframe::egui;

use crate::app::R2GlassApp;
use crate::visual::{BreakpointAction, EditAction, SearchAction};

impl R2GlassApp {
    pub(crate) fn editing_tools(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("Edit")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Rename function");
                ui.horizontal(|ui| {
                    let input = ui.add(
                        egui::TextEdit::singleline(&mut self.rename_input)
                            .hint_text("new name")
                            .desired_width(120.0),
                    );
                    if ui.button("Rename").on_hover_text("Rename function at cursor (afn)").clicked()
                        || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.edit_action(&EditAction::RenameFunction(self.rename_input.clone()));
                    }
                });
                ui.label("Comment at cursor");
                ui.horizontal(|ui| {
                    let input = ui.add(
                        egui::TextEdit::singleline(&mut self.comment_input)
                            .hint_text("comment text")
                            .desired_width(120.0),
                    );
                    if ui.button("Comment").on_hover_text("Set comment at current address (CC)").clicked()
                        || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.edit_action(&EditAction::SetComment(self.comment_input.clone()));
                    }
                });
                ui.label("Flag at cursor");
                ui.horizontal(|ui| {
                    let input = ui.add(
                        egui::TextEdit::singleline(&mut self.flag_input)
                            .hint_text("flag name")
                            .desired_width(120.0),
                    );
                    if ui.button("Flag").on_hover_text("Set flag at current address (f)").clicked()
                        || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.edit_action(&EditAction::SetFlag(self.flag_input.clone()));
                    }
                });
                ui.label("Patch hex bytes");
                ui.horizontal(|ui| {
                    let input = ui.add(
                        egui::TextEdit::singleline(&mut self.patch_hex_input)
                            .hint_text("e.g. 90 90")
                            .desired_width(120.0),
                    );
                    if ui.button("Patch").on_hover_text("Write hex bytes at cursor (wx)").clicked()
                        || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.edit_action(&EditAction::PatchHex(self.patch_hex_input.clone()));
                    }
                });
            });
    }

    pub(crate) fn search_tools(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("Search")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Text");
                ui.horizontal(|ui| {
                    let input = ui.add(
                        egui::TextEdit::singleline(&mut self.text_search_input)
                            .hint_text("search string")
                            .desired_width(120.0),
                    );
                    if ui.button("Find").on_hover_text("Search for text string (/ )").clicked()
                        || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.search_action(&SearchAction::Text(self.text_search_input.clone()));
                    }
                    if ui.button("Next").on_hover_text("Repeat last search (/)").clicked() {
                        self.search_action(&SearchAction::Next);
                    }
                });
                ui.label("Hex bytes");
                ui.horizontal(|ui| {
                    let input = ui.add(
                        egui::TextEdit::singleline(&mut self.hex_search_input)
                            .hint_text("e.g. 414243")
                            .desired_width(120.0),
                    );
                    if ui.button("Find").on_hover_text("Search for hex bytes (/x)").clicked()
                        || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.search_action(&SearchAction::Hex(self.hex_search_input.clone()));
                    }
                    if ui.button("Next").on_hover_text("Repeat last search (/)").clicked() {
                        self.search_action(&SearchAction::Next);
                    }
                });
            });
    }

    pub(crate) fn breakpoint_tools(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("Breakpoints")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let input = ui.add(
                        egui::TextEdit::singleline(&mut self.breakpoint_input)
                            .hint_text("address or symbol")
                            .desired_width(120.0),
                    );
                    if ui.button("Add").on_hover_text("Set breakpoint (db)").clicked()
                        || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.breakpoint_action(&BreakpointAction::Add(
                            self.breakpoint_input.clone(),
                        ));
                    }
                    if ui.button("Remove").on_hover_text("Remove breakpoint (db-)").clicked() {
                        self.breakpoint_action(&BreakpointAction::Remove(
                            self.breakpoint_input.clone(),
                        ));
                    }
                    if ui.button("Clear").on_hover_text("Clear all breakpoints (db-*)").clicked() {
                        self.breakpoint_action(&BreakpointAction::ClearAll);
                    }
                });
            });
    }
}
