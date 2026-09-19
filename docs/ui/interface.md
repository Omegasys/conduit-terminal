# Conduit User Interface

## Overview

Conduit provides a unified user interface architecture built around a shared terminal core.

The primary graphical interface is a traditional desktop application with menus, optional toolbars, tabs, panes, a sidebar, status information, and terminal views.

Conduit also provides TUI and CLI interfaces that expose the same underlying functionality.

The interface should be:

* Familiar by default.
* Highly configurable.
* Keyboard accessible.
* Fast.
* Modular.
* Consistent across GUI, TUI, and CLI.
* Compatible with live configuration changes.

## Interface Philosophy

Conduit follows the principle:

> Simple by default, powerful when opened up, configurable everywhere.

The default interface should not overwhelm users with every available feature.

Advanced functionality should remain accessible through:

* Menus.
* Command Palette.
* Keyboard shortcuts.
* Context menus.
* Sidebar tools.
* Settings.
* CLI commands.
* TUI commands.

## Main Window

The default desktop layout is:

```text
┌─────────────────────────────────────────────────────────────┐
│ File Edit View Terminal Tabs Panes Session Tools Help       │
├─────────────────────────────────────────────────────────────┤
│ Toolbar / Quick Actions                                     │
├──────────────┬──────────────────────────────────────────────┤
│              │ [Terminal 1] [Terminal 2] [+]               │
│   Sidebar    ├──────────────────────────────────────────────┤
│              │                                              │
│              │                 Terminal                     │
│              │                                              │
│              │                                              │
│              │                                              │
├──────────────┴──────────────────────────────────────────────┤
│ Status / Session / Connection / Resource Information        │
└─────────────────────────────────────────────────────────────┘
```

Individual elements should be configurable.

## Menu Bar

The standard menu bar contains:

* File
* Edit
* View
* Terminal
* Tabs
* Panes
* Session
* Tools
* Help

Menus should expose the same commands available through the Command Palette.

## Toolbar

The toolbar is optional.

Users should be able to:

* Enable or disable it.
* Change its position where supported.
* Choose visible actions.
* Use compact or expanded layouts.
* Reset it to defaults.

## Sidebar

The sidebar provides access to contextual information and management tools.

It may contain:

* Workspaces.
* Sessions.
* Connections.
* Profiles.
* Themes.
* Plugins.
* History.
* Diagnostics.
* Files.
* Flow View.

The sidebar should be collapsible.

## Terminal Area

The terminal area contains terminal panes and tabs.

Users should be able to:

* Create tabs.
* Split panes.
* Resize panes.
* Move panes.
* Reorder tabs.
* Detach windows.
* Restore sessions.
* Rename sessions.
* Switch profiles.
* Change themes.

## Tabs

Tabs represent terminal views or sessions according to the selected configuration.

Each tab may display:

* Session name.
* Shell.
* Connection type.
* Activity indicator.
* Exit status.
* Progress indicator.
* Security state.

## Panes

Panes allow multiple terminal views within one tab.

Supported layouts may include:

* Horizontal splits.
* Vertical splits.
* Nested splits.
* Resizable panes.
* Collapsible pane groups.

## Status Bar

The status bar can display:

* Current shell.
* Session name.
* Host.
* Working directory.
* Terminal dimensions.
* Security profile.
* Connection state.
* Recording state.
* Active protocol information.

Users should be able to configure which indicators are visible.

## Command Palette

The Command Palette provides a searchable interface to Conduit commands.

It should provide access to:

* Commands.
* Settings.
* Sessions.
* Workspaces.
* Profiles.
* Themes.
* Plugins.
* Diagnostics.
* Actions currently available in context.

## Context Menus

Context menus should expose actions relevant to the selected UI element.

For example, right-clicking a terminal tab may provide:

* Rename.
* Duplicate.
* Move.
* Split.
* Close.
* Restart.
* Change profile.
* Change theme.
* Record session.
* View diagnostics.

## Settings

Settings should be accessible from the UI and represented by the same configuration model used by:

* `config.toml`
* CLI
* TUI
* Configuration editor
* Profiles
* Workspaces

## Live Updates

UI configuration should support live updates whenever possible.

Examples include:

* Theme changes.
* Font changes.
* Keybindings.
* Sidebar visibility.
* Toolbar configuration.
* Status indicators.
* Terminal appearance.

Changes that cannot safely be applied live should trigger the smallest necessary component restart.

## Accessibility

The UI should support:

* Keyboard navigation.
* Screen readers where supported.
* High-contrast configurations.
* Adjustable font sizes.
* Reduced visual effects.
* Focus indicators.
* Configurable shortcuts.

## UI Modes

Conduit may provide:

* Full desktop mode.
* Compact mode.
* Terminal-only mode.
* Minimal mode.
* TUI mode.
* CLI-only mode.
* Safe Mode.

All modes should use the same underlying command and event architecture.

## Customization

Users should be able to customize:

* Menus.
* Toolbar.
* Sidebar.
* Keybindings.
* Themes.
* Fonts.
* Layout.
* Tabs.
* Panes.
* Status bar.
* Command Palette behavior.

## Design Principle

The interface should make common terminal operations immediately accessible while keeping Conduit's deeper capabilities available without forcing them into the default layout.
