#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SixelCommand {
    Character(u8),
    Repeat {
        count: usize,
        value: u8,
    },
    Color {
        index: u16,
        red: u8,
        green: u8,
        blue: u8,
    },
    CarriageReturn,
    NewLine,
    RasterAttributes(Vec<u16>),
}

#[derive(Debug, Default)]
pub struct SixelParser;

impl SixelParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &[u8]) -> Vec<SixelCommand> {
        let mut commands = Vec::new();
        let mut index = 0;

        while index < input.len() {
            match input[index] {
                b'!' => {
                    index += 1;

                    let start = index;

                    while index < input.len()
                        && input[index].is_ascii_digit()
                    {
                        index += 1;
                    }

                    let count = std::str::from_utf8(&input[start..index])
                        .ok()
                        .and_then(|value| value.parse::<usize>().ok())
                        .unwrap_or(1);

                    if index < input.len() {
                        commands.push(SixelCommand::Repeat {
                            count,
                            value: input[index],
                        });

                        index += 1;
                    }
                }

                b'$' => {
                    commands.push(SixelCommand::CarriageReturn);
                    index += 1;
                }

                b'-' => {
                    commands.push(SixelCommand::NewLine);
                    index += 1;
                }

                b'#' => {
                    index += 1;

                    let start = index;

                    while index < input.len()
                        && input[index].is_ascii_digit()
                    {
                        index += 1;
                    }

                    let color_index =
                        std::str::from_utf8(&input[start..index])
                            .ok()
                            .and_then(|value| value.parse().ok())
                            .unwrap_or(0);

                    commands.push(SixelCommand::Color {
                        index: color_index,
                        red: 0,
                        green: 0,
                        blue: 0,
                    });
                }

                value => {
                    commands.push(SixelCommand::Character(value));
                    index += 1;
                }
            }
        }

        commands
    }
}
