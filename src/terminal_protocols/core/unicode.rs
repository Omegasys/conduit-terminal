#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnicodeWidthMode {
    Narrow,
    Unicode,
    Cjk,
}

#[derive(Debug, Clone, Copy)]
pub struct UnicodeWidthProvider {
    mode: UnicodeWidthMode,
}

impl Default for UnicodeWidthProvider {
    fn default() -> Self {
        Self {
            mode: UnicodeWidthMode::Unicode,
        }
    }
}

impl UnicodeWidthProvider {
    pub fn new(mode: UnicodeWidthMode) -> Self {
        Self { mode }
    }

    pub fn mode(&self) -> UnicodeWidthMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: UnicodeWidthMode) {
        self.mode = mode;
    }

    pub fn width(&self, character: char) -> usize {
        if character == '\0' || character.is_control() {
            return 0;
        }

        match self.mode {
            UnicodeWidthMode::Narrow => 1,
            UnicodeWidthMode::Unicode => unicode_width(character),
            UnicodeWidthMode::Cjk => {
                if is_cjk(character) {
                    2
                } else {
                    unicode_width(character)
                }
            }
        }
    }
}

fn unicode_width(character: char) -> usize {
    let code = character as u32;

    if is_combining(code) {
        return 0;
    }

    if is_wide(code) {
        return 2;
    }

    1
}

fn is_combining(code: u32) -> bool {
    matches!(
        code,
        0x0300..=0x036F
            | 0x1AB0..=0x1AFF
            | 0x1DC0..=0x1DFF
            | 0x20D0..=0x20FF
            | 0xFE20..=0xFE2F
    )
}

fn is_wide(code: u32) -> bool {
    matches!(
        code,
        0x1100..=0x115F
            | 0x2329..=0x232A
            | 0x2E80..=0xA4CF
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
    )
}

fn is_cjk(character: char) -> bool {
    let code = character as u32;

    matches!(
        code,
        0x3040..=0x30FF
            | 0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xAC00..=0xD7AF
    )
}
