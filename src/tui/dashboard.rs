use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardPanel {
    Overview,
    Sessions,
    Resources,
    Processes,
    Events,
    Security,
    Performance,
}

impl DashboardPanel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Sessions => "Sessions",
            Self::Resources => "Resources",
            Self::Processes => "Processes",
            Self::Events => "Events",
            Self::Security => "Security",
            Self::Performance => "Performance",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardView {
    Full,
    Compact,
    Minimal,
}

impl DashboardView {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Full => "Full",
            Self::Compact => "Compact",
            Self::Minimal => "Minimal",
        }
    }
}

pub struct DashboardState {
    visible: bool,
    view: DashboardView,
    active_panel: DashboardPanel,
    panels: Vec<DashboardPanel>,

    terminal_count: usize,
    window_count: usize,
    tab_count: usize,
    pane_count: usize,
    process_count: usize,
    resource_count: usize,

    cpu_usage: f32,
    memory_usage: u64,
    events_processed: u64,

    last_update: Option<Instant>,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self::new()
    }
}

impl DashboardState {
    pub fn new() -> Self {
        Self {
            visible: true,
            view: DashboardView::Full,
            active_panel: DashboardPanel::Overview,
            panels: vec![
                DashboardPanel::Overview,
                DashboardPanel::Sessions,
                DashboardPanel::Resources,
                DashboardPanel::Processes,
                DashboardPanel::Events,
                DashboardPanel::Security,
                DashboardPanel::Performance,
            ],
            terminal_count: 0,
            window_count: 0,
            tab_count: 0,
            pane_count: 0,
            process_count: 0,
            resource_count: 0,
            cpu_usage: 0.0,
            memory_usage: 0,
            events_processed: 0,
            last_update: None,
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn view(&self) -> DashboardView {
        self.view
    }

    pub fn set_view(&mut self, view: DashboardView) {
        self.view = view;
    }

    pub fn active_panel(&self) -> DashboardPanel {
        self.active_panel
    }

    pub fn set_active_panel(&mut self, panel: DashboardPanel) {
        if self.panels.contains(&panel) {
            self.active_panel = panel;
        }
    }

    pub fn next_panel(&mut self) {
        if self.panels.is_empty() {
            return;
        }

        let index = self
            .panels
            .iter()
            .position(|panel| *panel == self.active_panel)
            .unwrap_or(0);

        let next = (index + 1) % self.panels.len();
        self.active_panel = self.panels[next];
    }

    pub fn previous_panel(&mut self) {
        if self.panels.is_empty() {
            return;
        }

        let index = self
            .panels
            .iter()
            .position(|panel| *panel == self.active_panel)
            .unwrap_or(0);

        let previous = if index == 0 {
            self.panels.len() - 1
        } else {
            index - 1
        };

        self.active_panel = self.panels[previous];
    }

    pub fn panels(&self) -> &[DashboardPanel] {
        &self.panels
    }

    pub fn terminal_count(&self) -> usize {
        self.terminal_count
    }

    pub fn window_count(&self) -> usize {
        self.window_count
    }

    pub fn tab_count(&self) -> usize {
        self.tab_count
    }

    pub fn pane_count(&self) -> usize {
        self.pane_count
    }

    pub fn process_count(&self) -> usize {
        self.process_count
    }

    pub fn resource_count(&self) -> usize {
        self.resource_count
    }

    pub fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    pub fn memory_usage(&self) -> u64 {
        self.memory_usage
    }

    pub fn events_processed(&self) -> u64 {
        self.events_processed
    }

    pub fn update_counts(
        &mut self,
        terminal_count: usize,
        window_count: usize,
        tab_count: usize,
        pane_count: usize,
        process_count: usize,
        resource_count: usize,
    ) {
        self.terminal_count = terminal_count;
        self.window_count = window_count;
        self.tab_count = tab_count;
        self.pane_count = pane_count;
        self.process_count = process_count;
        self.resource_count = resource_count;
        self.last_update = Some(Instant::now());
    }

    pub fn update_performance(&mut self, cpu_usage: f32, memory_usage: u64) {
        self.cpu_usage = cpu_usage.max(0.0);
        self.memory_usage = memory_usage;
        self.last_update = Some(Instant::now());
    }

    pub fn record_events(&mut self, count: u64) {
        self.events_processed = self.events_processed.saturating_add(count);
        self.last_update = Some(Instant::now());
    }

    pub fn last_update(&self) -> Option<Instant> {
        self.last_update
    }
}
