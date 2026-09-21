#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionPoint {
    pub row: usize,
    pub column: usize,
}

impl SelectionPoint {
    pub fn new(
        row: usize,
        column: usize,
    ) -> Self {
        Self { row, column }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Character,
    Word,
    Line,
    Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub start: SelectionPoint,
    pub end: SelectionPoint,
    pub mode: SelectionMode,
}

impl Selection {
    pub fn new(
        start: SelectionPoint,
        end: SelectionPoint,
        mode: SelectionMode,
    ) -> Self {
        Self {
            start,
            end,
            mode,
        }
    }

    pub fn normalized(
        &self,
    ) -> (SelectionPoint, SelectionPoint) {
        if self.start.row < self.end.row
            || (self.start.row == self.end.row
                && self.start.column <= self.end.column)
        {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        }
    }

    pub fn contains(
        &self,
        point: SelectionPoint,
    ) -> bool {
        let (start, end) = self.normalized();

        if point.row < start.row
            || point.row > end.row
        {
            return false;
        }

        if start.row == end.row {
            return point.column >= start.column
                && point.column <= end.column;
        }

        if point.row == start.row {
            return point.column >= start.column;
        }

        if point.row == end.row {
            return point.column <= end.column;
        }

        true
    }
}

#[derive(Debug, Default)]
pub struct SelectionState {
    active: Option<Selection>,
    selecting: bool,
}

impl SelectionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin(
        &mut self,
        point: SelectionPoint,
        mode: SelectionMode,
    ) {
        self.active = Some(
            Selection::new(
                point,
                point,
                mode,
            ),
        );

        self.selecting = true;
    }

    pub fn update(
        &mut self,
        point: SelectionPoint,
    ) {
        if let Some(selection) =
            self.active.as_mut()
        {
            selection.end = point;
        }
    }

    pub fn finish(&mut self) {
        self.selecting = false;
    }

    pub fn cancel(&mut self) {
        self.active = None;
        self.selecting = false;
    }

    pub fn selection(
        &self,
    ) -> Option<&Selection> {
        self.active.as_ref()
    }

    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }

    pub fn is_selecting(&self) -> bool {
        self.selecting
    }

    pub fn clear(&mut self) {
        self.cancel();
    }
}
