//! Kitty terminal protocol extensions.
//!
//! Includes Kitty keyboard protocol and Kitty graphics protocol primitives.
//! Actual graphics storage/rendering belongs to the renderer subsystem.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KittyKeyboardMode {
    Disabled,
    Disambiguate,
    ReportEvents,
    ReportAllKeys,
}

impl Default for KittyKeyboardMode {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KittyKeyboardFlags {
    pub disambiguate: bool,
    pub report_events: bool,
    pub report_alternate_keys: bool,
    pub report_all_keys: bool,
}

impl Default for KittyKeyboardFlags {
    fn default() -> Self {
        Self {
            disambiguate: false,
            report_events: false,
            report_alternate_keys: false,
            report_all_keys: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KittyKeyboardState {
    pub mode: KittyKeyboardMode,
    pub flags: KittyKeyboardFlags,
}

impl Default for KittyKeyboardState {
    fn default() -> Self {
        Self {
            mode: KittyKeyboardMode::Disabled,
            flags: KittyKeyboardFlags::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KittyGraphicsFormat {
    Rgba,
    Rgb,
    Png,
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KittyGraphicsAction {
    Transmit,
    Put,
    Delete,
    Query,
    Frame,
    Animate,
    Unknown(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KittyGraphicsCommand {
    pub action: KittyGraphicsAction,
    pub format: Option<KittyGraphicsFormat>,
    pub image_id: Option<u32>,
    pub placement_id: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub payload: Vec<u8>,
}

impl Default for KittyGraphicsCommand {
    fn default() -> Self {
        Self {
            action: KittyGraphicsAction::Unknown(0),
            format: None,
            image_id: None,
            placement_id: None,
            width: None,
            height: None,
            payload: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct KittyProtocol {
    keyboard: KittyKeyboardState,
}

impl KittyProtocol {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keyboard(&self) -> &KittyKeyboardState {
        &self.keyboard
    }

    pub fn set_keyboard_mode(&mut self, mode: KittyKeyboardMode) {
        self.keyboard.mode = mode;
    }

    pub fn parse_keyboard_push(
        &mut self,
        value: &str,
    ) -> Option<KittyKeyboardState> {
        let flags = value
            .split(';')
            .next()?
            .parse::<u16>()
            .ok()?;

        self.keyboard.flags.disambiguate = flags & 1 != 0;
        self.keyboard.flags.report_events = flags & 2 != 0;
        self.keyboard.flags.report_alternate_keys = flags & 4 != 0;
        self.keyboard.flags.report_all_keys = flags & 8 != 0;

        self.keyboard.mode = if self.keyboard.flags.report_all_keys {
            KittyKeyboardMode::ReportAllKeys
        } else if self.keyboard.flags.report_events {
            KittyKeyboardMode::ReportEvents
        } else if self.keyboard.flags.disambiguate {
            KittyKeyboardMode::Disambiguate
        } else {
            KittyKeyboardMode::Disabled
        };

        Some(self.keyboard.clone())
    }

    pub fn parse_graphics(
        &self,
        parameters: &str,
        payload: &[u8],
    ) -> KittyGraphicsCommand {
        let mut command = KittyGraphicsCommand {
            payload: payload.to_vec(),
            ..Default::default()
        };

        for item in parameters.split(',') {
            let mut pair = item.splitn(2, '=');

            let key = pair.next().unwrap_or_default();
            let value = pair.next().unwrap_or_default();

            match key {
                "a" => {
                    command.action = match value {
                        "t" => KittyGraphicsAction::Transmit,
                        "p" => KittyGraphicsAction::Put,
                        "d" => KittyGraphicsAction::Delete,
                        "q" => KittyGraphicsAction::Query,
                        "f" => KittyGraphicsAction::Frame,
                        "a" => KittyGraphicsAction::Animate,
                        _ => KittyGraphicsAction::Unknown(
                            value.bytes().next().unwrap_or(0),
                        ),
                    };
                }

                "f" => {
                    command.format = value.parse::<u16>().ok().map(|format| {
                        match format {
                            24 => KittyGraphicsFormat::Rgb,
                            32 => KittyGraphicsFormat::Rgba,
                            100 => KittyGraphicsFormat::Png,
                            other => KittyGraphicsFormat::Unknown(other),
                        }
                    });
                }

                "i" => {
                    command.image_id = value.parse().ok();
                }

                "p" => {
                    command.placement_id = value.parse().ok();
                }

                "s" => {
                    command.width = value.parse().ok();
                }

                "v" => {
                    command.height = value.parse().ok();
                }

                _ => {}
            }
        }

        command
    }
}
