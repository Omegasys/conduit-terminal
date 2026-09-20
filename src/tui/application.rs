use std::time::{Duration, Instant};

use super::dashboard::DashboardState;
use super::settings::SettingsState;
use super::tabs::TabList;
use super::windows::TuiWindowManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiExitReason {
    UserRequested,
    QuitCommand,
    Signal,
    Error,
    ApplicationShutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiApplicationState {
    Starting,
    Running,
    Paused,
    Stopping,
    Stopped,
    Failed,
}

pub struct TuiApplication {
    state: TuiApplicationState,
    started_at: Option<Instant>,
    stopped_at: Option<Instant>,
    exit_reason: Option<TuiExitReason>,

    dashboard: DashboardState,
    settings: SettingsState,
    windows: TuiWindowManager,
    tabs: TabList,

    tick_interval: Duration,
    tick_count: u64,
    redraw_requested: bool,
}

impl Default for TuiApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiApplication {
    pub fn new() -> Self {
        Self {
            state: TuiApplicationState::Starting,
            started_at: None,
            stopped_at: None,
            exit_reason: None,
            dashboard: DashboardState::new(),
            settings: SettingsState::new(),
            windows: TuiWindowManager::new(),
            tabs: TabList::new(),
            tick_interval: Duration::from_millis(16),
            tick_count: 0,
            redraw_requested: true,
        }
    }

    pub fn start(&mut self) {
        if matches!(
            self.state,
            TuiApplicationState::Running | TuiApplicationState::Starting
        ) {
            self.state = TuiApplicationState::Running;
            self.started_at = Some(Instant::now());
            self.stopped_at = None;
            self.exit_reason = None;
            self.request_redraw();
        }
    }

    pub fn pause(&mut self) {
        if self.state == TuiApplicationState::Running {
            self.state = TuiApplicationState::Paused;
            self.request_redraw();
        }
    }

    pub fn resume(&mut self) {
        if self.state == TuiApplicationState::Paused {
            self.state = TuiApplicationState::Running;
            self.request_redraw();
        }
    }

    pub fn stop(&mut self, reason: TuiExitReason) {
        if self.state == TuiApplicationState::Stopped {
            return;
        }

        self.state = TuiApplicationState::Stopping;
        self.exit_reason = Some(reason);
        self.stopped_at = Some(Instant::now());
        self.state = TuiApplicationState::Stopped;
        self.request_redraw();
    }

    pub fn fail(&mut self) {
        self.state = TuiApplicationState::Failed;
        self.exit_reason = Some(TuiExitReason::Error);
        self.stopped_at = Some(Instant::now());
        self.request_redraw();
    }

    pub fn tick(&mut self) {
        if self.state != TuiApplicationState::Running {
            return;
        }

        self.tick_count = self.tick_count.saturating_add(1);
        self.request_redraw();
    }

    pub fn state(&self) -> TuiApplicationState {
        self.state
    }

    pub fn is_running(&self) -> bool {
        self.state == TuiApplicationState::Running
    }

    pub fn is_stopped(&self) -> bool {
        matches!(
            self.state,
            TuiApplicationState::Stopped | TuiApplicationState::Failed
        )
    }

    pub fn started_at(&self) -> Option<Instant> {
        self.started_at
    }

    pub fn stopped_at(&self) -> Option<Instant> {
        self.stopped_at
    }

    pub fn exit_reason(&self) -> Option<TuiExitReason> {
        self.exit_reason
    }

    pub fn uptime(&self) -> Option<Duration> {
        self.started_at.map(|started| {
            let end = self.stopped_at.unwrap_or_else(Instant::now);
            end.saturating_duration_since(started)
        })
    }

    pub fn tick_interval(&self) -> Duration {
        self.tick_interval
    }

    pub fn set_tick_interval(&mut self, interval: Duration) {
        self.tick_interval = interval.max(Duration::from_millis(1));
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn redraw_requested(&self) -> bool {
        self.redraw_requested
    }

    pub fn request_redraw(&mut self) {
        self.redraw_requested = true;
    }

    pub fn clear_redraw_request(&mut self) {
        self.redraw_requested = false;
    }

    pub fn dashboard(&self) -> &DashboardState {
        &self.dashboard
    }

    pub fn dashboard_mut(&mut self) -> &mut DashboardState {
        &mut self.dashboard
    }

    pub fn settings(&self) -> &SettingsState {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut SettingsState {
        &mut self.settings
    }

    pub fn windows(&self) -> &TuiWindowManager {
        &self.windows
    }

    pub fn windows_mut(&mut self) -> &mut TuiWindowManager {
        &mut self.windows
    }

    pub fn tabs(&self) -> &TabList {
        &self.tabs
    }

    pub fn tabs_mut(&mut self) -> &mut TabList {
        &mut self.tabs
    }
}
