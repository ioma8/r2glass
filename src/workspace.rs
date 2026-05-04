#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NavigatorTab {
    Symbols,
    Data,
    References,
}

impl NavigatorTab {
    pub(crate) const ALL: [Self; 3] = [Self::Symbols, Self::Data, Self::References];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Symbols => "Symbols",
            Self::Data => "Data",
            Self::References => "Refs",
        }
    }

    pub(crate) const fn hover_text(self) -> &'static str {
        match self {
            Self::Symbols => "Browse functions and flags",
            Self::Data => "Browse strings, imports, exports, sections",
            Self::References => "Browse cross-references",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InspectorTab {
    Context,
    Debug,
    Tools,
}

impl InspectorTab {
    pub(crate) const ALL: [Self; 3] = [Self::Context, Self::Debug, Self::Tools];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Context => "Context",
            Self::Debug => "Debug",
            Self::Tools => "Tools",
        }
    }

    pub(crate) const fn hover_text(self) -> &'static str {
        match self {
            Self::Context => "View function context, callers, callees",
            Self::Debug => "View registers, stack, breakpoints",
            Self::Tools => "Search, edit, and set breakpoints",
        }
    }
}
