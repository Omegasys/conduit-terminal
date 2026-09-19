use std::sync::Arc;

use crate::events::EventBus;

use super::profiles::GuiProfileManager;
use super::settings::GuiSettings;
use super::window::GuiWindowManager;
use super::workspaces::GuiWorkspaceManager;

/// Overall state of the GUI application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiApplicationState {
    Created,
    Initializing,
    Running,
    Suspended,
    ShuttingDown,
    Stopped,
}

/// Main GUI application coordinator.
///
/// This layer owns GUI-facing state but intentionally does not perform
/// platform-specific rendering. A frontend backend can use this state
/// to implement GTK, Qt, winit, egui, or another GUI toolkit.
pub struct GuiApplication {
    state: GuiApplicationState,
    event_bus: Arc<EventBus>,
    windows: GuiWindowManager,
    profiles: GuiProfileManager,
    workspaces: GuiWorkspaceManager,
    settings: GuiSettings,
}

impl GuiApplication {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            state: GuiApplicationState::Created,
            event_bus,
            windows: GuiWindowManager::new(),
            profiles: GuiProfileManager::new(),
            workspaces: GuiWorkspaceManager::new(),
            settings: GuiSettings::new(),
        }
    }

    pub fn initialize(&mut self) {
        if self.state != GuiApplicationState::Created {
            return;
        }

        self.state = GuiApplicationState::Initializing;

        self.event_bus.emit(
            crate::events::EventCategory::Application,
            crate::events::EventSource::Gui,
            "gui.initializing",
            crate::events::EventPayload::None,
        );

        self.state = GuiApplicationState::Running;

        self.event_bus.emit(
            crate::events::EventCategory::Application,
            crate::events::EventSource::Gui,
            "gui.ready",
            crate::events::EventPayload::None,
        );
    }

    pub fn suspend(&mut self) {
        if self.state == GuiApplicationState::Running {
            self.state = GuiApplicationState::Suspended;

            self.event_bus.emit(
                crate::events::EventCategory::Application,
                crate::events::EventSource::Gui,
                "gui.suspended",
                crate::events::EventPayload::None,
            );
        }
    }

    pub fn resume(&mut self) {
        if self.state == GuiApplicationState::Suspended {
            self.state = GuiApplicationState::Running;

            self.event_bus.emit(
                crate::events::EventCategory::Application,
                crate::events::EventSource::Gui,
                "gui.resumed",
                crate::events::EventPayload::None,
            );
        }
    }

    pub fn shutdown(&mut self) {
        if matches!(
            self.state,
            GuiApplicationState::ShuttingDown | GuiApplicationState::Stopped
        ) {
            return;
        }

        self.state = GuiApplicationState::ShuttingDown;

        self.event_bus.emit(
            crate::events::EventCategory::Application,
            crate::events::EventSource::Gui,
            "gui.shutting_down",
            crate::events::EventPayload::None,
        );

        self.windows.close_all();
        self.state = GuiApplicationState::Stopped;

        self.event_bus.emit(
            crate::events::EventCategory::Application,
            crate::events::EventSource::Gui,
            "gui.stopped",
            crate::events::EventPayload::None,
        );
    }

    pub fn state(&self) -> GuiApplicationState {
        self.state
    }

    pub fn is_running(&self) -> bool {
        self.state == GuiApplicationState::Running
    }

    pub fn event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
    }

    pub fn windows(&self) -> &GuiWindowManager {
        &self.windows
    }

    pub fn windows_mut(&mut self) -> &mut GuiWindowManager {
        &mut self.windows
    }

    pub fn profiles(&self) -> &GuiProfileManager {
        &self.profiles
    }

    pub fn profiles_mut(&mut self) -> &mut GuiProfileManager {
        &mut self.profiles
    }

    pub fn workspaces(&self) -> &GuiWorkspaceManager {
        &self.workspaces
    }

    pub fn workspaces_mut(&mut self) -> &mut GuiWorkspaceManager {
        &mut self.workspaces
    }

    pub fn settings(&self) -> &GuiSettings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut GuiSettings {
        &mut self.settings
    }
}
