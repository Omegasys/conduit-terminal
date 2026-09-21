//! Bracketed paste mode support.
//!
//! Bracketed paste allows terminal applications to distinguish pasted text
//! from ordinary keyboard input.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketedPasteState {
    Disabled,
    Enabled,
}

impl Default for BracketedPasteState {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BracketedPasteEvent {
    Start,
    Data(Vec<u8>),
    End,
}

#[derive(Debug, Clone)]
pub struct BracketedPaste {
    state: BracketedPasteState,
}

impl Default for BracketedPaste {
    fn default() -> Self {
        Self::new()
    }
}

impl BracketedPaste {
    pub const START_SEQUENCE: &'static [u8] = b"\x1b[200~";
    pub const END_SEQUENCE: &'static [u8] = b"\x1b[201~";

    pub fn new() -> Self {
        Self {
            state: BracketedPasteState::Disabled,
        }
    }

    pub fn state(&self) -> BracketedPasteState {
        self.state
    }

    pub fn enabled(&self) -> bool {
        self.state == BracketedPasteState::Enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.state = if enabled {
            BracketedPasteState::Enabled
        } else {
            BracketedPasteState::Disabled
        };
    }

    pub fn enable(&mut self) {
        self.set_enabled(true);
    }

    pub fn disable(&mut self) {
        self.set_enabled(false);
    }

    pub fn wrap(&self, data: &[u8]) -> Vec<u8> {
        if !self.enabled() {
            return data.to_vec();
        }

        let mut result = Vec::with_capacity(
            Self::START_SEQUENCE.len()
                + data.len()
                + Self::END_SEQUENCE.len(),
        );

        result.extend_from_slice(Self::START_SEQUENCE);
        result.extend_from_slice(data);
        result.extend_from_slice(Self::END_SEQUENCE);

        result
    }

    pub fn start_sequence(&self) -> &'static [u8] {
        Self::START_SEQUENCE
    }

    pub fn end_sequence(&self) -> &'static [u8] {
        Self::END_SEQUENCE
    }
}

#[derive(Debug, Clone)]
pub struct BracketedPasteParser {
    buffer: Vec<u8>,
    active: bool,
}

impl Default for BracketedPasteParser {
    fn default() -> Self {
        Self::new()
    }
}

impl BracketedPasteParser {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            active: false,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.active = false;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn feed(&mut self, data: &[u8]) -> Vec<BracketedPasteEvent> {
        let mut events = Vec::new();
        self.buffer.extend_from_slice(data);

        loop {
            if !self.active {
                if let Some(position) = find_sequence(
                    &self.buffer,
                    BracketedPaste::START_SEQUENCE,
                ) {
                    self.buffer.drain(..position + BracketedPaste::START_SEQUENCE.len());
                    self.active = true;
                    events.push(BracketedPasteEvent::Start);
                    continue;
                }

                break;
            }

            if let Some(position) = find_sequence(
                &self.buffer,
                BracketedPaste::END_SEQUENCE,
            ) {
                let content = self.buffer.drain(..position).collect::<Vec<_>>();

                self.buffer
                    .drain(..BracketedPaste::END_SEQUENCE.len());

                if !content.is_empty() {
                    events.push(BracketedPasteEvent::Data(content));
                }

                self.active = false;
                events.push(BracketedPasteEvent::End);
                continue;
            }

            break;
        }

        events
    }
}

fn find_sequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
