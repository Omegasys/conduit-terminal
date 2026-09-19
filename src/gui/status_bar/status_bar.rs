use super::indicators::{
    IndicatorKind,
    StatusIndicator,
    StatusIndicatorManager,
    StatusIndicatorState,
};
use super::segments::{
    StatusSegment,
    StatusSegmentAlignment,
    StatusSegmentKind,
    StatusSegmentManager,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusBarPosition {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusBarLayout {
    SingleRow,
    Compact,
    Expanded,
}

#[derive(Debug)]
pub struct StatusBar {
    id: String,
    title: String,
    position: StatusBarPosition,
    layout: StatusBarLayout,
    visible: bool,
    enabled: bool,
    resizable: bool,
    segments: StatusSegmentManager,
    indicators: StatusIndicatorManager,
}

impl StatusBar {
    pub fn new(id: impl Into<String>) -> Self {
        let mut status_bar = Self {
            id: id.into(),
            title: "Status Bar".to_string(),
            position: StatusBarPosition::Bottom,
            layout: StatusBarLayout::SingleRow,
            visible: true,
            enabled: true,
            resizable: false,
            segments: StatusSegmentManager::new(),
            indicators: StatusIndicatorManager::new(),
        };

        status_bar.initialize_defaults();
        status_bar
    }

    fn initialize_defaults(&mut self) {
        let mut mode = StatusSegment::new("mode", "Mode", "Terminal");
        mode.set_priority(0);

        let mut shell = StatusSegment::new("shell", "Shell", "Unknown");
        shell.set_priority(10);

        let mut cwd = StatusSegment::new("cwd", "Directory", "~");
        cwd.set_priority(20);

        let mut command = StatusSegment::new("command", "Command", "");
        command.set_priority(30);

        let mut encoding = StatusSegment::new("encoding", "Encoding", "UTF-8");
        encoding.set_priority(40);

        let mut terminal = StatusSegment::new("terminal-size", "Terminal Size", "0 × 0");
        terminal.set_alignment(StatusSegmentAlignment::Right);
        terminal.set_priority(50);

        self.segments.add(mode);
        self.segments.add(shell);
        self.segments.add(cwd);
        self.segments.add(command);
        self.segments.add(encoding);
        self.segments.add(terminal);

        let connection =
            StatusIndicator::new("connection", "Connection", IndicatorKind::Connection);

        let session =
            StatusIndicator::new("session", "Session", IndicatorKind::Session);

        let security =
            StatusIndicator::new("security", "Security", IndicatorKind::Security);

        let recording =
            StatusIndicator::new("recording", "Recording", IndicatorKind::Recording);

        let synchronization = StatusIndicator::new(
            "synchronization",
            "Synchronization",
            IndicatorKind::Synchronization,
        );

        self.indicators.add(connection);
        self.indicators.add(session);
        self.indicators.add(security);
        self.indicators.add(recording);
        self.indicators.add(synchronization);
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn position(&self) -> StatusBarPosition {
        self.position
    }

    pub fn layout(&self) -> StatusBarLayout {
        self.layout
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_resizable(&self) -> bool {
        self.resizable
    }

    pub fn segments(&self) -> &StatusSegmentManager {
        &self.segments
    }

    pub fn segments_mut(&mut self) -> &mut StatusSegmentManager {
        &mut self.segments
    }

    pub fn indicators(&self) -> &StatusIndicatorManager {
        &self.indicators
    }

    pub fn indicators_mut(&mut self) -> &mut StatusIndicatorManager {
        &mut self.indicators
    }

    pub fn add_segment(&mut self, segment: StatusSegment) {
        self.segments.add(segment);
    }

    pub fn remove_segment(&mut self, id: &str) -> Option<StatusSegment> {
        self.segments.remove(id)
    }

    pub fn add_indicator(&mut self, indicator: StatusIndicator) {
        self.indicators.add(indicator);
    }

    pub fn remove_indicator(&mut self, id: &str) -> Option<StatusIndicator> {
        self.indicators.remove(id)
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_position(&mut self, position: StatusBarPosition) {
        self.position = position;
    }

    pub fn set_layout(&mut self, layout: StatusBarLayout) {
        self.layout = layout;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        for segment in self.segments.iter_mut() {
            segment.set_enabled(enabled);
        }

        for indicator in self.indicators.iter_mut() {
            indicator.set_enabled(enabled);
        }
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }

    pub fn update_segment(
        &mut self,
        id: &str,
        value: impl Into<String>,
    ) -> bool {
        if let Some(segment) = self.segments.get_mut(id) {
            segment.set_value(value);
            true
        } else {
            false
        }
    }

    pub fn update_indicator(
        &mut self,
        id: &str,
        state: StatusIndicatorState,
    ) -> bool {
        if let Some(indicator) = self.indicators.get_mut(id) {
            indicator.set_state(state);
            true
        } else {
            false
        }
    }

    pub fn set_connection_state(
        &mut self,
        state: StatusIndicatorState,
    ) -> bool {
        self.update_indicator("connection", state)
    }

    pub fn set_session_state(
        &mut self,
        state: StatusIndicatorState,
    ) -> bool {
        self.update_indicator("session", state)
    }

    pub fn set_security_state(
        &mut self,
        state: StatusIndicatorState,
    ) -> bool {
        self.update_indicator("security", state)
    }

    pub fn set_recording_state(
        &mut self,
        state: StatusIndicatorState,
    ) -> bool {
        self.update_indicator("recording", state)
    }

    pub fn set_synchronization_state(
        &mut self,
        state: StatusIndicatorState,
    ) -> bool {
        self.update_indicator("synchronization", state)
    }

    pub fn set_shell(&mut self, shell: impl Into<String>) -> bool {
        self.update_segment("shell", shell)
    }

    pub fn set_current_directory(
        &mut self,
        directory: impl Into<String>,
    ) -> bool {
        self.update_segment("cwd", directory)
    }

    pub fn set_command(&mut self, command: impl Into<String>) -> bool {
        self.update_segment("command", command)
    }

    pub fn set_terminal_size(&mut self, columns: u16, rows: u16) -> bool {
        self.update_segment(
            "terminal-size",
            format!("{columns} × {rows}"),
        )
    }

    pub fn visible_segments(&self) -> Vec<&StatusSegment> {
        self.segments.visible()
    }

    pub fn visible_indicators(&self) -> Vec<&StatusIndicator> {
        self.indicators.visible()
    }

    pub fn clear_segments(&mut self) {
        self.segments.clear();
    }

    pub fn clear_indicators(&mut self) {
        self.indicators.clear();
    }

    pub fn reset_defaults(&mut self) {
        self.segments.clear();
        self.indicators.clear();
        self.initialize_defaults();
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    pub fn indicator_count(&self) -> usize {
        self.indicators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.segment_count() == 0 && self.indicator_count() == 0
    }

    pub fn supports_segment_kind(&self, kind: StatusSegmentKind) -> bool {
        self.segments
            .iter()
            .any(|segment| segment.kind() == kind)
    }
}
