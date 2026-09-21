#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImeComposition {
    pub text: String,
    pub cursor: usize,
    pub selection_start: Option<usize>,
    pub selection_end: Option<usize>,
}

impl ImeComposition {
    pub fn new(
        text: impl Into<String>,
    ) -> Self {
        let text = text.into();

        Self {
            cursor: text.chars().count(),
            text,
            selection_start: None,
            selection_end: None,
        }
    }

    pub fn set_cursor(
        &mut self,
        cursor: usize,
    ) {
        self.cursor = cursor.min(
            self.text.chars().count(),
        );
    }

    pub fn set_selection(
        &mut self,
        start: usize,
        end: usize,
    ) {
        self.selection_start = Some(start);
        self.selection_end = Some(end);
    }

    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImeEvent {
    Start,
    Update(ImeComposition),
    Commit(String),
    Cancel,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImeState {
    Inactive,
    Composing,
}

#[derive(Debug, Default)]
pub struct ImeManager {
    state: ImeState,
    composition: Option<ImeComposition>,
}

impl ImeManager {
    pub fn new() -> Self {
        Self {
            state: ImeState::Inactive,
            composition: None,
        }
    }

    pub fn handle(
        &mut self,
        event: ImeEvent,
    ) -> Option<String> {
        match event {
            ImeEvent::Start => {
                self.state = ImeState::Composing;
                self.composition = Some(
                    ImeComposition::new(""),
                );

                None
            }

            ImeEvent::Update(composition) => {
                self.state = ImeState::Composing;
                self.composition = Some(composition);

                None
            }

            ImeEvent::Commit(text) => {
                self.state = ImeState::Inactive;
                self.composition = None;

                Some(text)
            }

            ImeEvent::Cancel | ImeEvent::End => {
                self.state = ImeState::Inactive;
                self.composition = None;

                None
            }
        }
    }

    pub fn state(&self) -> ImeState {
        self.state
    }

    pub fn is_composing(&self) -> bool {
        self.state == ImeState::Composing
    }

    pub fn composition(
        &self,
    ) -> Option<&ImeComposition> {
        self.composition.as_ref()
    }

    pub fn clear(&mut self) {
        self.state = ImeState::Inactive;
        self.composition = None;
    }
}
