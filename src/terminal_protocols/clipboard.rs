//! Terminal clipboard protocol support.
//!
//! This module represents clipboard operations requested through terminal
//! protocols such as OSC 52. Actual clipboard access should be performed by
//! Conduit's clipboard/security subsystem.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardSelection {
    Primary,
    Clipboard,
    Secondary,
    Select,
    CutBuffer(u8),
    Unknown,
}

impl ClipboardSelection {
    pub fn from_name(value: &str) -> Self {
        match value {
            "p" | "P" | "primary" => Self::Primary,
            "c" | "C" | "clipboard" => Self::Clipboard,
            "s" | "S" | "secondary" => Self::Secondary,
            "0" => Self::CutBuffer(0),
            "1" => Self::CutBuffer(1),
            "2" => Self::CutBuffer(2),
            "3" => Self::CutBuffer(3),
            "4" => Self::CutBuffer(4),
            "5" => Self::CutBuffer(5),
            "6" => Self::CutBuffer(6),
            "7" => Self::CutBuffer(7),
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardOperation {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardRequest {
    pub operation: ClipboardOperation,
    pub selection: ClipboardSelection,
    pub data: Option<Vec<u8>>,
}

impl ClipboardRequest {
    pub fn read(selection: ClipboardSelection) -> Self {
        Self {
            operation: ClipboardOperation::Read,
            selection,
            data: None,
        }
    }

    pub fn write(selection: ClipboardSelection, data: Vec<u8>) -> Self {
        Self {
            operation: ClipboardOperation::Write,
            selection,
            data: Some(data),
        }
    }

    pub fn is_read(&self) -> bool {
        self.operation == ClipboardOperation::Read
    }

    pub fn is_write(&self) -> bool {
        self.operation == ClipboardOperation::Write
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardError {
    InvalidEncoding,
    InvalidSelection,
    EmptyRequest,
    DecodeFailed,
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEncoding => write!(f, "invalid clipboard encoding"),
            Self::InvalidSelection => write!(f, "invalid clipboard selection"),
            Self::EmptyRequest => write!(f, "empty clipboard request"),
            Self::DecodeFailed => write!(f, "clipboard data decoding failed"),
        }
    }
}

impl std::error::Error for ClipboardError {}

#[derive(Debug, Clone, Default)]
pub struct ClipboardParser;

impl ClipboardParser {
    pub fn new() -> Self {
        Self
    }

    /// Parses an OSC 52-style payload.
    ///
    /// The payload has the form:
    ///
    /// `selection;base64-data`
    pub fn parse_osc52(
        &self,
        value: &str,
    ) -> Result<ClipboardRequest, ClipboardError> {
        let mut parts = value.splitn(2, ';');

        let selection = parts
            .next()
            .ok_or(ClipboardError::EmptyRequest)?;

        let data = parts.next().unwrap_or_default();

        let selection = ClipboardSelection::from_name(selection);

        if selection == ClipboardSelection::Unknown {
            return Err(ClipboardError::InvalidSelection);
        }

        if data.is_empty() {
            return Ok(ClipboardRequest::read(selection));
        }

        let decoded = decode_base64(data)?;

        Ok(ClipboardRequest::write(selection, decoded))
    }
}

fn decode_base64(value: &str) -> Result<Vec<u8>, ClipboardError> {
    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0u8;

    for byte in value.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            b'\r' | b'\n' | b' ' | b'\t' => continue,
            _ => return Err(ClipboardError::InvalidEncoding),
        };

        buffer = (buffer << 6) | value as u32;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            output.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }

    Ok(output)
}
