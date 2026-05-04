use r2glass::visual::{
    AnalysisAction, BreakpointAction, DebugAction, EditAction, FunctionRow, QuickViewAction,
    SearchAction, SeekStep, VisualAction, VisualView, clickable_seek_target, debug_error_hint,
    decompiler_crashed, parse_functions,
};

// ---------------------------------------------------------------------------
// VisualView cycling
// ---------------------------------------------------------------------------

#[test]
fn cycles_visual_views_like_r2_print_modes() {
    assert_eq!(VisualView::Disassembly.next(), VisualView::Hex);
    assert_eq!(VisualView::Hex.next(), VisualView::Graph);
    assert_eq!(VisualView::Graph.next(), VisualView::Decompile);
    assert_eq!(VisualView::Decompile.next(), VisualView::Info);
    assert_eq!(VisualView::Info.next(), VisualView::Disassembly);
}

#[test]
fn maps_mouse_and_key_actions_to_r2_commands() {
    assert_eq!(VisualAction::Analyze.command(), Some("aaa".to_owned()));
    assert_eq!(VisualAction::Refresh.command(), Some("pd $r".to_owned()));
    assert_eq!(
        VisualAction::Seek("main".to_owned()).command(),
        Some("s main".to_owned())
    );
    assert_eq!(
        VisualAction::Step(SeekStep::PageDown).command(),
        Some("s +256".to_owned())
    );
}

#[test]
fn maps_analysis_actions_to_r2_commands() {
    assert_eq!(AnalysisAction::Basic.command(), "aa");
    assert_eq!(AnalysisAction::Full.command(), "aaa");
    assert_eq!(AnalysisAction::Deep.command(), "aaaa");
    assert_eq!(AnalysisAction::Types.command(), "aaft");
    assert_eq!(AnalysisAction::Full.label(), "full analysis");
}

#[test]
fn maps_quick_view_actions_to_r2_commands() {
    assert_eq!(QuickViewAction::Headers.command(), "ih");
    assert_eq!(QuickViewAction::FileInfo.command(), "iI");
    assert_eq!(QuickViewAction::Symbols.command(), "is");
    assert_eq!(QuickViewAction::Entrypoints.command(), "ie");
}

#[test]
fn maps_debug_actions_to_r2_commands() {
    assert_eq!(DebugAction::Start.command(), "ood");
    assert_eq!(DebugAction::Continue.command(), "dc");
    assert_eq!(DebugAction::StepInto.command(), "ds");
    assert_eq!(DebugAction::StepOver.command(), "dso");
    assert_eq!(DebugAction::BreakHere.command(), "db $$");
    assert_eq!(DebugAction::Registers.command(), "dr");
}

#[test]
fn maps_breakpoint_actions_to_r2_commands() {
    assert_eq!(
        BreakpointAction::Add("main".to_owned()).command(),
        Some("db main".to_owned())
    );
    assert_eq!(
        BreakpointAction::Remove("0x1000".to_owned()).command(),
        Some("db- 0x1000".to_owned())
    );
    assert_eq!(
        BreakpointAction::ClearAll.command(),
        Some("db-*".to_owned())
    );
}

#[test]
fn maps_edit_actions_to_r2_commands() {
    assert_eq!(
        EditAction::RenameFunction("sym.good".to_owned()).command(),
        Some("afn sym.good".to_owned())
    );
    assert_eq!(
        EditAction::SetComment("checked branch".to_owned()).command(),
        Some("CC checked branch".to_owned())
    );
    assert_eq!(
        EditAction::SetFlag("interesting".to_owned()).command(),
        Some("f interesting".to_owned())
    );
    assert_eq!(
        EditAction::PatchHex("9090".to_owned()).command(),
        Some("wx 9090".to_owned())
    );
}

#[test]
fn maps_search_actions_to_r2_commands() {
    assert_eq!(
        SearchAction::Text("password".to_owned()).command(),
        Some("/ password".to_owned())
    );
    assert_eq!(
        SearchAction::Hex("414243".to_owned()).command(),
        Some("/x 414243".to_owned())
    );
    assert_eq!(SearchAction::Next.command(), Some("/".to_owned()));
}

#[test]
fn explains_macos_unsigned_debugger_errors() {
    let output = "ptrace: Cannot attach: Invalid argument\nINFO: Possibly unsigned r2.";
    let hint = debug_error_hint(output);

    assert!(hint.is_some_and(|text| text.contains("codesign")));
}

