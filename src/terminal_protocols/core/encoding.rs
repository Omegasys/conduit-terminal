use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalEncoding {
    Utf8,
    Latin1,
    Ascii,
}

#[derive(Debug)]
pub struct TerminalTextDecoder {
    encoding: TerminalEncoding,
}

impl Default for TerminalTextDecoder {
    fn default() -> Self {
        Self::new(TerminalEncoding::Utf8)
    }
}

impl TerminalTextDecoder {
    pub fn new(encoding: TerminalEncoding) -> Self {
        Self { encoding }
    }

    pub fn encoding(&self) -> TerminalEncoding {
        self.encoding
    }

    pub fn set_encoding(&mut self, encoding: TerminalEncoding) {
        self.encoding = encoding;
    }

    pub fn decode<'a>(&self, data: &'a [u8]) -> Cow<'a, str> {
        match self.encoding {
            TerminalEncoding::Utf8 => String::from_utf8_lossy(data),

            TerminalEncoding::Ascii => {
                Cow::Owned(
                    data.iter()
                        .map(|byte| {
                            if *byte < 0x80 {
                                *byte as char
                            } else {
                                '\u{FFFD}'
                            }
                        })
                        .collect(),
                )
            }

            TerminalEncoding::Latin1 => Cow::Owned(
                data.iter().map(|byte| *byte as char).collect(),
            ),
        }
    }
}
