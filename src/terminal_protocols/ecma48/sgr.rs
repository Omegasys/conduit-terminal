#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SgrAttribute {
    Reset,
    Bold,
    Faint,
    Italic,
    Underline,
    DoubleUnderline,
    Blink,
    Reverse,
    Conceal,
    Strikethrough,
    Foreground(u8),
    Background(u8),
    ExtendedForeground(Vec<u16>),
    ExtendedBackground(Vec<u16>),
    Unknown(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SgrSequence {
    pub attributes: Vec<SgrAttribute>,
}

impl SgrSequence {
    pub fn parse(parameters: &[u16]) -> Self {
        let mut attributes = Vec::new();
        let mut index = 0;

        while index < parameters.len() {
            let value = parameters[index];

            match value {
                0 => attributes.push(SgrAttribute::Reset),
                1 => attributes.push(SgrAttribute::Bold),
                2 => attributes.push(SgrAttribute::Faint),
                3 => attributes.push(SgrAttribute::Italic),
                4 => attributes.push(SgrAttribute::Underline),
                5 | 6 => attributes.push(SgrAttribute::Blink),
                7 => attributes.push(SgrAttribute::Reverse),
                8 => attributes.push(SgrAttribute::Conceal),
                9 => attributes.push(SgrAttribute::Strikethrough),

                21 => attributes.push(SgrAttribute::DoubleUnderline),

                30..=37 | 90..=97 => {
                    attributes.push(SgrAttribute::Foreground(value as u8));
                }

                40..=47 | 100..=107 => {
                    attributes.push(SgrAttribute::Background(value as u8));
                }

                38 | 48 => {
                    let mut values = Vec::new();

                    if let Some(mode) = parameters.get(index + 1) {
                        values.push(*mode);

                        if *mode == 5 {
                            if let Some(color) = parameters.get(index + 2) {
                                values.push(*color);
                                index += 2;
                            }
                        } else if *mode == 2 {
                            for offset in 2..=4 {
                                if let Some(color) =
                                    parameters.get(index + offset)
                                {
                                    values.push(*color);
                                }
                            }

                            index += 4;
                        }
                    }

                    if value == 38 {
                        attributes.push(
                            SgrAttribute::ExtendedForeground(values),
                        );
                    } else {
                        attributes.push(
                            SgrAttribute::ExtendedBackground(values),
                        );
                    }
                }

                _ => attributes.push(SgrAttribute::Unknown(value)),
            }

            index += 1;
        }

        Self { attributes }
    }
}
