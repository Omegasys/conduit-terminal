# Conduit Settings

## Overview

Conduit provides a centralized settings system for configuring the terminal, interface, sessions, workspaces, security, rendering, plugins, integrations, and other application behavior.

Settings are exposed through the GUI settings interface, TUI configuration interface, CLI commands, and the main Conduit configuration file.

All interfaces use the same internal configuration model.

The goal is to make configuration understandable, discoverable, editable, and immediately applicable whenever possible.

## Settings Sources

Conduit can obtain settings from multiple sources:

* Built-in defaults
* System configuration
* User configuration
* Profile configuration
* Workspace configuration
* Session configuration
* Command-line overrides
* Plugin configuration

Higher-priority configuration can override lower-priority configuration according to the configuration precedence rules.

## Main Configuration File

The primary user configuration file is:

`~/.config/conduit/config.toml`

Additional resources can be stored in the Conduit configuration and data directories.

Examples include:

* Themes
* Profiles
* Workspaces
* Plugin configuration
* Keybindings
* Resource manifests

## Settings Categories

The settings interface should organize configuration into logical categories.

### General

General application behavior.

Examples:

* Startup behavior
* Default shell
* Default profile
* Default workspace
* Confirmation behavior
* Notifications
* Application language

### Appearance

Visual configuration.

Examples:

* Theme
* Font
* Font size
* Cursor appearance
* Transparency
* Padding
* Borders
* Tab appearance
* Sidebar appearance
* Toolbar visibility

### Terminal

Terminal behavior.

Examples:

* Scrollback size
* Bell behavior
* Cursor behavior
* Mouse reporting
* Alternate screen
* Unicode handling
* Character encoding
* Terminal identification

### Rendering

Rendering configuration.

Examples:

* Rendering backend
* GPU acceleration
* Software fallback
* Frame rate
* Scaling
* Font rendering
* Ligatures
* Image rendering

### Shell

Shell integration.

Examples:

* Default shell
* Shell arguments
* Environment variables
* Working directory
* Shell integration
* History integration
* Prompt integration

### Tabs and Panes

Window layout behavior.

Examples:

* Tab placement
* New-tab behavior
* Pane splitting
* Pane resizing
* Pane focus
* Automatic layout behavior

### Workspaces

Workspace behavior.

Examples:

* Startup workspace
* Workspace persistence
* Workspace restoration
* Workspace-specific profiles
* Workspace-specific themes
* Workspace layout

### Input

Keyboard and mouse behavior.

Examples:

* Keybindings
* Mouse bindings
* Modifier behavior
* Copy and paste shortcuts
* Command palette shortcuts

### Clipboard

Clipboard behavior.

Examples:

* Copy mode
* Paste confirmation
* Clipboard history
* Clipboard protocol support
* Sensitive clipboard handling

### Security

Security controls.

Examples:

* Security profile
* Hyperlink handling
* Clipboard protections
* Plugin permissions
* Sandboxing
* Untrusted output handling
* Safe mode

### Plugins

Plugin behavior.

Examples:

* Enabled plugins
* Plugin permissions
* Plugin sandboxing
* Plugin-specific configuration
* Plugin update behavior

### Remote Sessions

Remote connection behavior.

Examples:

* SSH integration
* Remote environment detection
* Connection profiles
* Remote clipboard behavior
* Remote security restrictions

### Notifications

Notification behavior.

Examples:

* Terminal bell notifications
* Command completion notifications
* Session notifications
* Plugin notifications

### Accessibility

Accessibility options.

Examples:

* High contrast
* Reduced motion
* Screen-reader support
* Keyboard navigation
* Font scaling
* Focus indicators
* Audible notifications

## Configuration Scope

Settings can have different scopes.

### Global

Global settings apply throughout Conduit.

Examples:

* Default theme
* Global keybindings
* Security defaults

### Profile

Profile settings apply when a particular profile is active.

### Workspace

Workspace settings apply to a workspace and its associated sessions.

### Session

Session settings apply only to a particular terminal session.

### Component

Some settings apply only to a specific component.

Examples:

* Renderer settings
* Sidebar settings
* Plugin settings

## Live Settings

Conduit should apply settings without requiring a full restart whenever technically possible.

Examples include:

* Theme changes
* Font changes
* Sidebar visibility
* Toolbar visibility
* Keybindings
* Tab settings
* Pane settings
* Security policy changes
* Notification settings

If a setting cannot safely be changed while Conduit is running, the settings interface should explain why.

Only the smallest affected component should be restarted when possible.

## Setting Validation

Settings are validated before being applied.

Validation can include:

* Type validation
* Range validation
* Enumeration validation
* Path validation
* Dependency validation
* Security validation
* Compatibility validation

Invalid settings should not replace a currently valid configuration.

## Pending Changes

The settings interface can maintain a pending configuration state.

This allows users to:

* Preview changes
* Apply changes
* Revert changes
* Compare changes
* Restore previous values

Settings that support immediate application should update as they are changed.

## Configuration Transactions

Related settings can be changed as a transaction.

A transaction should either:

* Apply successfully
* Roll back safely

Partial application should be avoided when it could leave Conduit in an inconsistent state.

## Search

The settings interface should provide searchable settings.

Search should support:

* Setting names
* Descriptions
* Categories
* Command IDs
* Configuration keys
* Plugin settings

Selecting a search result should navigate directly to the relevant setting.

## Configuration File Integration

GUI changes should update the configuration model and persist them to the appropriate configuration file.

External configuration-file changes should be detected by the live configuration system.

Conduit should then:

1. Detect the change
2. Parse the configuration
3. Validate it
4. Calculate the difference
5. Apply safe changes
6. Roll back invalid changes
7. Report diagnostics

## Security

Security-related settings receive additional validation.

A plugin or untrusted resource must not be able to silently weaken global security settings.

Security restrictions should follow the established security precedence rules.

## Diagnostics

Settings errors should provide useful diagnostics.

A diagnostic should identify:

* Setting
* Current value
* Requested value
* Problem
* Affected component
* Suggested correction when appropriate

Diagnostics can be displayed through the settings interface, command palette, status bar, or diagnostics panel.

## Design Principles

Conduit settings should follow these principles:

* One configuration model
* Clear categories
* Searchable settings
* Safe defaults
* Explicit validation
* Live updates when possible
* Minimal restarts
* Transactional changes
* GUI/TUI/CLI parity
* Security-aware configuration
* Recoverable failures

The settings system should make Conduit powerful without requiring users to manually edit configuration files for ordinary tasks.