#[test]
fn detects_r2dec_crash_output() {
    assert!(decompiler_crashed("r2dec has crashed at 0x2ee"));
    assert!(decompiler_crashed(
        "report at https://github.com/wargio/r2dec-js/issues"
    ));
    assert!(!decompiler_crashed("int main(void) { return 0; }"));
}

#[test]
fn extracts_clickable_seek_targets_from_r2_rows() {
    assert_eq!(
        clickable_seek_target("0x100003f10 42 sym.main"),
        Some("0x100003f10".to_owned())
    );
    assert_eq!(
        clickable_seek_target("sym.main 0x100003f10"),
        Some("0x100003f10".to_owned())
    );
    assert_eq!(
        clickable_seek_target("vaddr=0x100003f10 paddr=0x3f10 string"),
        Some("0x100003f10".to_owned())
    );
    assert_eq!(clickable_seek_target("no address here"), None);
}

#[test]
fn parses_all_function_rows_from_r2_json() {
    let rows =
        parse_functions(r#"[{"addr":4096,"name":"sym.main"},{"offset":8192,"name":"entry0"}]"#);

    assert_eq!(
        rows,
        vec![
            FunctionRow::new("0x1000".to_owned(), "sym.main".to_owned()),
            FunctionRow::new("0x2000".to_owned(), "entry0".to_owned()),
        ]
    );
    assert_eq!(rows[0].label(), "sym.main");
}

// ---------------------------------------------------------------------------
// Missing enum variants
// ---------------------------------------------------------------------------

#[test]
fn maps_missing_quick_view_variants() {
    assert_eq!(QuickViewAction::Relocations.command(), "ir");
    assert_eq!(QuickViewAction::Comments.command(), "CC");
}

#[test]
fn maps_missing_debug_variant() {
    assert_eq!(DebugAction::Backtrace.command(), "dbt");
}

// ---------------------------------------------------------------------------
// Action command edge cases — empty & whitespace-only input
// ---------------------------------------------------------------------------

#[test]
fn rejects_empty_seek() {
    assert_eq!(VisualAction::Seek(String::new()).command(), None);
}

#[test]
fn rejects_whitespace_seek() {
    assert_eq!(VisualAction::Seek("   ".to_owned()).command(), None);
}

#[test]
fn rejects_all_empty_edit_actions() {
    assert_eq!(EditAction::RenameFunction(String::new()).command(), None);
    assert_eq!(EditAction::SetComment(String::new()).command(), None);
    assert_eq!(EditAction::SetFlag(String::new()).command(), None);
    assert_eq!(EditAction::PatchHex(String::new()).command(), None);
}

#[test]
fn rejects_whitespace_edit_actions() {
    assert_eq!(EditAction::RenameFunction("  ".to_owned()).command(), None);
    assert_eq!(EditAction::SetComment("  ".to_owned()).command(), None);
    assert_eq!(EditAction::SetFlag("  ".to_owned()).command(), None);
    assert_eq!(EditAction::PatchHex("  ".to_owned()).command(), None);
}

#[test]
fn rejects_empty_search_and_breakpoint() {
    assert_eq!(SearchAction::Text(String::new()).command(), None);
    assert_eq!(SearchAction::Hex(String::new()).command(), None);
    assert_eq!(BreakpointAction::Add(String::new()).command(), None);
    assert_eq!(BreakpointAction::Remove(String::new()).command(), None);
}

// ---------------------------------------------------------------------------
// Command injection prevention — metacharacter rejection
// ---------------------------------------------------------------------------

#[test]
fn rejects_semicolons_in_seek() {
    assert_eq!(
        VisualAction::Seek("main;!cat /etc/passwd".to_owned()).command(),
        None
    );
}

#[test]
fn rejects_shell_escape_in_edit_actions() {
    assert_eq!(
        EditAction::RenameFunction("fn;!rm -rf /".to_owned()).command(),
        None
    );
    assert_eq!(
        EditAction::SetComment("ok;!id".to_owned()).command(),
        None
    );
    assert_eq!(
        EditAction::SetFlag("x;!echo pwned".to_owned()).command(),
        None
    );
}

#[test]
fn rejects_non_hex_chars_in_patch() {
    assert_eq!(EditAction::PatchHex("hello".to_owned()).command(), None);
    assert_eq!(EditAction::PatchHex(";!rm".to_owned()).command(), None);
    assert_eq!(EditAction::PatchHex("GGXX".to_owned()).command(), None);
}

#[test]
fn allows_hex_whitespace_in_patch() {
    assert_eq!(
        EditAction::PatchHex("90 90".to_owned()).command(),
        Some("wx 90 90".to_owned())
    );
    assert_eq!(
        EditAction::PatchHex("deadbeef".to_owned()).command(),
        Some("wx deadbeef".to_owned())
    );
}

#[test]
fn rejects_injected_search_queries() {
    assert_eq!(
        SearchAction::Text("password;!ls".to_owned()).command(),
        None
    );
    assert_eq!(
        SearchAction::Hex("4142;!rm".to_owned()).command(),
        None
    );
}

#[test]
fn rejects_semicolons_in_breakpoint_targets() {
    assert_eq!(
        BreakpointAction::Add("main;!ls".to_owned()).command(),
        None
    );
    assert_eq!(
        BreakpointAction::Remove("0x1000;!id".to_owned()).command(),
        None
    );
}

// ---------------------------------------------------------------------------
// clickable_seek_target edge cases
// ---------------------------------------------------------------------------

#[test]
fn extracts_first_address_when_multiple_present() {
    assert_eq!(
        clickable_seek_target("0x1000 0x2000 0x3000"),
        Some("0x1000".to_owned())
    );
}

#[test]
fn rejects_non_hex_looking_addresses() {
    assert_eq!(clickable_seek_target("0xGG"), None);
    assert_eq!(clickable_seek_target("0x"), None);
}

#[test]
fn handles_empty_and_garbage_input() {
    assert_eq!(clickable_seek_target(""), None);
    assert_eq!(clickable_seek_target("   "), None);
    assert_eq!(clickable_seek_target("!@#$%^"), None);
}

#[test]
fn extracts_address_from_trailing_position() {
    assert_eq!(
        clickable_seek_target("label1            0x100003f10"),
        Some("0x100003f10".to_owned())
    );
}

#[test]
fn extracts_address_with_trailing_punctuation() {
    assert_eq!(
        clickable_seek_target("0x100003f10,"),
        Some("0x100003f10".to_owned())
    );
    assert_eq!(
        clickable_seek_target("(0x100003f10)"),
        Some("0x100003f10".to_owned())
    );
    assert_eq!(
        clickable_seek_target("0x100003f10;"),
        Some("0x100003f10".to_owned())
    );
}

// ---------------------------------------------------------------------------
// parse_functions edge cases
// ---------------------------------------------------------------------------

#[test]
fn parses_empty_array() {
    assert!(parse_functions("[]").is_empty());
}

#[test]
fn returns_empty_for_malformed_json() {
    assert!(parse_functions("").is_empty());
    assert!(parse_functions("{broken json").is_empty());
    assert!(parse_functions("not json at all").is_empty());
    assert!(parse_functions("{}").is_empty());
}

#[test]
fn skips_entries_missing_address() {
    let rows = parse_functions(r#"[{"name":"orphan"}]"#);
    assert!(rows.is_empty());
}

#[test]
fn skips_entries_missing_name() {
    let rows = parse_functions(r#"[{"addr":4096}]"#);
    assert!(rows.is_empty());
}

#[test]
fn handles_mixed_valid_and_invalid_entries() {
    let rows = parse_functions(
        r#"[{"addr":4096,"name":"good"},{"name":"orphan"},{"offset":8192,"name":"also_good"}]"#,
    );
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].label(), "good");
    assert_eq!(rows[1].label(), "also_good");
}

// ---------------------------------------------------------------------------
// debug_error_hint edge cases
// ---------------------------------------------------------------------------

#[test]
fn no_hint_for_unrelated_output() {
    assert!(debug_error_hint("int main(void) { return 0; }").is_none());
    assert!(debug_error_hint("").is_none());
}

// ---------------------------------------------------------------------------
// decompiler_crashed edge cases
// ---------------------------------------------------------------------------

#[test]
fn no_false_positive_for_benign_output() {
    assert!(!decompiler_crashed(""));
    assert!(!decompiler_crashed("r2dec loaded successfully"));
}

// ---------------------------------------------------------------------------
// SeekStep variants map correctly
// ---------------------------------------------------------------------------

#[test]
fn maps_all_seek_steps() {
    // SeekStep::command() is not pub, but VisualAction::Step tests cover it
    assert_eq!(
        VisualAction::Step(SeekStep::LineUp).command(),
        Some("s -16".to_owned())
    );
    assert_eq!(
        VisualAction::Step(SeekStep::LineDown).command(),
        Some("s +16".to_owned())
    );
    assert_eq!(
        VisualAction::Step(SeekStep::PageUp).command(),
        Some("s -256".to_owned())
    );
    assert_eq!(
        VisualAction::Step(SeekStep::PageDown).command(),
        Some("s +256".to_owned())
    );
}
