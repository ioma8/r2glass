use crate::visual::VisualView;

const MAX_INPUT_LENGTH: usize = 4096;

/// Characters that have special meaning in radare2 command syntax and can be
/// used for command injection: command separator (`;`), shell escape (`!`),
/// and shell pipe (`|`).
const R2_METACHARS: [char; 4] = [';', '!', '|', '`'];

fn validate_input(input: &str) -> Option<&str> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_INPUT_LENGTH {
        return None;
    }
    if trimmed.contains(&R2_METACHARS[..]) {
        return None;
    }
    Some(trimmed)
}

fn validate_hex(input: &str) -> Option<&str> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.len() > MAX_INPUT_LENGTH {
        return None;
    }
    // Only hex digits and whitespace are valid in hex byte input
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_hexdigit() || c.is_ascii_whitespace())
    {
        return None;
    }
    Some(trimmed)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalysisAction {
    Basic,
    Full,
    Deep,
    Types,
}

impl AnalysisAction {
    #[must_use]
    pub const fn command(self) -> &'static str {
        match self {
            Self::Basic => "aa",
            Self::Full => "aaa",
            Self::Deep => "aaaa",
            Self::Types => "aaft",
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Basic => "basic analysis",
            Self::Full => "full analysis",
            Self::Deep => "deep analysis",
            Self::Types => "type analysis",
        }
    }

    #[must_use]
    pub const fn hover_text(self) -> &'static str {
        match self {
            Self::Basic => "Run basic analysis (aa)",
            Self::Full => "Run full analysis (aaa)",
            Self::Deep => "Run deep analysis (aaaa)",
            Self::Types => "Recover type information (aaft)",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuickViewAction {
    Headers,
    FileInfo,
    Symbols,
    Relocations,
    Entrypoints,
    Comments,
}

impl QuickViewAction {
    #[must_use]
    pub const fn command(self) -> &'static str {
        match self {
            Self::Headers => "ih",
            Self::FileInfo => "iI",
            Self::Symbols => "is",
            Self::Relocations => "ir",
            Self::Entrypoints => "ie",
            Self::Comments => "CC",
        }
    }

    #[must_use]
    pub const fn hover_text(self) -> &'static str {
        match self {
            Self::Headers => "Show binary headers (ih)",
            Self::FileInfo => "Show file info (iI)",
            Self::Symbols => "List symbols (is)",
            Self::Relocations => "List relocations (ir)",
            Self::Entrypoints => "List entrypoints (ie)",
            Self::Comments => "List comments (CC)",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DebugAction {
    Start,
    Continue,
    StepInto,
    StepOver,
    BreakHere,
    Registers,
    Backtrace,
}

impl DebugAction {
    #[must_use]
    pub const fn command(self) -> &'static str {
        match self {
            Self::Start => "ood",
            Self::Continue => "dc",
            Self::StepInto => "ds",
            Self::StepOver => "dso",
            Self::BreakHere => "db $$",
            Self::Registers => "dr",
            Self::Backtrace => "dbt",
        }
    }

    #[must_use]
    pub const fn hover_text(self) -> &'static str {
        match self {
            Self::Start => "Start debug session (ood)",
            Self::Continue => "Continue execution (dc)",
            Self::StepInto => "Step into instruction (ds)",
            Self::StepOver => "Step over instruction (dso)",
            Self::BreakHere => "Set breakpoint here (db $$)",
            Self::Registers => "Show register state (dr)",
            Self::Backtrace => "Show backtrace (dbt)",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeekStep {
    LineUp,
    LineDown,
    PageUp,
    PageDown,
}

impl SeekStep {
    const fn command(self) -> &'static str {
        match self {
            Self::LineUp => "s -16",
            Self::LineDown => "s +16",
            Self::PageUp => "s -256",
            Self::PageDown => "s +256",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VisualAction {
    Analyze,
    Refresh,
    Seek(String),
    Step(SeekStep),
}

impl VisualAction {
    #[must_use]
    pub fn command(&self) -> Option<String> {
        match self {
            Self::Analyze => Some("aaa".to_owned()),
            Self::Refresh => Some(VisualView::Disassembly.render_command().to_owned()),
            Self::Seek(target) => validate_input(target).map(|t| format!("s {t}")),
            Self::Step(step) => Some(step.command().to_owned()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EditAction {
    RenameFunction(String),
    SetComment(String),
    SetFlag(String),
    PatchHex(String),
}

impl EditAction {
    #[must_use]
    pub fn command(&self) -> Option<String> {
        match self {
            Self::RenameFunction(name) => validate_input(name).map(|n| format!("afn {n}")),
            Self::SetComment(comment) => validate_input(comment).map(|c| format!("CC {c}")),
            Self::SetFlag(name) => validate_input(name).map(|n| format!("f {n}")),
            Self::PatchHex(bytes) => validate_hex(bytes).map(|b| format!("wx {b}")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchAction {
    Text(String),
    Hex(String),
    Next,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BreakpointAction {
    Add(String),
    Remove(String),
    ClearAll,
}

impl BreakpointAction {
    #[must_use]
    pub fn command(&self) -> Option<String> {
        match self {
            Self::Add(target) => validate_input(target).map(|t| format!("db {t}")),
            Self::Remove(target) => validate_input(target).map(|t| format!("db- {t}")),
            Self::ClearAll => Some("db-*".to_owned()),
        }
    }
}

impl SearchAction {
    #[must_use]
    pub fn command(&self) -> Option<String> {
        match self {
            Self::Text(query) => validate_input(query).map(|q| format!("/ {q}")),
            Self::Hex(bytes) => validate_hex(bytes).map(|b| format!("/x {b}")),
            Self::Next => Some("/".to_owned()),
        }
    }
}
