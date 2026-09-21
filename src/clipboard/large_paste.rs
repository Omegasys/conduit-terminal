use super::paste_protection::PasteDecision;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargePasteStrategy {
    Allow,
    Confirm,
    Chunk,
    Block,
}

#[derive(Debug, Clone)]
pub struct LargePasteConfig {
    pub threshold_bytes: usize,
    pub threshold_lines: usize,
    pub strategy: LargePasteStrategy,
    pub chunk_size: usize,
}

impl Default for LargePasteConfig {
    fn default() -> Self {
        Self {
            threshold_bytes: 1024 * 1024,
            threshold_lines: 10_000,
            strategy: LargePasteStrategy::Confirm,
            chunk_size: 64 * 1024,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LargePasteHandler {
    config: LargePasteConfig,
}

impl LargePasteHandler {
    pub fn new(config: LargePasteConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &LargePasteConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut LargePasteConfig {
        &mut self.config
    }

    pub fn is_large(&self, text: &str) -> bool {
        text.len() >= self.config.threshold_bytes
            || text.lines().count() >= self.config.threshold_lines
    }

    pub fn prepare(&self, text: &str) -> PasteDecision {
        if !self.is_large(text) {
            return PasteDecision::Allowed;
        }

        let bytes = text.len();
        let lines = text.lines().count();

        match self.config.strategy {
            LargePasteStrategy::Allow => PasteDecision::Allowed,

            LargePasteStrategy::Confirm => {
                PasteDecision::LargePaste { bytes, lines }
            }

            LargePasteStrategy::Chunk => {
                PasteDecision::LargePaste { bytes, lines }
            }

            LargePasteStrategy::Block => PasteDecision::Blocked,
        }
    }

    pub fn chunks<'a>(&self, text: &'a str) -> Vec<&'a str> {
        if self.config.chunk_size == 0 || text.len() <= self.config.chunk_size {
            return vec![text];
        }

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < text.len() {
            let mut end = (start + self.config.chunk_size).min(text.len());

            while end > start && !text.is_char_boundary(end) {
                end -= 1;
            }

            if end == start {
                if let Some(character) = text[start..].chars().next() {
                    end = start + character.len_utf8();
                } else {
                    break;
                }
            }

            chunks.push(&text[start..end]);
            start = end;
        }

        chunks
    }
}
