//! xterm protocol extensions.
//!
//! xterm extends the ANSI/VT family with OSC commands, private CSI modes,
//! alternate screen buffers, hyperlinks, clipboard operations, title
//! management, bracketed paste, mouse reporting, and other extensions.

use super::ansi::{AnsiAction, AnsiDecoder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XtermCapabilities {
    pub true_color: bool,
    pub indexed_color: bool,
    pub alternate_screen: bool,
    pub bracketed_paste: bool,
    pub focus_reporting: bool,
    pub mouse_reporting: bool,
    pub sgr_mouse: bool,
    pub hyperlinks: bool,
    pub clipboard: bool,
    pub title_reporting: bool,
    pub synchronized_updates: bool,
    pub sixel: bool,
    pub kitty_graphics: bool,
}

impl Default for XtermCapabilities {
    fn default() -> Self {
        Self {
            true_color: true,
            indexed_color: true,
            alternate_screen: true,
            bracketed_paste: true,
            focus_reporting: true,
            mouse_reporting: true,
            sgr_mouse: true,
            hyperlinks: true,
            clipboard: true,
            title_reporting: true,
            synchronized_updates: true,
            sixel: false,
            kitty_graphics: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum XtermAction {
    Ansi(AnsiAction),

    SetTitle(String),
    SetIconName(String),
    SetTitleAndIcon(String),

    Hyperlink {
        id: Option<String>,
        uri: String,
    },

    Clipboard {
        command: String,
        data: String,
    },

    SetColor {
        target: String,
        value: String,
    },

    ResetColor {
        target: String,
    },

    EnableAlternateScreen,
    DisableAlternateScreen,

    EnableBracketedPaste,
    DisableBracketedPaste,

    EnableFocusReporting,
    DisableFocusReporting,

    EnableMouseReporting,
    DisableMouseReporting,

    EnableSynchronizedUpdates,
    DisableSynchronizedUpdates,

    UnknownOsc(String),
}

#[derive(Debug, Clone)]
pub struct XtermProtocol {
    capabilities: XtermCapabilities,
    decoder: AnsiDecoder,
}

impl Default for XtermProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl XtermProtocol {
    pub fn new() -> Self {
        Self {
            capabilities: XtermCapabilities::default(),
            decoder: AnsiDecoder::new(),
        }
    }

    pub fn capabilities(&self) -> XtermCapabilities {
        self.capabilities
    }

    pub fn decoder(&self) -> &AnsiDecoder {
        &self.decoder
    }

    pub fn decoder_mut(&mut self) -> &mut AnsiDecoder {
        &mut self.decoder
    }

    pub fn reset(&mut self) {
        self.decoder.reset();
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Vec<XtermAction> {
        self.decoder
            .feed(bytes)
            .into_iter()
            .flat_map(|action| self.translate(action))
            .collect()
    }

    fn translate(&self, action: AnsiAction) -> Vec<XtermAction> {
        match action {
            AnsiAction::OperatingSystemCommand(command) => {
                self.parse_osc(&command)
            }

            AnsiAction::SetMode { mode, enabled } => {
                match mode {
                    1049 | 47 | 1047 => {
                        if enabled {
                            vec![XtermAction::EnableAlternateScreen]
                        } else {
                            vec![XtermAction::DisableAlternateScreen]
                        }
                    }

                    2004 => {
                        if enabled {
                            vec![XtermAction::EnableBracketedPaste]
                        } else {
                            vec![XtermAction::DisableBracketedPaste]
                        }
                    }

                    1004 => {
                        if enabled {
                            vec![XtermAction::EnableFocusReporting]
                        } else {
                            vec![XtermAction::DisableFocusReporting]
                        }
                    }

                    1000 | 1002 | 1003 | 1006 => {
                        if enabled {
                            vec![XtermAction::EnableMouseReporting]
                        } else {
                            vec![XtermAction::DisableMouseReporting]
                        }
                    }

                    2026 => {
                        if enabled {
                            vec![XtermAction::EnableSynchronizedUpdates]
                        } else {
                            vec![XtermAction::DisableSynchronizedUpdates]
                        }
                    }

                    _ => vec![XtermAction::Ansi(
                        AnsiAction::SetMode { mode, enabled },
                    )],
                }
            }

            AnsiAction::ResetMode { mode } => {
                match mode {
                    1049 | 47 | 1047 => {
                        vec![XtermAction::DisableAlternateScreen]
                    }

                    2004 => vec![XtermAction::DisableBracketedPaste],

                    1004 => vec![XtermAction::DisableFocusReporting],

                    1000 | 1002 | 1003 | 1006 => {
                        vec![XtermAction::DisableMouseReporting]
                    }

                    2026 => vec![XtermAction::DisableSynchronizedUpdates],

                    _ => vec![XtermAction::Ansi(AnsiAction::ResetMode { mode })],
                }
            }

            other => vec![XtermAction::Ansi(other)],
        }
    }

    fn parse_osc(&self, command: &str) -> Vec<XtermAction> {
        let mut parts = command.splitn(2, ';');

        let code = parts.next().unwrap_or_default();
        let value = parts.next().unwrap_or_default();

        match code {
            "0" => vec![XtermAction::SetTitleAndIcon(value.to_string())],

            "1" => vec![XtermAction::SetIconName(value.to_string())],

            "2" => vec![XtermAction::SetTitle(value.to_string())],

            "4" => {
                vec![XtermAction::SetColor {
                    target: "palette".to_string(),
                    value: value.to_string(),
                }]
            }

            "8" => self.parse_hyperlink(value),

            "10" | "11" | "12" | "17" | "19" => {
                vec![XtermAction::SetColor {
                    target: code.to_string(),
                    value: value.to_string(),
                }]
            }

            "52" => {
                vec![XtermAction::Clipboard {
                    command: code.to_string(),
                    data: value.to_string(),
                }]
            }

            "104" | "110" | "111" | "112" | "113" | "114" => {
                vec![XtermAction::ResetColor {
                    target: code.to_string(),
                }]
            }

            _ => vec![XtermAction::UnknownOsc(command.to_string())],
        }
    }

    fn parse_hyperlink(&self, value: &str) -> Vec<XtermAction> {
        let mut parts = value.splitn(2, ';');

        let params = parts.next().unwrap_or_default();
        let uri = parts.next().unwrap_or_default();

        let id = params
            .split(':')
            .find_map(|part| part.strip_prefix("id="))
            .map(str::to_string);

        if uri.is_empty() {
            vec![XtermAction::Hyperlink {
                id,
                uri: String::new(),
            }]
        } else {
            vec![XtermAction::Hyperlink {
                id,
                uri: uri.to_string(),
            }]
        }
    }
}
