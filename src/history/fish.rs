use super::shell_history::{
    ShellHistory,
    ShellHistoryEntry,
    ShellHistoryFormat,
};

#[derive(Debug, Default)]
pub struct FishHistory {
    entries: Vec<ShellHistoryEntry>,
}

impl FishHistory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ShellHistory for FishHistory {
    fn format(&self) -> ShellHistoryFormat {
        ShellHistoryFormat::Fish
    }

    fn entries(&self) -> &[ShellHistoryEntry] {
        &self.entries
    }

    fn import_text(
        &mut self,
        text: &str,
    ) -> Result<usize, String> {
        let mut count = 0;
        let mut command: Option<String> = None;

        for line in text.lines() {
            let trimmed = line.trim();

            if let Some(value) = trimmed.strip_prefix("- cmd:") {
                if let Some(previous) = command.take() {
                    self.entries
                        .push(ShellHistoryEntry::new(previous));
                    count += 1;
                }

                command = Some(value.trim().to_string());
            } else if let Some(value) =
                trimmed.strip_prefix("  cmd:")
            {
                command = Some(value.trim().to_string());
            }
        }

        if let Some(command) = command {
            self.entries
                .push(ShellHistoryEntry::new(command));
            count += 1;
        }

        Ok(count)
    }

    fn export_text(&self) -> String {
        let mut output = String::new();

        for entry in &self.entries {
            output.push_str("- cmd: ");
            output.push_str(entry.command());
            output.push('\n');
        }

        output
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}
