//! Terminal screen state.
//!
//! The screen is a logical representation of what the terminal should
//! display. Rendering backends consume this state without modifying it.

/// A single terminal cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScreenCell {
    pub character: char,
    pub foreground: u32,
    pub background: u32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Default for ScreenCell {
    fn default() -> Self {
        Self {
            character: ' ',
            foreground: 0xFFFFFF,
            background: 0x000000,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

/// Terminal screen buffer.
pub struct Screen {
    rows: usize,
    columns: usize,
    cells: Vec<ScreenCell>,

    cursor_row: usize,
    cursor_column: usize,

    scroll_top: usize,
    scroll_bottom: usize,

    saved_cursor: Option<(usize, usize)>,

    current_cell: ScreenCell,
}

impl Screen {
    /// Creates a new blank screen.
    pub fn new(rows: usize, columns: usize) -> Self {
        let rows = rows.max(1);
        let columns = columns.max(1);

        let size = rows * columns;

        Self {
            rows,
            columns,
            cells: vec![ScreenCell::default(); size],

            cursor_row: 0,
            cursor_column: 0,

            scroll_top: 0,
            scroll_bottom: rows - 1,

            saved_cursor: None,

            current_cell: ScreenCell::default(),
        }
    }

    /// Returns the number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the cursor position.
    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_row, self.cursor_column)
    }

    /// Returns a cell.
    pub fn cell(&self, row: usize, column: usize) -> Option<&ScreenCell> {
        if row >= self.rows || column >= self.columns {
            return None;
        }

        Some(&self.cells[row * self.columns + column])
    }

    /// Resizes the screen.
    pub fn resize(&mut self, rows: usize, columns: usize) {
        let rows = rows.max(1);
        let columns = columns.max(1);

        let mut new_cells = vec![ScreenCell::default(); rows * columns];

        let copy_rows = self.rows.min(rows);
        let copy_columns = self.columns.min(columns);

        for row in 0..copy_rows {
            for column in 0..copy_columns {
                let old_index = row * self.columns + column;
                let new_index = row * columns + column;

                new_cells[new_index] = self.cells[old_index].clone();
            }
        }

        self.rows = rows;
        self.columns = columns;
        self.cells = new_cells;

        self.scroll_top = 0;
        self.scroll_bottom = rows - 1;

        self.cursor_row = self.cursor_row.min(rows - 1);
        self.cursor_column = self.cursor_column.min(columns - 1);
    }

    /// Processes raw terminal output.
    ///
    /// This is a deliberately small bootstrap parser. The dedicated
    /// terminal protocol subsystem will eventually perform complete
    /// ANSI/VT/OSC/CSI parsing and call the screen API.
    pub fn process_bytes(&mut self, data: &[u8]) {
        let mut index = 0;

        while index < data.len() {
            match data[index] {
                0x1B => {
                    index += 1;
                    self.process_escape_sequence(data, &mut index);
                }

                b'\n' => {
                    self.line_feed();
                    index += 1;
                }

                b'\r' => {
                    self.cursor_column = 0;
                    index += 1;
                }

                0x08 => {
                    self.cursor_column = self.cursor_column.saturating_sub(1);
                    index += 1;
                }

                b'\t' => {
                    self.tab();
                    index += 1;
                }

                0x07 => {
                    // BEL is handled by the higher-level event system.
                    index += 1;
                }

                byte if byte >= 0x20 => {
                    self.put_character(byte as char);
                    index += 1;
                }

                _ => {
                    index += 1;
                }
            }
        }
    }

    /// Writes a character at the current cursor.
    pub fn put_character(&mut self, character: char) {
        if self.cursor_column >= self.columns {
            self.line_feed();
            self.cursor_column = 0;
        }

        let index = self.cursor_row * self.columns + self.cursor_column;

        self.cells[index] = ScreenCell {
            character,
            ..self.current_cell.clone()
        };

        self.cursor_column += 1;

        if self.cursor_column >= self.columns {
            self.cursor_column = self.columns;
            self.line_feed();
        }
    }

    /// Moves the cursor to a specific position.
    pub fn move_cursor(&mut self, row: usize, column: usize) {
        self.cursor_row = row.min(self.rows - 1);
        self.cursor_column = column.min(self.columns - 1);
    }

    /// Clears the entire screen.
    pub fn clear(&mut self) {
        self.cells.fill(ScreenCell::default());
        self.cursor_row = 0;
        self.cursor_column = 0;
    }

    /// Clears from the cursor to the end of the screen.
    pub fn erase_to_end_of_screen(&mut self) {
        let start = self.cursor_row * self.columns + self.cursor_column;

        for cell in self.cells.iter_mut().skip(start) {
            *cell = ScreenCell::default();
        }
    }

    /// Clears from the cursor to the end of the current line.
    pub fn erase_to_end_of_line(&mut self) {
        let start = self.cursor_row * self.columns + self.cursor_column;
        let end = (self.cursor_row + 1) * self.columns;

        for cell in self.cells[start..end].iter_mut() {
            *cell = ScreenCell::default();
        }
    }

    /// Saves the current cursor position.
    pub fn save_cursor(&mut self) {
        self.saved_cursor = Some((self.cursor_row, self.cursor_column));
    }

    /// Restores the previously saved cursor position.
    pub fn restore_cursor(&mut self) {
        if let Some((row, column)) = self.saved_cursor {
            self.cursor_row = row.min(self.rows - 1);
            self.cursor_column = column.min(self.columns - 1);
        }
    }

    fn line_feed(&mut self) {
        if self.cursor_row >= self.scroll_bottom {
            self.scroll_up();
        } else {
            self.cursor_row += 1;
        }
    }

    fn scroll_up(&mut self) {
        for row in self.scroll_top..self.scroll_bottom {
            for column in 0..self.columns {
                let source = (row + 1) * self.columns + column;
                let destination = row * self.columns + column;

                self.cells[destination] = self.cells[source].clone();
            }
        }

        let final_row = self.scroll_bottom * self.columns;

        for column in 0..self.columns {
            self.cells[final_row + column] = ScreenCell::default();
        }
    }

    fn tab(&mut self) {
        let next_tab = ((self.cursor_column / 8) + 1) * 8;

        self.cursor_column = next_tab.min(self.columns - 1);
    }

    fn process_escape_sequence(&mut self, data: &[u8], index: &mut usize) {
        if *index >= data.len() {
            return;
        }

        match data[*index] {
            b'[' => {
                *index += 1;
                self.process_csi(data, index);
            }

            b'7' => {
                self.save_cursor();
                *index += 1;
            }

            b'8' => {
                self.restore_cursor();
                *index += 1;
            }

            b'c' => {
                self.clear();
                *index += 1;
            }

            _ => {
                *index += 1;
            }
        }
    }

    fn process_csi(&mut self, data: &[u8], index: &mut usize) {
        let mut parameters = Vec::new();
        let mut current = String::new();

        while *index < data.len() {
            let byte = data[*index];

            if byte.is_ascii_digit() {
                current.push(byte as char);
                *index += 1;
                continue;
            }

            if byte == b';' {
                parameters.push(parse_parameter(&current));
                current.clear();
                *index += 1;
                continue;
            }

            if byte == b'?' || byte == b'>' {
                *index += 1;
                continue;
            }

            if (0x40..=0x7E).contains(&byte) {
                if !current.is_empty() || !parameters.is_empty() {
                    parameters.push(parse_parameter(&current));
                }

                *index += 1;

                self.execute_csi(byte as char, &parameters);

                return;
            }

            *index += 1;
        }
    }

    fn execute_csi(&mut self, command: char, parameters: &[usize]) {
        let first = parameters.first().copied().unwrap_or(1);

        match command {
            'A' => {
                self.cursor_row = self
                    .cursor_row
                    .saturating_sub(first);
            }

            'B' => {
                self.cursor_row = (self.cursor_row + first)
                    .min(self.rows - 1);
            }

            'C' => {
                self.cursor_column = (self.cursor_column + first)
                    .min(self.columns - 1);
            }

            'D' => {
                self.cursor_column = self
                    .cursor_column
                    .saturating_sub(first);
            }

            'E' => {
                self.cursor_row = (self.cursor_row + first)
                    .min(self.rows - 1);

                self.cursor_column = 0;
            }

            'F' => {
                self.cursor_row = self
                    .cursor_row
                    .saturating_sub(first);

                self.cursor_column = 0;
            }

            'G' | '`' => {
                self.cursor_column = first
                    .saturating_sub(1)
                    .min(self.columns - 1);
            }

            'H' | 'f' => {
                let row = parameters
                    .first()
                    .copied()
                    .unwrap_or(1)
                    .saturating_sub(1);

                let column = parameters
                    .get(1)
                    .copied()
                    .unwrap_or(1)
                    .saturating_sub(1);

                self.move_cursor(row, column);
            }

            'J' => {
                match first {
                    0 => self.erase_to_end_of_screen(),
                    2 | 3 => self.clear(),
                    _ => {}
                }
            }

            'K' => {
                match first {
                    0 => self.erase_to_end_of_line(),

                    2 => {
                        let start = self.cursor_row * self.columns;
                        let end = start + self.columns;

                        for cell in self.cells[start..end].iter_mut() {
                            *cell = ScreenCell::default();
                        }
                    }

                    _ => {}
                }
            }

            's' => self.save_cursor(),

            'u' => self.restore_cursor(),

            _ => {}
        }
    }
}

fn parse_parameter(value: &str) -> usize {
    if value.is_empty() {
        1
    } else {
        value.parse().unwrap_or(1)
    }
}
