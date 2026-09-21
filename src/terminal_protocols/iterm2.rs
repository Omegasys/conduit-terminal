//! iTerm2 terminal escape-sequence extensions.
//!
//! Primarily handles OSC 1337 commands and inline image metadata.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Iterm2Command {
    Image(Iterm2ImageCommand),

    File(Iterm2FileCommand),

    SetMark,

    ClearMark,

    SetBadge(String),

    SetCursorShape(u8),

    SetProperty {
        name: String,
        value: String,
    },

    Unknown {
        name: String,
        value: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Iterm2ImageCommand {
    pub name: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub preserve_aspect_ratio: bool,
    pub inline: bool,
    pub payload: Vec<u8>,
}

impl Default for Iterm2ImageCommand {
    fn default() -> Self {
        Self {
            name: None,
            width: None,
            height: None,
            preserve_aspect_ratio: true,
            inline: true,
            payload: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Iterm2FileCommand {
    pub name: Option<String>,
    pub size: Option<u64>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub inline: bool,
    pub payload: Vec<u8>,
}

impl Default for Iterm2FileCommand {
    fn default() -> Self {
        Self {
            name: None,
            size: None,
            width: None,
            height: None,
            inline: false,
            payload: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Iterm2Protocol;

impl Iterm2Protocol {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_osc1337(
        &self,
        value: &str,
    ) -> Option<Iterm2Command> {
        let mut command = value.splitn(2, ';');

        let name = command.next()?.to_string();
        let data = command.next().unwrap_or_default();

        match name.as_str() {
            "File" => Some(Iterm2Command::File(
                self.parse_file(data),
            )),

            "SetMark" => Some(Iterm2Command::SetMark),

            "ClearMark" => Some(Iterm2Command::ClearMark),

            "SetBadgeFormat" => Some(Iterm2Command::SetBadge(
                data.to_string(),
            )),

            "CursorShape" => {
                let shape = data.parse::<u8>().ok()?;

                Some(Iterm2Command::SetCursorShape(shape))
            }

            "CurrentDir" => Some(Iterm2Command::SetProperty {
                name,
                value: data.to_string(),
            }),

            _ if name.starts_with("SetProfileProperty") => {
                Some(Iterm2Command::SetProperty {
                    name,
                    value: data.to_string(),
                })
            }

            _ => Some(Iterm2Command::Unknown {
                name,
                value: data.to_string(),
            }),
        }
    }

    fn parse_file(&self, value: &str) -> Iterm2FileCommand {
        let mut result = Iterm2FileCommand::default();

        let mut parts = value.splitn(2, ':');

        let metadata = parts.next().unwrap_or_default();
        let payload = parts.next().unwrap_or_default();

        for item in metadata.split(';') {
            let mut pair = item.splitn(2, '=');

            let key = pair.next().unwrap_or_default();
            let value = pair.next().unwrap_or_default();

            match key {
                "name" => result.name = Some(value.to_string()),
                "size" => result.size = value.parse().ok(),
                "width" => result.width = Some(value.to_string()),
                "height" => result.height = Some(value.to_string()),
                "inline" => result.inline = value == "1",
                _ => {}
            }
        }

        result.payload = decode_base64(payload).unwrap_or_default();

        result
    }

    pub fn parse_image(&self, value: &str) -> Iterm2ImageCommand {
        let mut result = Iterm2ImageCommand::default();

        let mut parts = value.splitn(2, ':');

        let metadata = parts.next().unwrap_or_default();
        let payload = parts.next().unwrap_or_default();

        for item in metadata.split(';') {
            let mut pair = item.splitn(2, '=');

            let key = pair.next().unwrap_or_default();
            let value = pair.next().unwrap_or_default();

            match key {
                "name" => result.name = Some(value.to_string()),
                "width" => result.width = Some(value.to_string()),
                "height" => result.height = Some(value.to_string()),
                "preserveAspectRatio" => {
                    result.preserve_aspect_ratio = value != "0"
                }
                "inline" => result.inline = value == "1",
                _ => {}
            }
        }

        result.payload = decode_base64(payload).unwrap_or_default();

        result
    }
}

fn decode_base64(value: &str) -> Option<Vec<u8>> {
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
            b'\r' | b'\n' => continue,
            _ => return None,
        };

        buffer = (buffer << 6) | value as u32;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            output.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }

    Some(output)
}
