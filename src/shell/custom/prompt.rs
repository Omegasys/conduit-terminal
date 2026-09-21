#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustomPromptState {
    Unknown,
    Ready,
    Running,
    Continuation,
}

#[derive(Clone, Debug)]
pub struct CustomPrompt {
    template: String,
    current: String,
    state: CustomPromptState,
}

impl CustomPrompt {
    pub fn new<S: Into<String>>(
        template: S,
    ) -> Self {
        Self {
            template: template.into(),
            current: String::new(),
            state: CustomPromptState::Unknown,
        }
    }

    pub fn template(&self) -> &str {
        &self.template
    }

    pub fn current(&self) -> &str {
        &self.current
    }

    pub fn state(&self) -> CustomPromptState {
        self.state
    }

    pub fn set_template<S: Into<String>>(
        &mut self,
        template: S,
    ) {
        self.template = template.into();
    }

    pub fn set_current<S: Into<String>>(
        &mut self,
        prompt: S,
    ) {
        self.current = prompt.into();
    }

    pub fn set_state(
        &mut self,
        state: CustomPromptState,
    ) {
        self.state = state;
    }

    pub fn is_ready(&self) -> bool {
        matches!(
            self.state,
            CustomPromptState::Ready
        )
    }

    pub fn is_running(&self) -> bool {
        matches!(
            self.state,
            CustomPromptState::Running
        )
    }

    pub fn is_continuation(&self) -> bool {
        matches!(
            self.state,
            CustomPromptState::Continuation
        )
    }

    pub fn reset(&mut self) {
        self.current.clear();
        self.state = CustomPromptState::Unknown;
    }
}

impl Default for CustomPrompt {
    fn default() -> Self {
        Self::new("> ")
    }
}
