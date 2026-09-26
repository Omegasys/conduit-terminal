#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ecma48Parameter {
    Value(u16),
    Omitted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ecma48Csi {
    pub private: Option<u8>,
    pub parameters: Vec<Ecma48Parameter>,
    pub intermediates: Vec<u8>,
    pub final_byte: u8,
}

impl Ecma48Csi {
    pub fn parse(input: &[u8]) -> Option<Self> {
        if input.is_empty() {
            return None;
        }

        let final_byte = *input.last()?;

        if !(0x40..=0x7E).contains(&final_byte) {
            return None;
        }

        let body = &input[..input.len() - 1];

        let mut index = 0;
        let private = if body.first().is_some_and(|b| {
            matches!(*b, b'?' | b'>' | b'!' | b'=' | b'<')
        }) {
            let value = body[0];
            index = 1;
            Some(value)
        } else {
            None
        };

        let mut parameter_end = index;

        while parameter_end < body.len()
            && ((0x30..=0x3F).contains(&body[parameter_end]))
        {
            parameter_end += 1;
        }

        let parameter_bytes = &body[index..parameter_end];

        let parameters = if parameter_bytes.is_empty() {
            Vec::new()
        } else {
            parameter_bytes
                .split(|byte| *byte == b';' || *byte == b':')
                .map(|part| {
                    if part.is_empty() {
                        Ecma48Parameter::Omitted
                    } else {
                        std::str::from_utf8(part)
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .map(Ecma48Parameter::Value)
                            .unwrap_or(Ecma48Parameter::Omitted)
                    }
                })
                .collect()
        };

        Some(Self {
            private,
            parameters,
            intermediates: body[parameter_end..].to_vec(),
            final_byte,
        })
    }
}
