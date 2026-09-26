#[derive(Debug, Clone, PartialEq)]
pub enum RegisCommand {
    Move(f32, f32),
    Draw(f32, f32),
    Point(f32, f32),
    LineTo(f32, f32),
    SetColor(u8),
    SetWidth(f32),
    Text(String),
    Escape,
    Unknown(char),
}

#[derive(Debug, Default)]
pub struct RegisParser;

impl RegisParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> Vec<RegisCommand> {
        let mut commands = Vec::new();

        for token in input.split_whitespace() {
            let mut chars = token.chars();

            let Some(command) = chars.next() else {
                continue;
            };

            let value = chars.as_str();

            let parsed = match command {
                'M' => parse_pair(value).map(|(x, y)| RegisCommand::Move(x, y)),
                'D' => parse_pair(value).map(|(x, y)| RegisCommand::Draw(x, y)),
                'P' => parse_pair(value).map(|(x, y)| RegisCommand::Point(x, y)),
                'L' => parse_pair(value).map(|(x, y)| RegisCommand::LineTo(x, y)),
                'C' => value.parse().ok().map(RegisCommand::SetColor),
                'W' => value.parse().ok().map(RegisCommand::SetWidth),
                'T' => Some(RegisCommand::Text(value.to_string())),
                _ => Some(RegisCommand::Unknown(command)),
            };

            if let Some(command) = parsed {
                commands.push(command);
            }
        }

        commands
    }
}

fn parse_pair(value: &str) -> Option<(f32, f32)> {
    let mut parts = value.split(',');

    let x = parts.next()?.parse().ok()?;
    let y = parts.next()?.parse().ok()?;

    Some((x, y))
}
