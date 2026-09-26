use super::database::HistoryDatabase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryExportFormat {
    PlainText,
    JsonLines,
    Csv,
}

#[derive(Debug, Default)]
pub struct HistoryExporter;

impl HistoryExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export(
        &self,
        database: &HistoryDatabase,
        format: HistoryExportFormat,
    ) -> String {
        match format {
            HistoryExportFormat::PlainText => {
                database
                    .entries()
                    .iter()
                    .map(|entry| entry.command())
                    .collect::<Vec<_>>()
                    .join("\n")
            }

            HistoryExportFormat::Csv => {
                let mut output =
                    String::from("id,command,shell,exit_status\n");

                for entry in database.entries() {
                    let command =
                        csv_escape(entry.command());

                    let shell = entry
                        .metadata()
                        .shell()
                        .map(csv_escape)
                        .unwrap_or_default();

                    let status = entry
                        .metadata()
                        .exit_status()
                        .map(|value| value.to_string())
                        .unwrap_or_default();

                    output.push_str(&format!(
                        "{},{},{},{}\n",
                        entry.id(),
                        command,
                        shell,
                        status
                    ));
                }

                output
            }

            HistoryExportFormat::JsonLines => {
                let mut output = String::new();

                for entry in database.entries() {
                    let command =
                        json_escape(entry.command());

                    let shell = entry
                        .metadata()
                        .shell()
                        .map(json_escape)
                        .unwrap_or_else(|| "null".to_string());

                    let status = entry
                        .metadata()
                        .exit_status()
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "null".to_string());

                    output.push_str(&format!(
                        "{{\"id\":{},\"command\":\"{}\",\"shell\":{},\"exit_status\":{}}}\n",
                        entry.id(),
                        command,
                        shell,
                        status
                    ));
                }

                output
            }
        }
    }
}

fn csv_escape(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
