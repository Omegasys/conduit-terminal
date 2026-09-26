use super::shell_history::{
    ShellHistory,
    ShellHistoryEntry,
    ShellHistoryFormat,
};

#[derive(Debug, Default)]
pub struct PowerShellHistory {
    entries: Vec<ShellHistoryEntry>,
}

impl PowerShellHistory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ShellHistory for PowerShellHistory {
    fn format(&self) -> ShellHistoryFormat {
        ShellHistoryFormat::PowerShell
    }

    fn entries(&self) -> &[ShellHistoryEntry] {
        &self.entries
    }

    fn import_text(
        &mut self,
        text: &str,
    ) -> Result<usize, String> {
        let mut count = 0;

        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }

            self.entries
                .push(ShellHistoryEntry::new(line));

            count += 1;
        }

        Ok(count)
    }

    fn export_text(&self) -> String {
        self.entries
            .iter()
            .map(|entry| entry.command())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}
