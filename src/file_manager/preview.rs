use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilePreviewKind {
    Text,
    Binary,
    Directory,
    Empty,
    Error,
}

#[derive(Debug, Clone)]
pub struct FilePreview {
    kind: FilePreviewKind,
    content: String,
    truncated: bool,
}

impl FilePreview {
    pub fn new(
        kind: FilePreviewKind,
        content: impl Into<String>,
        truncated: bool,
    ) -> Self {
        Self {
            kind,
            content: content.into(),
            truncated,
        }
    }

    pub fn kind(&self) -> FilePreviewKind {
        self.kind
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FilePreviewer {
    max_bytes: usize,
    max_lines: usize,
}

impl Default for FilePreviewer {
    fn default() -> Self {
        Self {
            max_bytes: 64 * 1024,
            max_lines: 1000,
        }
    }
}

impl FilePreviewer {
    pub fn new(max_bytes: usize, max_lines: usize) -> Self {
        Self {
            max_bytes,
            max_lines,
        }
    }

    pub fn preview(&self, path: &Path) -> io::Result<FilePreview> {
        let metadata = fs::symlink_metadata(path)?;

        if metadata.is_dir() {
            return Ok(FilePreview::new(
                FilePreviewKind::Directory,
                "[directory]",
                false,
            ));
        }

        if metadata.len() == 0 {
            return Ok(FilePreview::new(
                FilePreviewKind::Empty,
                "",
                false,
            ));
        }

        let bytes = fs::read(path)?;
        let truncated_bytes = bytes.len() > self.max_bytes;

        let bytes = if truncated_bytes {
            &bytes[..self.max_bytes]
        } else {
            &bytes[..]
        };

        match std::str::from_utf8(bytes) {
            Ok(text) => {
                let mut lines = Vec::new();
                let mut truncated_lines = false;

                for line in text.lines() {
                    if lines.len() >= self.max_lines {
                        truncated_lines = true;
                        break;
                    }

                    lines.push(line);
                }

                Ok(FilePreview::new(
                    FilePreviewKind::Text,
                    lines.join("\n"),
                    truncated_bytes || truncated_lines,
                ))
            }
            Err(_) => Ok(FilePreview::new(
                FilePreviewKind::Binary,
                Self::hex_preview(bytes),
                truncated_bytes,
            )),
        }
    }

    fn hex_preview(bytes: &[u8]) -> String {
        bytes
            .chunks(16)
            .enumerate()
            .map(|(offset, chunk)| {
                let hex = chunk
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<Vec<_>>()
                    .join(" ");

                format!("{:08x}  {}", offset * 16, hex)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn max_bytes(&self) -> usize {
        self.max_bytes
    }

    pub fn max_lines(&self) -> usize {
        self.max_lines
    }
}
