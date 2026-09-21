//! OSC (Operating System Command) parsing.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OscCommand {
    SetTitle(String),
    SetIconName(String),
    SetTitleAndIcon(String),

    SetPalette {
        index: u16,
        color: String,
    },

    SetForeground(String),
    SetBackground(String),
    SetCursorColor(String),

    ResetPalette {
        index: Option<u16>,
    },

    Hyperlink {
        id: Option<String>,
        uri: String,
    },

    Clipboard {
        selection: String,
        data: String,
    },

    Notification {
        title: String,
        body: String,
    },

    CurrentDirectory(String),

    Unknown {
        code: u16,
        value: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OscTerminator {
    Bell,
    StringTerminator,
}

#[derive(Debug, Clone)]
pub struct OscParser {
    buffer: Vec<u8>,
    terminated: bool,
}

impl Default for OscParser {
    fn default() -> Self {
        Self::new()
    }
}

impl OscParser {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            terminated: false,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.terminated = false;
    }

    pub fn feed(&mut self, byte: u8) -> Option<OscCommand> {
        if self.terminated {
            self.reset();
        }

        match byte {
            0x07 => {
                self.terminated = true;
                self.finish()
            }

            0x1b => None,

            _ => {
                self.buffer.push(byte);
                None
            }
        }
    }

    pub fn finish(&mut self) -> Option<OscCommand> {
        if self.buffer.is_empty() {
            self.reset();
            return None;
        }

        let text = String::from_utf8_lossy(&self.buffer).into_owned();
        let result = Self::parse(&text);

        self.reset();

        result
    }

    pub fn parse(value: &str) -> Option<OscCommand> {
        let mut parts = value.splitn(2, ';');

        let code = parts.next()?.parse::<u16>().ok()?;
        let data = parts.next().unwrap_or_default();

        match code {
            0 => Some(OscCommand::SetTitleAndIcon(data.to_string())),
            1 => Some(OscCommand::SetIconName(data.to_string())),
            2 => Some(OscCommand::SetTitle(data.to_string())),

            4 => {
                let mut palette = data.splitn(2, ';');

                let index = palette.next()?.parse::<u16>().ok()?;
                let color = palette.next().unwrap_or_default().to_string();

                Some(OscCommand::SetPalette { index, color })
            }

            10 => Some(OscCommand::SetForeground(data.to_string())),
            11 => Some(OscCommand::SetBackground(data.to_string())),
            12 => Some(OscCommand::SetCursorColor(data.to_string())),

            104 => {
                let index = if data.is_empty() {
                    None
                } else {
                    data.parse::<u16>().ok()
                };

                Some(OscCommand::ResetPalette { index })
            }

            8 => {
                let mut hyperlink = data.splitn(2, ';');
                let params = hyperlink.next().unwrap_or_default();
                let uri = hyperlink.next().unwrap_or_default();

                let id = params
                    .split(':')
                    .find_map(|part| part.strip_prefix("id="))
                    .map(str::to_string);

                Some(OscCommand::Hyperlink {
                    id,
                    uri: uri.to_string(),
                })
            }

            52 => {
                let mut clipboard = data.splitn(2, ';');

                let selection = clipboard.next().unwrap_or_default().to_string();
                let encoded = clipboard.next().unwrap_or_default();

                Some(OscCommand::Clipboard {
                    selection,
                    data: encoded.to_string(),
                })
            }

            9 => {
                let mut notification = data.splitn(2, ';');

                Some(OscCommand::Notification {
                    title: notification.next().unwrap_or_default().to_string(),
                    body: notification.next().unwrap_or_default().to_string(),
                })
            }

            7 => Some(OscCommand::CurrentDirectory(data.to_string())),

            _ => Some(OscCommand::Unknown {
                code,
                value: data.to_string(),
            }),
        }
    }
}
