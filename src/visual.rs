pub use crate::actions::{
    AnalysisAction, BreakpointAction, DebugAction, EditAction, QuickViewAction, SearchAction,
    SeekStep, VisualAction,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisualView {
    Disassembly,
    Hex,
    Graph,
    Decompile,
    Info,
}

impl VisualView {
    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::Disassembly => Self::Hex,
            Self::Hex => Self::Graph,
            Self::Graph => Self::Decompile,
            Self::Decompile => Self::Info,
            Self::Info => Self::Disassembly,
        }
    }

    #[must_use]
    pub const fn render_command(self) -> &'static str {
        match self {
            Self::Disassembly => "pd $r",
            Self::Hex => "px 512",
            Self::Graph => "agf",
            Self::Decompile => "pdd",
            Self::Info => "iI;dr;afl~.",
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Disassembly => "Disasm",
            Self::Hex => "Hex",
            Self::Graph => "Graph",
            Self::Decompile => "Decompile",
            Self::Info => "Info",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionRow {
    pub address: String,
    pub name: String,
}

impl FunctionRow {
    #[must_use]
    pub const fn new(address: String, name: String) -> Self {
        Self { address, name }
    }

    #[must_use]
    pub fn label(&self) -> String {
        self.name.clone()
    }
}

#[must_use]
pub fn debug_error_hint(output: &str) -> Option<&'static str> {
    let lower = output.to_ascii_lowercase();
    if lower.contains("possibly unsigned r2") || lower.contains("ptrace: cannot attach") {
        Some(
            "macOS blocked r2 debugger attach. radare2 needs a trusted codesign setup for debugging: run `sudo DevToolsSecurity -enable`, install/sign r2 using radare2's macOS signing steps, then restart r2glass. System binaries outside your home may also require SIP changes.",
        )
    } else {
        None
    }
}

#[must_use]
pub fn decompiler_crashed(output: &str) -> bool {
    let lower = output.to_ascii_lowercase();
    lower.contains("r2dec has crashed") || lower.contains("r2dec-js/issues")
}

#[must_use]
pub fn clickable_seek_target(row: &str) -> Option<String> {
    row.split_whitespace()
        .filter_map(|part| part.find("0x").map(|index| &part[index..]))
        .filter_map(trim_address)
        .find(|part| part.len() > 2)
        .map(ToOwned::to_owned)
}

fn trim_address(value: &str) -> Option<&str> {
    let trimmed = value.trim_end_matches(|ch: char| !ch.is_ascii_hexdigit());
    if trimmed.is_empty() { None } else { Some(trimmed) }
}

#[must_use]
pub fn parse_functions(json_text: &str) -> Vec<FunctionRow> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json_text) else {
        return Vec::new();
    };
    value
        .as_array()
        .map(|items| items.iter().filter_map(parse_function).collect())
        .unwrap_or_default()
}

fn parse_function(value: &serde_json::Value) -> Option<FunctionRow> {
    let offset = value
        .get("addr")
        .or_else(|| value.get("offset"))?
        .as_u64()?;
    let name = value.get("name")?.as_str()?.to_owned();
    Some(FunctionRow::new(format!("0x{offset:x}"), name))
}
