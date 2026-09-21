#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromptState {
    Unknown,
    Ready,
    Running,
    Continuation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PromptInfo {
    text: String,
    state: PromptState,
    shell_name: Option<String>,
}

impl PromptInfo {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            state: PromptState::Unknown,
            shell_name: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn state(&self) -> PromptState {
        self.state
    }

    pub fn shell_name(&self) -> Option<&str> {
        self.shell_name.as_deref()
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.text = text.into();
    }

    pub fn set_state(&mut self, state: PromptState) {
        self.state = state;
    }

    pub fn set_shell_name<S: Into<String>>(&mut self, shell: S) {
        self.shell_name = Some(shell.into());
    }

    pub fn clear_shell_name(&mut self) {
        self.shell_name = None;
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, PromptState::Ready)
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, PromptState::Running)
    }

    pub fn is_continuation(&self) -> bool {
        matches!(self.state, PromptState::Continuation)
    }
}

impl Default for PromptInfo {
    fn default() -> Self {
        Self::new("")
    }
}

#[derive(Clone, Debug)]
pub struct PromptDetector {
    markers: Vec<String>,
    state: PromptState,
}

impl PromptDetector {
    pub fn new() -> Self {
        Self {
            markers: Vec::new(),
            state: PromptState::Unknown,
        }
    }

    pub fn add_marker<S: Into<String>>(&mut self, marker: S) {
        let marker = marker.into();

        if !self.markers.contains(&marker) {
            self.markers.push(marker);
        }
    }

    pub fn set_markers<I, S>(&mut self, markers: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.markers.clear();

        for marker in markers {
            self.add_marker(marker);
        }
    }

    pub fn markers(&self) -> &[String] {
        &self.markers
    }

    pub fn detect(&mut self, output: &str) -> Option<PromptInfo> {
        let line = output.lines().last()?;

        if !self
            .markers
            .iter()
            .any(|marker| line.contains(marker))
        {
            return None;
        }

        self.state = PromptState::Ready;

        let mut info = PromptInfo::new(line);
        info.set_state(PromptState::Ready);

        Some(info)
    }

    pub fn set_state(&mut self, state: PromptState) {
        self.state = state;
    }

    pub fn state(&self) -> PromptState {
        self.state
    }

    pub fn reset(&mut self) {
        self.state = PromptState::Unknown;
    }
}

impl Default for PromptDetector {
    fn default() -> Self {
        Self::new()
    }
}
