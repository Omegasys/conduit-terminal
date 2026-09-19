# Conduit Sidebar

## Overview

The Conduit sidebar provides a persistent location for navigation, resource management, session information, and advanced tools.

It should be collapsible so that users can maximize terminal space.

## Sidebar Architecture

The sidebar should use modular panels.

Possible panels include:

* Workspaces
* Sessions
* Connections
* Profiles
* Themes
* History
* Plugins
* Files
* Diagnostics
* Flow View

## Panel System

Each panel should have:

* Unique identifier.
* Display name.
* Icon.
* Command integration.
* Configuration.
* Optional plugin ownership.
* Permission requirements where applicable.

## Workspaces Panel

The Workspaces panel provides access to:

* Available workspaces.
* Active workspace.
* Workspace templates.
* Recent workspaces.
* Workspace creation.
* Workspace management.

Resources should update automatically when workspace files are added, removed, or changed.

## Sessions Panel

The Sessions panel may show:

* Active sessions.
* Local sessions.
* Remote sessions.
* Disconnected sessions.
* Restorable sessions.
* Session status.
* Security profile.

Users should be able to select a session and focus its terminal.

## Connections Panel

The Connections panel provides access to configured connections.

Examples include:

* Local shell.
* SSH.
* Serial.
* Container.
* Other supported remote connections.

Connection credentials and secrets should not be displayed unnecessarily.

## Profiles Panel

The Profiles panel allows users to:

* Browse profiles.
* Activate profiles.
* Create profiles.
* Duplicate profiles.
* Edit profiles.
* Delete user-created profiles.

Security profiles should have additional indicators showing their effective restrictions.

## Themes Panel

The Themes panel provides:

* Installed themes.
* Active theme.
* Theme preview.
* Theme search.
* Theme management.
* Theme reload status.

New theme files should appear automatically through the resource discovery system.

## History Panel

The History panel provides access to Conduit's searchable command and session history.

It may display:

* Commands.
* Shell.
* Session.
* Workspace.
* Timestamp.
* Working directory.
* Exit status.

Sensitive history should respect configured redaction and retention policies.

## Plugins Panel

The Plugins panel provides:

* Installed plugins.
* Plugin status.
* Permissions.
* Enabled/disabled state.
* Plugin diagnostics.
* Plugin configuration.

Permission information should be clearly visible.

## Files Panel

The Files panel can provide workspace-related filesystem navigation.

Filesystem access should remain subject to the relevant security policy.

## Diagnostics Panel

The Diagnostics panel may show:

* Renderer status.
* PTYs.
* Sessions.
* Memory usage.
* Plugin status.
* Resource loading.
* Live reload state.
* Event activity.
* Security events.
* Component restart information.

## Flow View Panel

Flow View can be displayed inside the sidebar or expanded into a dedicated window.

It provides a visual representation of Conduit's internal information flow.

## Collapsing

The entire sidebar should be collapsible.

Possible commands include:

* Toggle Sidebar.
* Focus Sidebar.
* Focus Terminal.
* Select Panel.

## Panel State

Conduit should remember:

* Sidebar visibility.
* Width.
* Active panel.
* Panel ordering.
* Expanded/collapsed state.

Workspace-specific settings may override global preferences.

## Live Resource Updates

The sidebar should subscribe to resource and event notifications.

For example:

```text
New theme file
    ↓
Resource watcher
    ↓
Theme registry
    ↓
Resource event
    ↓
Themes panel
    ↓
UI update
```

No application restart should be required for ordinary resource discovery.

## Plugin Panels

Plugins may contribute sidebar panels when granted the appropriate UI permissions.

A plugin panel should remain isolated from the core UI and should be removable if the plugin is disabled.

## Accessibility

Sidebar panels should support:

* Keyboard navigation.
* Focus management.
* Screen readers.
* Expand/collapse controls.
* Accessible labels.

## Design Principle

The sidebar is Conduit's navigation and management workspace.

It should provide access to advanced functionality without requiring the user to memorize commands or navigate through multiple settings windows.
