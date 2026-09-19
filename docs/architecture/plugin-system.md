# Conduit Plugin System

## Overview

Conduit uses a modular plugin architecture to extend functionality without requiring the terminal core to contain every feature directly.

Plugins can add integrations, commands, UI components, file previews, connection types, diagnostics, automation, and other capabilities while communicating with Conduit through a defined plugin API.

The plugin system is designed around four principles:

* Plugins should be optional.
* Plugins should be isolated from the core whenever practical.
* Plugin permissions should be explicit.
* Plugins should be discoverable and manageable without restarting Conduit whenever possible.

## Architecture

The plugin system is primarily implemented under:

```text
src/plugins/
├── manager.rs
├── loader.rs
├── lifecycle.rs
├── permissions.rs
├── sandbox.rs
├── api.rs
├── registry.rs
├── manifest.rs
├── dependencies.rs
└── updates.rs
```

The plugin manager coordinates discovery, validation, loading, lifecycle management, permissions, and unloading.

## Plugin Responsibilities

A plugin may provide:

* Commands
* Command palette entries
* Keyboard actions
* Menu items
* Toolbar actions
* Sidebar panels
* Settings pages
* Status indicators
* Terminal integrations
* Connection providers
* File previews
* Workspace functionality
* Diagnostics
* Notifications
* Custom protocol handlers
* External application integrations
* Automation
* Developer tools

Plugins should not directly manipulate internal terminal state when a stable public API can provide the required operation.

## Plugin Manifest

Every installed plugin should have a manifest describing the plugin.

Example:

```toml
[plugin]
name = "git"
version = "1.0.0"
description = "Git integration for Conduit"
author = "Conduit Project"

[plugin.api]
version = "1"

[permissions]
filesystem = "read"
process = false
network = false
terminal = true
ui = true
```

The manifest allows Conduit to determine what the plugin is, which API version it requires, and what capabilities it requests.

## Plugin Discovery

Plugins can be discovered from supported plugin directories.

Typical locations include:

```text
/usr/share/conduit/plugins/
~/.local/share/conduit/plugins/
~/.config/conduit/plugins/
```

User-installed plugins should be handled separately from system-installed plugins.

The resource system and plugin system are related but intentionally distinct.

Themes, profiles, and workspaces are data resources.

Plugins may contain executable code and therefore require additional validation and security controls.

## Plugin Registry

The plugin registry maintains information about discovered plugins.

A registry entry may contain:

* Plugin identifier
* Display name
* Version
* Installation path
* API compatibility
* Enabled state
* Requested permissions
* Granted permissions
* Dependencies
* Loaded state
* Error state
* Last validation result

The registry should be available to the GUI, TUI, CLI, diagnostics system, and command palette.

## Plugin Lifecycle

A plugin follows a controlled lifecycle:

```text
Discover
   ↓
Read Manifest
   ↓
Validate
   ↓
Resolve Dependencies
   ↓
Evaluate Permissions
   ↓
Load
   ↓
Initialize
   ↓
Register Capabilities
   ↓
Active
   ↓
Disable / Reload / Unload
```

Plugins should not be considered active until validation and initialization have succeeded.

## Hot Reloading

Conduit should support plugin reloads where technically safe.

When a plugin file changes, the plugin manager can determine whether:

* The plugin can be reloaded immediately.
* A specific component must be restarted.
* The plugin must be disabled first.
* A full Conduit restart is required.

The live reload system should always prefer the smallest necessary restart.

For example, changing a plugin's UI configuration should not require restarting unrelated terminal sessions.

## Plugin Permissions

Plugins should use explicit permissions.

Possible permissions include:

* Terminal access
* Process execution
* Filesystem read
* Filesystem write
* Network access
* Clipboard access
* Environment access
* Secret access
* UI modification
* Workspace access
* Session access
* System integration

Permissions should be visible through the Security Center and plugin manager.

## Sandboxing

Where platform capabilities permit it, plugins should run with restricted privileges.

The sandbox layer should limit access according to the plugin's granted permissions.

The architecture should avoid assuming that every Linux environment provides identical sandboxing capabilities.

If strong isolation is unavailable, Conduit should still enforce logical permission boundaries at the API level.

## Plugin API

The plugin API provides stable interfaces for interacting with Conduit.

The API should expose abstractions rather than private implementation details.

Examples include:

```text
Terminal API
Session API
Workspace API
Command API
Event API
Configuration API
Resource API
UI API
Notification API
Connection API
Diagnostics API
```

The API should be versioned independently from the internal implementation.

## Events and Commands

Plugins communicate with Conduit through the central event bus and command system.

A plugin can:

* Register commands.
* Subscribe to events.
* Publish events where permitted.
* Add command palette entries.
* Respond to lifecycle events.
* React to workspace changes.
* React to session changes.
* Provide UI components.

This avoids requiring every plugin to establish its own communication mechanism.

## Dependencies

Plugins may depend on other plugins or specific Conduit API versions.

Dependency resolution should detect:

* Missing dependencies
* Incompatible versions
* Circular dependencies
* Disabled dependencies
* Conflicting capabilities

A plugin with unresolved required dependencies should remain inactive.

## Failure Isolation

A plugin failure should not normally terminate the entire terminal application.

If a plugin crashes or becomes invalid:

1. Detect the failure.
2. Record diagnostic information.
3. Disable the affected plugin if necessary.
4. Preserve unrelated sessions and workspaces.
5. Notify the user.
6. Offer recovery or reload options.

The exact isolation mechanism depends on the plugin implementation and platform.

## Plugin Security

Plugins should be treated as potentially privileged extensions.

Installing a plugin should therefore not automatically grant unlimited access to:

* Files
* Network
* Processes
* Clipboard
* Environment variables
* Credentials
* Terminal sessions

Security-sensitive capabilities should require explicit authorization where appropriate.

## Plugin Manager

The GUI plugin manager should provide:

* Installed plugins
* Available plugins
* Enabled/disabled state
* Version information
* Permissions
* Dependencies
* Validation status
* Reload controls
* Uninstall controls
* Diagnostics

The TUI and CLI should expose equivalent management functionality where practical.

## CLI Integration

Examples:

```text
conduit plugin list
conduit plugin enable <plugin>
conduit plugin disable <plugin>
conduit plugin reload <plugin>
conduit plugin info <plugin>
conduit plugin permissions <plugin>
conduit plugin diagnose <plugin>
```

## Diagnostics

Plugin failures should integrate with the diagnostics system.

Diagnostics may include:

* Load failures
* API incompatibilities
* Permission errors
* Dependency failures
* Runtime errors
* Reload failures
* Resource usage
* Sandbox violations

## Design Goals

The plugin system should make Conduit extensible without turning the core application into an unmaintainable collection of integrations.

The goal is:

> Extend the terminal without requiring the terminal core to know everything.

Plugins should provide powerful functionality while respecting the same configuration, event, security, and lifecycle systems used by the rest of Conduit.
