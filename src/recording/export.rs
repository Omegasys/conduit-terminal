use std::fmt::Write;

use super::format::{
    RecordingEventKind,
    RecordingFile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingExportFormat {
    PlainText,
    JsonLines,
    Asciinema,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RecordingExporter;

impl RecordingExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export(
        &self,
        recording: &RecordingFile,
        format: RecordingExportFormat,
    ) -> String {
        match format {
            RecordingExportFormat::PlainText => {
                self.plain_text(recording)
            }
            RecordingExportFormat::JsonLines => {
                self.json_lines(recording)
            }
            RecordingExportFormat::Asciinema => {
                self.asciinema(recording)
            }
        }
    }

    fn plain_text(&self, recording: &RecordingFile) -> String {
        let mut output = String::new();

        for event in recording.events() {
            if let RecordingEventKind::Output = event.kind() {
                output.push_str(&String::from_utf8_lossy(event.data()));
            }
        }

        output
    }

    fn json_lines(&self, recording: &RecordingFile) -> String {
        let mut output = String::new();

        for event in recording.events() {
            let timestamp = event.timestamp().as_secs_f64();

            match event.kind() {
                RecordingEventKind::Output => {
                    let data = escape_json(
                        &String::from_utf8_lossy(event.data()),
                    );

                    let _ = writeln!(
                        output,
                        "{{\"time\":{timestamp},\"type\":\"output\",\"data\":\"{data}\"}}"
                    );
                }

                RecordingEventKind::Input => {
                    let data = escape_json(
                        &String::from_utf8_lossy(event.data()),
                    );

                    let _ = writeln!(
                        output,
                        "{{\"time\":{timestamp},\"type\":\"input\",\"data\":\"{data}\"}}"
                    );
                }

                RecordingEventKind::Bell => {
                    let _ = writeln!(
                        output,
                        "{{\"time\":{timestamp},\"type\":\"bell\"}}"
                    );
                }

                RecordingEventKind::Resize {
                    columns,
                    rows,
                } => {
                    let _ = writeln!(
                        output,
                        "{{\"time\":{timestamp},\"type\":\"resize\",\"columns\":{columns},\"rows\":{rows}}}"
                    );
                }

                RecordingEventKind::TitleChanged(title) => {
                    let title = escape_json(title);

                    let _ = writeln!(
                        output,
                        "{{\"time\":{timestamp},\"type\":\"title\",\"data\":\"{title}\"}}"
                    );
                }

                RecordingEventKind::WorkingDirectoryChanged(path) => {
                    let path = escape_json(path);

                    let _ = writeln!(
                        output,
                        "{{\"time\":{timestamp},\"type\":\"cwd\",\"data\":\"{path}\"}}"
                    );
                }

                RecordingEventKind::Marker(marker) => {
                    let marker = escape_json(marker);

                    let _ = writeln!(
                        output,
                        "{{\"time\":{timestamp},\"type\":\"marker\",\"data\":\"{marker}\"}}"
                    );
                }
            }
        }

        output
    }

    fn asciinema(&self, recording: &RecordingFile) -> String {
        let metadata = recording.metadata();

        let mut output = String::new();

        let _ = writeln!(
            output,
            "{{\"version\":2,\"width\":{},\"height\":{},\"timestamp\":{}}}",
            metadata.columns,
            metadata.rows,
            metadata.created_at.unwrap_or_default()
        );

        for event in recording.events() {
            if let RecordingEventKind::Output = event.kind() {
                let text = escape_json(
                    &String::from_utf8_lossy(event.data()),
                );

                let _ = writeln!(
                    output,
                    "[{},\"o\",\"{}\"]",
                    event.timestamp().as_secs_f64(),
                    text
                );
            }
        }

        output
    }
}

fn escape_json(value: &str) -> String {
    let mut output = String::with_capacity(value.len());

    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0c}' => output.push_str("\\f"),
            character if character.is_control() => {
                let _ = write!(
                    output,
                    "\\u{:04x}",
                    character as u32
                );
            }
            character => output.push(character),
        }
    }

    output
}
