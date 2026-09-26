use std::collections::BTreeMap;

/// State of a shell job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JobState {
    Running,
    Stopped,
    Continued,
    Completed,
    Failed,
    Terminated,
}

/// A process/job tracked by the shell integration layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellJob {
    id: u32,
    process_id: u32,
    command: String,
    state: JobState,
    exit_status: Option<i32>,
}

impl ShellJob {
    pub fn new(
        id: u32,
        process_id: u32,
        command: impl Into<String>,
    ) -> Self {
        Self {
            id,
            process_id,
            command: command.into(),
            state: JobState::Running,
            exit_status: None,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn process_id(&self) -> u32 {
        self.process_id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn state(&self) -> JobState {
        self.state
    }

    pub fn exit_status(&self) -> Option<i32> {
        self.exit_status
    }

    pub fn set_state(&mut self, state: JobState) {
        self.state = state;
    }

    pub fn complete(&mut self, status: i32) {
        self.exit_status = Some(status);
        self.state = if status == 0 {
            JobState::Completed
        } else {
            JobState::Failed
        };
    }

    pub fn terminate(&mut self, status: Option<i32>) {
        self.exit_status = status;
        self.state = JobState::Terminated;
    }

    pub fn stop(&mut self) {
        self.state = JobState::Stopped;
    }

    pub fn continue_job(&mut self) {
        self.state = JobState::Continued;
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            JobState::Running | JobState::Stopped | JobState::Continued
        )
    }

    pub fn is_finished(&self) -> bool {
        !self.is_active()
    }
}

/// Tracks jobs belonging to a shell session.
#[derive(Debug, Default)]
pub struct JobController {
    next_id: u32,
    jobs: BTreeMap<u32, ShellJob>,
    foreground: Option<u32>,
}

impl JobController {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            jobs: BTreeMap::new(),
            foreground: None,
        }
    }

    /// Registers a new process as a shell job.
    pub fn register(
        &mut self,
        process_id: u32,
        command: impl Into<String>,
    ) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1).max(1);

        let job = ShellJob::new(id, process_id, command);
        self.jobs.insert(id, job);

        id
    }

    pub fn get(&self, id: u32) -> Option<&ShellJob> {
        self.jobs.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut ShellJob> {
        self.jobs.get_mut(&id)
    }

    pub fn remove(&mut self, id: u32) -> Option<ShellJob> {
        if self.foreground == Some(id) {
            self.foreground = None;
        }

        self.jobs.remove(&id)
    }

    pub fn set_foreground(&mut self, id: Option<u32>) -> bool {
        match id {
            Some(id) if self.jobs.contains_key(&id) => {
                self.foreground = Some(id);
                true
            }
            None => {
                self.foreground = None;
                true
            }
            _ => false,
        }
    }

    pub fn foreground(&self) -> Option<&ShellJob> {
        self.foreground.and_then(|id| self.jobs.get(&id))
    }

    pub fn foreground_id(&self) -> Option<u32> {
        self.foreground
    }

    pub fn mark_stopped(&mut self, id: u32) -> bool {
        if let Some(job) = self.jobs.get_mut(&id) {
            job.stop();

            if self.foreground == Some(id) {
                self.foreground = None;
            }

            true
        } else {
            false
        }
    }

    pub fn mark_continued(&mut self, id: u32) -> bool {
        if let Some(job) = self.jobs.get_mut(&id) {
            job.continue_job();
            true
        } else {
            false
        }
    }

    pub fn mark_finished(&mut self, id: u32, status: i32) -> bool {
        if let Some(job) = self.jobs.get_mut(&id) {
            job.complete(status);

            if self.foreground == Some(id) {
                self.foreground = None;
            }

            true
        } else {
            false
        }
    }

    pub fn mark_terminated(
        &mut self,
        id: u32,
        status: Option<i32>,
    ) -> bool {
        if let Some(job) = self.jobs.get_mut(&id) {
            job.terminate(status);

            if self.foreground == Some(id) {
                self.foreground = None;
            }

            true
        } else {
            false
        }
    }

    pub fn active_jobs(&self) -> impl Iterator<Item = &ShellJob> {
        self.jobs.values().filter(|job| job.is_active())
    }

    pub fn all_jobs(&self) -> impl Iterator<Item = &ShellJob> {
        self.jobs.values()
    }

    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    pub fn clear_finished(&mut self) {
        self.jobs.retain(|_, job| job.is_active());

        if let Some(id) = self.foreground {
            if !self.jobs.contains_key(&id) {
                self.foreground = None;
            }
        }
    }

    pub fn clear(&mut self) {
        self.jobs.clear();
        self.foreground = None;
    }
}
