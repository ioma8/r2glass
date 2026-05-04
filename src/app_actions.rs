use eframe::egui;

use crate::app::R2GlassApp;
use crate::visual::{
    AnalysisAction, BreakpointAction, DebugAction, EditAction, QuickViewAction, SearchAction,
    SeekStep, VisualAction, debug_error_hint, parse_functions,
};
use crate::visual::{VisualView, decompiler_crashed};

impl R2GlassApp {
    pub(crate) fn refresh_view(&mut self) {
        if self.background_job.is_some() {
            "Background command running".clone_into(&mut self.status);
            return;
        }
        if self.view == VisualView::Decompile {
            self.decompile_current();
        } else {
            self.run_command(self.view.render_command());
        }
        self.refresh_lists();
        self.refresh_inspector();
    }

    fn refresh_lists(&mut self) {
        if self.background_job.is_some() {
            return;
        }
        self.functions = self.function_rows();
        self.flags = self.list_command("f");
        self.xrefs = self.list_command("axt @ $$");
        self.strings = self.list_command("izz");
        self.imports = self.list_command("ii");
        self.exports = self.list_command("iE");
        self.sections = self.list_command("iS");
    }

    fn function_rows(&mut self) -> Vec<crate::visual::FunctionRow> {
        self.session
            .as_mut()
            .and_then(|session| session.command("aflj").ok())
            .map(|text| parse_functions(&text))
            .unwrap_or_default()
    }

    fn session_command(&mut self, command: &str) -> Option<String> {
        self.session
            .as_mut()
            .and_then(|session| session.command(command).ok())
    }

    fn list_command(&mut self, command: &str) -> Vec<String> {
        self.session_command(command)
            .map(|text| text.lines().map(ToOwned::to_owned).collect())
            .unwrap_or_default()
    }

    pub(crate) fn decompile_all(&mut self) {
        self.start_background_job_with_fallback(
            "full decompile",
            "pdd @@f",
            false,
            Some("pdc @@f".to_owned()),
        );
    }

    pub(crate) fn analysis_action(&mut self, action: AnalysisAction) {
        self.start_background_job(action.label(), action.command(), true);
    }

    pub(crate) fn quick_view_action(&mut self, action: QuickViewAction) {
        self.run_command(action.command());
    }

    fn decompile_current(&mut self) {
        self.run_command("pdd");
        if decompiler_crashed(&self.output) {
            self.run_command("pdc");
            self.output = format!(
                "r2dec crashed for this function; showing radare2 pseudo-code fallback.\n\n{}",
                self.output
            );
        }
    }

    pub(crate) fn edit_action(&mut self, action: &EditAction) {
        if let Some(command) = action.command() {
            self.run_command(&command);
            self.refresh_lists();
            self.refresh_inspector();
        }
    }

    pub(crate) fn search_action(&mut self, action: &SearchAction) {
        if let Some(command) = action.command() {
            self.run_command(&command);
            self.search_results = self.output.lines().map(ToOwned::to_owned).collect();
        }
    }

    pub(crate) fn breakpoint_action(&mut self, action: &BreakpointAction) {
        if let Some(command) = action.command() {
            self.run_debug_command(&command);
            self.breakpoint_output = self.command_text("db");
        }
    }

    pub(crate) fn refresh_inspector(&mut self) {
        if self.background_job.is_some() {
            return;
        }
        self.current_offset = self.command_text("s");
        self.register_output = self.command_text("dr");
        self.function_info = self.command_text("afi");
        self.callers_output = self.command_text("axt @ $$");
        self.callees_output = self.command_text("axf @ $$");
        self.stack_output = self.command_text("pxq 128 @ sp");
        self.breakpoint_output = self.command_text("db");
    }

    pub(crate) fn show_current_function_xrefs(&mut self) {
        self.run_command("axt @ $$");
    }

    pub(crate) fn show_current_function_callees(&mut self) {
        self.run_command("axf @ $$");
    }

    pub(crate) fn show_current_function_graph(&mut self) {
        self.run_command("agf");
    }

    pub(crate) fn debug_action(&mut self, action: DebugAction) {
        self.run_debug_command(action.command());
        if matches!(
            action,
            DebugAction::Start
                | DebugAction::Continue
                | DebugAction::StepInto
                | DebugAction::StepOver
                | DebugAction::BreakHere
        ) {
            self.refresh_lists();
            self.refresh_inspector();
        }
    }

    fn command_text(&mut self, command: &str) -> String {
        self.session_command(command).unwrap_or_default()
    }

    fn run_debug_command(&mut self, command: &str) {
        self.run_command(command);
        if let Some(hint) = debug_error_hint(&self.output) {
            self.output = format!("{}\n\n{}", self.output, hint);
            hint.clone_into(&mut self.status);
        }
    }

    pub(crate) fn apply_action(&mut self, action: &VisualAction) {
        if let Some(command) = action.command() {
            self.run_command(&command);
            if command.starts_with('s') || command == "aaa" {
                self.refresh_view();
            }
        }
    }

    pub(crate) fn handle_keys(&mut self, ctx: &egui::Context) {
        // Don't capture navigation keys when a text field has focus
        if ctx.memory(|mem| mem.focused().is_some()) {
            return;
        }
        let (pressed_p, up, down, page_up, page_down) = ctx.input(|input| {
            (
                input.key_pressed(egui::Key::P),
                input.key_pressed(egui::Key::ArrowUp),
                input.key_pressed(egui::Key::ArrowDown),
                input.key_pressed(egui::Key::PageUp),
                input.key_pressed(egui::Key::PageDown),
            )
        });
        if pressed_p {
            self.view = self.view.next();
            self.refresh_view();
        }
        if up {
            self.apply_action(&VisualAction::Step(SeekStep::LineUp));
        }
        if down {
            self.apply_action(&VisualAction::Step(SeekStep::LineDown));
        }
        if page_up {
            self.apply_action(&VisualAction::Step(SeekStep::PageUp));
        }
        if page_down {
            self.apply_action(&VisualAction::Step(SeekStep::PageDown));
        }
    }
}
