/// Matches a text range only when it forms a complete word.
#[derive(Debug, Clone, Copy, Default)]
pub struct WholeWordMatcher;

impl WholeWordMatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn is_word_character(character: char) -> bool {
        character.is_alphanumeric() || character == '_'
    }

    pub fn is_boundary(text: &str, index: usize) -> bool {
        if index == 0 || index >= text.len() {
            return true;
        }

        let previous = text[..index].chars().next_back();
        let next = text[index..].chars().next();

        match (previous, next) {
            (Some(left), Some(right)) => {
                !Self::is_word_character(left) || !Self::is_word_character(right)
            }
            _ => true,
        }
    }

    pub fn matches(
        &self,
        text: &str,
        start: usize,
        end: usize,
    ) -> bool {
        if start > end || end > text.len() {
            return false;
        }

        let left_boundary = start == 0
            || text[..start]
                .chars()
                .next_back()
                .map(|c| !Self::is_word_character(c))
                .unwrap_or(true);

        let right_boundary = end == text.len()
            || text[end..]
                .chars()
                .next()
                .map(|c| !Self::is_word_character(c))
                .unwrap_or(true);

        left_boundary && right_boundary
    }
}
