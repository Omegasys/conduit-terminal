use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaneCommandAction {
    Create,
    Close,
    Focus,
    FocusNext,
    FocusPrevious,
    SplitHorizontal,
    SplitVertical,
    Resize,
    Zoom,
    Unzoom,
    Swap,
    Synchronize,
    Unsynchronize,
    List,
}

impl PaneCommandAction {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Create => "new-pane",
            Self::Close => "close-pane",
            Self::Focus => "focus-pane",
            Self::FocusNext => "focus-next-pane",
            Self::FocusPrevious => "focus-previous-pane",
            Self::SplitHorizontal => "split-horizontal",
            Self::SplitVertical => "split-vertical",
            Self::Resize => "resize-pane",
            Self::Zoom => "zoom-pane",
            Self::Unzoom => "unzoom-pane",
            Self::Swap => "swap-pane",
            Self::Synchronize => "synchronize-panes",
            Self::Unsynchronize => "unsynchronize-panes",
            Self::List => "list-panes",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaneCommand {
    action: PaneCommandAction,
    pane_id: Option<String>,
    value: Option<i32>,
}

impl PaneCommand {
    pub fn new(action: PaneCommandAction) -> Self {
        Self {
            action,
            pane_id: None,
            value: None,
        }
    }

    pub fn with_pane_id(
        action: PaneCommandAction,
        pane_id: impl Into<String>,
    ) -> Self {
        Self {
            action,
            pane_id: Some(pane_id.into()),
            value: None,
        }
    }

    pub fn with_value(
        action: PaneCommandAction,
        value: i32,
    ) -> Self {
        Self {
            action,
            pane_id: None,
            value: Some(value),
        }
    }

    pub fn action(&self) -> PaneCommandAction {
        self.action
    }

    pub fn pane_id(&self) -> Option<&str> {
        self.pane_id.as_deref()
    }

    pub fn value(&self) -> Option<i32> {
        self.value
    }

    pub fn set_pane_id(&mut self, id: impl Into<String>) {
        self.pane_id = Some(id.into());
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = Some(value);
    }

    pub fn is_targeted(&self) -> bool {
        self.pane_id.is_some()
    }
}

impl fmt::Display for PaneCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.action.name())?;

        if let Some(id) = &self.pane_id {
            write!(formatter, " {}", id)?;
        }

        if let Some(value) = self.value {
            write!(formatter, " {}", value)?;
        }

        Ok(())
    }
}
