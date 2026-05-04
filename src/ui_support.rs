use eframe::egui;
use egui_code_editor::Syntax;

use crate::visual::{FunctionRow, VisualView};

pub(crate) fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(18, 21, 26);
    visuals.window_fill = egui::Color32::from_rgb(23, 27, 33);
    visuals.extreme_bg_color = egui::Color32::from_rgb(12, 14, 18);
    visuals.faint_bg_color = egui::Color32::from_rgb(31, 36, 44);
    visuals.selection.bg_fill = egui::Color32::from_rgb(42, 93, 143);
    ctx.set_visuals(visuals);

    apply_typography(ctx);
}

fn apply_typography(ctx: &egui::Context) {
    use egui::TextStyle::*;
    let mut style = (*ctx.global_style()).clone();
    style.text_styles = [
        (Heading,   egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        (Body,      egui::FontId::new(13.0, egui::FontFamily::Proportional)),
        (Monospace, egui::FontId::new(13.0, egui::FontFamily::Monospace)),
        (Small,     egui::FontId::new(11.0, egui::FontFamily::Proportional)),
        (Button,    egui::FontId::new(13.0, egui::FontFamily::Proportional)),
    ]
    .into();
    ctx.set_global_style(style);
}

pub(crate) fn output_syntax(view: VisualView) -> Syntax {
    match view {
        VisualView::Decompile => c_like_syntax(),
        VisualView::Disassembly | VisualView::Graph | VisualView::Hex | VisualView::Info => {
            Syntax::asm()
        }
    }
}

fn c_like_syntax() -> Syntax {
    Syntax::new("C")
        .with_comment("//")
        .with_comment_multiline(["/*", "*/"])
        .with_keywords([
            "break", "case", "const", "continue", "default", "do", "else", "enum", "for", "goto",
            "if", "return", "sizeof", "static", "struct", "switch", "typedef", "union", "while",
        ])
        .with_types([
            "bool", "char", "double", "float", "int", "int16_t", "int32_t", "int64_t", "int8_t",
            "long", "short", "size_t", "uint16_t", "uint32_t", "uint64_t", "uint8_t", "void",
        ])
        .with_special(["NULL", "false", "true"])
}

pub(crate) fn filtered_functions(rows: &[FunctionRow], filter: &str) -> Vec<FunctionRow> {
    let needle = filter.trim().to_ascii_lowercase();
    rows.iter()
        .filter(|row| needle.is_empty() || row.name.to_ascii_lowercase().contains(&needle))
        .cloned()
        .collect()
}

pub(crate) fn filtered_lines(rows: &[String], filter: &str) -> Vec<String> {
    let needle = filter.trim().to_ascii_lowercase();
    rows.iter()
        .filter(|row| needle.is_empty() || row.to_ascii_lowercase().contains(&needle))
        .cloned()
        .collect()
}

