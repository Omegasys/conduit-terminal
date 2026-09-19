//! Terminal cursor state and behavior.

/// Cursor visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorVisibility {
    Visible,
    Hidden,
}

/// Cursor shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    Block,
    Underline,
    Bar,
}

/// Terminal cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    row: usize,
    column: usize,
    visibility: CursorVisibility,
    shape: CursorShape,
    blinking: bool,

    saved_row: usize,
    saved_column: usize,
}

impl Cursor {
    /// Creates a cursor at the upper-left corner.
    pub fn new() -> Self {
        Self {
            row: 0,
            column: 0,
            visibility: CursorVisibility::Visible,
            shape: CursorShape::Block,
            blinking: true,
            saved_row: 0,
            saved_column: 0,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn position(&self) -> (usize, usize) {
        (self.row, self.column)
    }

    pub fn visibility(&self) -> CursorVisibility {
        self.visibility
    }

    pub fn shape(&self) -> CursorShape {
        self.shape
    }

    pub fn blinking(&self) -> bool {
        self.blinking
    }

    pub fn set_position(&mut self, row: usize, column: usize) {
        self.row = row;
        self.column = column;
    }

    pub fn set_visibility(&mut self, visibility: CursorVisibility) {
        self.visibility = visibility;
    }

    pub fn set_shape(&mut self, shape: CursorShape) {
        self.shape = shape;
    }

    pub fn set_blinking(&mut self, blinking: bool) {
        self.blinking = blinking;
    }

    pub fn move_up(&mut self, amount: usize) {
        self.row = self.row.saturating_sub(amount);
    }

    pub fn move_down(&mut self, amount: usize, max_rows: usize) {
        if max_rows == 0 {
            self.row = 0;
            return;
        }

        self.row = self
            .row
            .saturating_add(amount)
            .min(max_rows - 1);
    }

    pub fn move_left(&mut self, amount: usize) {
        self.column = self.column.saturating_sub(amount);
    }

    pub fn move_right(&mut self, amount: usize, max_columns: usize) {
        if max_columns == 0 {
            self.column = 0;
            return;
        }

        self.column = self
            .column
            .saturating_add(amount)
            .min(max_columns - 1);
    }

    pub fn home(&mut self) {
        self.row = 0;
        self.column = 0;
    }

    pub fn save(&mut self) {
        self.saved_row = self.row;
        self.saved_column = self.column;
    }

    pub fn restore(&mut self) {
        self.row = self.saved_row;
        self.column = self.saved_column;
    }

    pub fn clamp(&mut self, rows: usize, columns: usize) {
        if rows == 0 || columns == 0 {
            self.row = 0;
            self.column = 0;
            return;
        }

        self.row = self.row.min(rows - 1);
        self.column = self.column.min(columns - 1);
    }

    pub fn reset(&mut self) {
        self.row = 0;
        self.column = 0;
        self.visibility = CursorVisibility::Visible;
        self.shape = CursorShape::Block;
        self.blinking = true;
        self.saved_row = 0;
        self.saved_column = 0;
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}
