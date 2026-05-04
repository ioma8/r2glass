use std::path::{Path, PathBuf};

use crate::background::BackgroundJob;
use crate::history::HistoryEntry;
use crate::r2_session::R2Session;
use crate::visual::{FunctionRow, VisualView};
use crate::workspace::{InspectorTab, NavigatorTab};

pub struct R2GlassApp {
    pub(crate) target_input: String,
    pub(crate) target_path: Option<PathBuf>,
    pub(crate) session: Option<R2Session>,
    pub(crate) background_job: Option<BackgroundJob>,
    pub(crate) view: VisualView,
    pub(crate) output: String,
    pub(crate) status: String,
    pub(crate) command_input: String,
    pub(crate) seek_input: String,
    pub(crate) functions: Vec<FunctionRow>,
    pub(crate) flags: Vec<String>,
    pub(crate) xrefs: Vec<String>,
    pub(crate) strings: Vec<String>,
    pub(crate) imports: Vec<String>,
    pub(crate) exports: Vec<String>,
    pub(crate) sections: Vec<String>,
    pub(crate) search_results: Vec<String>,
    pub(crate) history: Vec<HistoryEntry>,
    pub(crate) symbol_filter: String,
    pub(crate) flag_filter: String,
    pub(crate) xref_filter: String,
    pub(crate) data_filter: String,
    pub(crate) rename_input: String,
    pub(crate) comment_input: String,
    pub(crate) flag_input: String,
    pub(crate) patch_hex_input: String,
    pub(crate) text_search_input: String,
    pub(crate) hex_search_input: String,
    pub(crate) breakpoint_input: String,
    pub(crate) register_output: String,
    pub(crate) function_info: String,
    pub(crate) stack_output: String,
    pub(crate) breakpoint_output: String,
    pub(crate) callers_output: String,
    pub(crate) callees_output: String,
    pub(crate) current_offset: String,
    pub(crate) navigator_tab: NavigatorTab,
    pub(crate) inspector_tab: InspectorTab,
    pub(crate) output_wrap: bool,
    pub(crate) output_line_numbers: bool,
}

impl R2GlassApp {
    #[must_use]
    pub fn new(target: Option<PathBuf>) -> Self {
        let target_input = target
            .as_ref()
            .map_or_else(String::new, |path| path.display().to_string());
        let mut app = Self {
            target_input,
            target_path: None,
            session: None,
            background_job: None,
            view: VisualView::Disassembly,
            output: String::new(),
            status: "No target loaded".to_owned(),
            command_input: String::new(),
            seek_input: String::new(),
            functions: Vec::new(),
            flags: Vec::new(),
            xrefs: Vec::new(),
            strings: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            sections: Vec::new(),
            search_results: Vec::new(),
            history: Vec::new(),
            symbol_filter: String::new(),
            flag_filter: String::new(),
            xref_filter: String::new(),
            data_filter: String::new(),
            rename_input: String::new(),
            comment_input: String::new(),
            flag_input: String::new(),
            patch_hex_input: String::new(),
            text_search_input: String::new(),
            hex_search_input: String::new(),
            breakpoint_input: String::new(),
            register_output: String::new(),
            function_info: String::new(),
            stack_output: String::new(),
            breakpoint_output: String::new(),
            callers_output: String::new(),
            callees_output: String::new(),
            current_offset: String::new(),
            navigator_tab: NavigatorTab::Symbols,
            inspector_tab: InspectorTab::Context,
            output_wrap: false,
            output_line_numbers: true,
        };
        if let Some(path) = target {
            app.open_target(&path);
        }
        app
    }

    pub(crate) fn open_target(&mut self, target: &Path) {
        match R2Session::open(target) {
            Ok(session) => {
                self.session = Some(session);
                self.target_path = Some(target.to_path_buf());
                self.status = format!("Loaded {}", target.display());
                self.start_background_job("initial analysis", "aaa", true);
            }
            Err(err) => {
                self.status = err.to_string();
                self.session = None;
                self.target_path = None;
            }
        }
    }

    pub(crate) fn run_command(&mut self, command: &str) {
        if command.trim().is_empty() {
            return;
        }
        if self.background_job.is_some() {
            "Background command running".clone_into(&mut self.status);
            return;
        }
        let Some(session) = self.session.as_mut() else {
            "No r2 session".clone_into(&mut self.status);
            return;
        };
        match session.command(command) {
            Ok(output) => {
                self.output = output;
                self.history.push(HistoryEntry::command(command));
                self.status = format!(":{command}");
            }
            Err(err) => self.status = err.to_string(),
        }
    }
}
