use std::time::SystemTime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomHistoryEntry {
    pub id: u64,
    pub command: String,
    pub timestamp: SystemTime,
    pub exit_code: Option<i32>,
}

impl CustomHistoryEntry {
    pub fn new<S: Into<String>>(
        id: u64,
        command: S,
    ) -> Self {
        Self {
            id,
            command: command.into(),
            timestamp: SystemTime::now(),
            exit_code: None,
        }
    }

    pub fn set_exit_code(
        &mut self,
        code: i32,
    ) {
        self.exit_code = Some(code);
    }
}

#[derive(Clone, Debug)]
pub struct CustomShellHistory {
    entries: Vec<CustomHistoryEntry>,
    next_id: u64,
    maximum_entries: usize,
}

impl CustomShellHistory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            maximum_entries: 10_000,
        }
    }

    pub fn add<S: Into<String>>(
        &mut self,
        command: S,
    ) -> u64 {
        let id = self.next_id;

        self.next_id += 1;

        self.entries.push(
            CustomHistoryEntry::new(
                id,
                command,
            ),
        );

        self.trim();

        id
    }

    pub fn get(
        &self,
        id: u64,
    ) -> Option<&CustomHistoryEntry> {
        self.entries
            .iter()
            .find(|entry| entry.id == id)
    }

    pub fn get_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut CustomHistoryEntry> {
        self.entries
            .iter_mut()
            .find(|entry| entry.id == id)
    }

    pub fn entries(
        &self,
    ) -> &[CustomHistoryEntry] {
        &self.entries
    }

    pub fn search(
        &self,
        query: &str,
    ) -> Vec<&CustomHistoryEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.command.contains(query)
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn set_maximum_entries(
        &mut self,
        maximum: usize,
    ) {
        self.maximum_entries = maximum;
        self.trim();
    }

    fn trim(&mut self) {
        if self.entries.len()
            > self.maximum_entries
        {
            let excess =
                self.entries.len()
                    - self.maximum_entries;

            self.entries
                .drain(0..excess);
        }
    }
}

impl Default for CustomShellHistory {
    fn default() -> Self {
        Self::new()
    }
}
