#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HistoryEntry {
    label: String,
    command: Option<String>,
}

impl HistoryEntry {
    pub(crate) fn command(command: impl Into<String>) -> Self {
        let command = command.into();
        Self {
            label: command.clone(),
            command: Some(command),
        }
    }

    #[cfg(test)]
    pub(crate) fn note(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            command: None,
        }
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn replay_command(&self) -> Option<&str> {
        self.command.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::HistoryEntry;

    #[test]
    fn command_entries_are_replayable() {
        let entry = HistoryEntry::command("pdc @@f");

        assert_eq!(entry.label(), "pdc @@f");
        assert_eq!(entry.replay_command(), Some("pdc @@f"));
    }

    #[test]
    fn note_entries_are_not_replayable() {
        let entry = HistoryEntry::note("r2dec crashed");

        assert_eq!(entry.label(), "r2dec crashed");
        assert_eq!(entry.replay_command(), None);
    }
}
