use super::shell_history::{
    ShellHistory,
    ShellHistoryEntry,
    ShellHistoryFormat,
};

#[derive(Debug, Default)]
pub struct BashHistory {
    entries: Vec<ShellHistoryEntry>,
}

impl BashHistory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ShellHistory for BashHistory {
    fn format(&self) -> ShellHistoryFormat {
        ShellHistoryFormat::Bash
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
            if line.is_empty() {
                continue;
            }

            if line.starts_with('#') {
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
