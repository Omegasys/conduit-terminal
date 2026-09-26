use super::shell_history::{
    ShellHistory,
    ShellHistoryEntry,
    ShellHistoryFormat,
};

#[derive(Debug, Default)]
pub struct ZshHistory {
    entries: Vec<ShellHistoryEntry>,
}

impl ZshHistory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ShellHistory for ZshHistory {
    fn format(&self) -> ShellHistoryFormat {
        ShellHistoryFormat::Zsh
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

            let command = if line.starts_with(':') {
                line.split_once(';')
                    .map(|(_, command)| command)
                    .unwrap_or(line)
            } else {
                line
            };

            self.entries
                .push(ShellHistoryEntry::new(command));

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
