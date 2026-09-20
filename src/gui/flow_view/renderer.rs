use super::panes::{FlowPaneId, PaneInfo, PaneState};
use super::pty::{PtySession, PtyState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Normal,
    Debug,
    Minimal,
}

impl Default for RenderMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderLayer {
    Background,
    Terminal,
    Cursor,
    Selection,
    Overlay,
    Debug,
}

#[derive(Debug, Clone)]
pub struct RenderCell {
    pub character: char,
    pub row: usize,
    pub column: usize,
    pub selected: bool,
    pub bold: bool,
    pub dim: bool,
}

impl RenderCell {
    pub fn new(character: char, row: usize, column: usize) -> Self {
        Self {
            character,
            row,
            column,
            selected: false,
            bold: false,
            dim: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaneRenderState {
    pane_id: FlowPaneId,
    title: String,
    state: PaneState,
    pty_state: Option<PtyState>,
    columns: u16,
    rows: u16,
    cursor_row: usize,
    cursor_column: usize,
    cells: Vec<RenderCell>,
}

impl PaneRenderState {
    pub fn new(
        pane: &PaneInfo,
        pty: Option<&PtySession>,
    ) -> Self {
        let (columns, rows, pty_state) = match pty {
            Some(session) => (
                session.size().columns,
                session.size().rows,
                Some(session.state()),
            ),
            None => (80, 24, None),
        };

        Self {
            pane_id: pane.id(),
            title: pane.title().to_string(),
            state: pane.state(),
            pty_state,
            columns,
            rows,
            cursor_row: 0,
            cursor_column: 0,
            cells: Vec::new(),
        }
    }

    pub fn pane_id(&self) -> FlowPaneId {
        self.pane_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn state(&self) -> PaneState {
        self.state
    }

    pub fn pty_state(&self) -> Option<PtyState> {
        self.pty_state
    }

    pub fn columns(&self) -> u16 {
        self.columns
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_row, self.cursor_column)
    }

    pub fn set_cursor_position(&mut self, row: usize, column: usize) {
        self.cursor_row = row;
        self.cursor_column = column;
    }

    pub fn cells(&self) -> &[RenderCell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: Vec<RenderCell>) {
        self.cells = cells;
    }
}

#[derive(Debug)]
pub struct TerminalRenderer {
    mode: RenderMode,
    antialiasing: bool,
    cursor_visible: bool,
    damage_tracking: bool,
    frame_number: u64,
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self {
            mode: RenderMode::Normal,
            antialiasing: true,
            cursor_visible: true,
            damage_tracking: true,
            frame_number: 0,
        }
    }
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mode(&self) -> RenderMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: RenderMode) {
        self.mode = mode;
    }

    pub fn antialiasing(&self) -> bool {
        self.antialiasing
    }

    pub fn set_antialiasing(&mut self, enabled: bool) {
        self.antialiasing = enabled;
    }

    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.cursor_visible = visible;
    }

    pub fn damage_tracking(&self) -> bool {
        self.damage_tracking
    }

    pub fn set_damage_tracking(&mut self, enabled: bool) {
        self.damage_tracking = enabled;
    }

    pub fn frame_number(&self) -> u64 {
        self.frame_number
    }

    pub fn begin_frame(&mut self) {
        self.frame_number = self.frame_number.saturating_add(1);
    }

    pub fn prepare_pane(
        &self,
        pane: &PaneInfo,
        pty: Option<&PtySession>,
    ) -> PaneRenderState {
        PaneRenderState::new(pane, pty)
    }

    pub fn render_text(
        &self,
        state: &mut PaneRenderState,
        text: &str,
    ) {
        state.set_cells(
            text.chars()
                .enumerate()
                .map(|(column, character)| {
                    RenderCell::new(character, 0, column)
                })
                .collect(),
        );
    }

    pub fn clear(&mut self) {
        self.frame_number = 0;
    }
}
