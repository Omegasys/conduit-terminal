#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Iterm2UserVariable {
    pub name: String,
    pub value: String,
}

impl Iterm2UserVariable {
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Iterm2Notification {
    pub title: Option<String>,
    pub body: String,
}

impl Iterm2Notification {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            title: None,
            body: body.into(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}
