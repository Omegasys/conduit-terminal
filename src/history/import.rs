use super::database::HistoryDatabase;
use super::metadata::HistoryMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryImportFormat {
    PlainText,
    JsonLines,
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

#[derive(Debug, Default)]
pub struct HistoryImporter;

impl HistoryImporter {
    pub fn new() -> Self {
        Self
    }

    pub fn import(
        &self,
        database: &mut HistoryDatabase,
        input: &str,
        format: HistoryImportFormat,
    ) -> Result<usize, String> {
        match format {
            HistoryImportFormat::PlainText => {
                Ok(self.import_plain_text(database, input))
            }

            HistoryImportFormat::Bash => {
                Ok(self.import_plain_text(database, input))
            }

            HistoryImportFormat::Zsh => {
                Ok(self.import_plain_text(database, input))
            }

            HistoryImportFormat::Fish => {
                self.import_fish(database, input)
            }

            HistoryImportFormat::PowerShell => {
                Ok(self.import_plain_text(database, input))
            }

            HistoryImportFormat::JsonLines => {
                Err("JSON Lines import requires a JSON backend".into())
            }
        }
    }

    fn import_plain_text(
        &self,
        database: &mut HistoryDatabase,
        input: &str,
    ) -> usize {
        let mut count = 0;

        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }

            database.insert(line, HistoryMetadata::new());
            count += 1;
        }

        count
    }

    fn import_fish(
        &self,
        database: &mut HistoryDatabase,
        input: &str,
    ) -> Result<usize, String> {
        let mut count = 0;

        for line in input.lines() {
            let line = line.trim();

            if line.starts_with("- cmd:") {
                let command = line
                    .trim_start_matches("- cmd:")
                    .trim();

                database.insert(command, HistoryMetadata::new());
                count += 1;
            }
        }

        Ok(count)
    }
}
