# Conduit Configuration

## Overview

Conduit uses a centralized configuration system shared by the GUI, TUI, CLI, terminal core, plugins, and supporting subsystems.

The primary user configuration file is:

```text
~/.config/conduit/config.toml
```

The `.config` directory is the user's standard configuration directory. Conduit stores its configuration inside the `conduit` subdirectory.

## Configuration Architecture

Configuration functionality is implemented through:

```text
src/config_engine/
├── loader.rs
├── parser.rs
├── schema.rs
├── validator.rs
├── defaults.rs
├── profiles.rs
├── workspaces.rs
├── diff.rs
├── transaction.rs
├── rollback.rs
├── migration.rs
├── serialization.rs
├── state.rs
└── errors.rs
```

The configuration engine is independent from any particular user interface.

## Configuration Sources

Conduit can combine configuration from several sources.

Typical sources include:

1. Built-in defaults
2. System configuration
3. User configuration
4. Profile configuration
5. Workspace configuration
6. Session-specific settings
7. Command-line overrides
8. Runtime changes

The exact precedence is determined by the configuration engine.

## Main Configuration File

A basic configuration might look like:

```toml
[terminal]
shell = "/bin/bash"
scrollback_lines = 10000

[appearance]
theme = "default"
font_size = 12.0

[window]
remember_size = true

[behavior]
confirm_close = true
```

The example is intentionally small. Conduit can expose substantially more configuration through the same system.

## Configuration Model

The GUI, TUI, and CLI should all operate on the same internal configuration model.

For example:

```text
GUI
 │
TUI ──→ Configuration Engine ←── CLI
 │
Plugins
 │
Core
```

This prevents different interfaces from developing incompatible configuration behavior.

## Parsing

Configuration files are parsed as TOML.

Parsing and validation are separate operations.

A syntactically valid TOML file can still contain invalid Conduit configuration.

## Schema Validation

The configuration schema defines:

* Valid sections
* Valid keys
* Data types
* Allowed values
* Defaults
* Constraints
* Deprecated settings
* Security-sensitive settings

Invalid configuration should produce actionable diagnostics.

## Configuration Errors

Conduit should identify:

* File
* Section
* Key
* Invalid value
* Expected value
* Reason for rejection

Where possible, Conduit should continue using the last known valid configuration instead of entering an undefined state.

## Live Configuration

Configuration changes should normally be detected automatically.

The general flow is:

```text
config.toml
    ↓
File Watcher
    ↓
Parser
    ↓
Validator
    ↓
Configuration Diff
    ↓
Dependency Analysis
    ↓
Live Apply
```

## Hot Reloading

Changes that do not require reconstruction should be applied immediately.

Examples may include:

* Theme
* Font size
* Cursor appearance
* Keybindings
* Notifications
* Scrollback preferences
* UI visibility

Changes affecting fundamental process or renderer architecture may require a component restart.

Conduit should restart the smallest affected component rather than the entire application whenever practical.

## Transactions

Configuration changes should be applied transactionally.

The conceptual process is:

```text
Current State
    ↓
Create Candidate State
    ↓
Validate
    ↓
Security Check
    ↓
Apply
    ↓
Success
```

If application fails:

```text
Failed Apply
    ↓
Rollback
    ↓
Previous Valid State
```

## Configuration Editor

The GUI configuration editor provides:

* TOML editing
* Syntax highlighting
* Autocomplete
* Schema information
* Validation
* Error diagnostics
* Documentation
* Preview
* Apply
* Revert

The editor should use the same configuration engine as normal file loading.

## Settings Interface

The graphical Settings application provides a structured interface for common configuration.

Categories can include:

* General
* Appearance
* Fonts
* Colors
* Terminal
* Windows
* Tabs
* Panes
* Workspaces
* Keyboard
* Mouse
* Clipboard
* Shell
* Remote
* Rendering
* Graphics
* Notifications
* Accessibility
* Security
* Plugins
* Profiles
* Themes
* Recording
* Advanced
* Developer

Changes made through Settings should update the same underlying configuration state as changes made through `config.toml`.

## Configuration Profiles

Profiles provide reusable groups of settings.

Profiles are documented separately in `profiles.md`.

## Workspaces

Workspace-specific configuration allows different environments to use different settings.

Workspace configuration is documented in `workspaces.md`.

## Themes

Themes are configurable resources rather than hard-coded application states.

Theme behavior is documented in `themes.md`.

## Keybindings

Keybindings are configuration data and can be changed without rebuilding Conduit.

Keybinding behavior is documented in `keybindings.md`.

## Runtime Configuration

The configuration engine maintains an in-memory configuration state.

This state represents the currently active configuration after:

* Defaults
* Files
* Profiles
* Workspaces
* Overrides
* Runtime changes

have been resolved.

## Configuration Diff

When configuration changes, Conduit calculates the difference between the previous and candidate state.

The diff identifies:

* Added values
* Removed values
* Changed values
* Affected components
* Reload requirements

## Dependency Graph

Configuration settings can affect different components.

For example:

```text
font.size
   ↓
Font Manager
   ↓
Text Layout
   ↓
Renderer
```

Another setting may produce:

```text
theme.name
   ↓
Theme Manager
   ↓
Renderer
   ↓
UI
```

The dependency graph helps determine what needs to be updated.

## CLI Overrides

The CLI can override configuration for a particular invocation.

These overrides should normally remain temporary unless explicitly saved.

Example:

```text
conduit --profile development
conduit --theme nord
conduit --font-size 14
```

## Environment Variables

Environment variables may be used for selected runtime behavior where appropriate.

Environment variables should not silently override security-sensitive configuration without being clearly documented.

## Migration

Configuration formats may evolve.

The migration subsystem handles older configurations when possible.

Migration should:

1. Detect the configuration version.
2. Determine whether migration is required.
3. Transform the configuration.
4. Validate the result.
5. Preserve the original when appropriate.
6. Apply the migrated configuration.

## Backup and Recovery

Before significant automatic migrations or destructive configuration operations, Conduit may create a backup.

Users should also be able to export configuration manually.

## Security

Configuration files can influence powerful application behavior.

Conduit should therefore:

* Validate all configuration.
* Avoid executing arbitrary configuration values.
* Treat externally supplied configuration as untrusted.
* Protect security-sensitive settings.
* Preserve known-good state when a change fails.

## Design Goal

Conduit configuration should provide one consistent control system regardless of how the user interacts with the application.

The principle is:

> Configure it once. Let every Conduit interface understand it.
