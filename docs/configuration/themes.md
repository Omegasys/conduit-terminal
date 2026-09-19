# Conduit Themes

## Overview

Themes control the visual appearance of Conduit.

Unlike hard-coded themes, Conduit treats themes as dynamically discoverable resources.

A user can add a valid theme file to the supported theme directory and Conduit can make it available automatically.

## Theme Locations

User themes can be placed in:

```text
~/.config/conduit/themes/
```

Additional user data themes may be supported under:

```text
~/.local/share/conduit/themes/
```

System-wide themes can be installed under:

```text
/usr/share/conduit/themes/
```

## Theme Discovery

Conduit automatically searches supported directories for theme resources.

For example:

```text
~/.config/conduit/themes/
├── my-theme.toml
├── solarized.toml
└── custom-dark.toml
```

A valid `.toml` theme should be recognized without requiring a new Conduit release.

## Theme Architecture

Theme management is implemented through:

```text
src/themes/
├── manager.rs
├── loader.rs
├── watcher.rs
├── theme.rs
├── colors.rs
├── fonts.rs
└── appearance.rs
```

The general resource system also provides:

```text
src/resources/
├── discovery.rs
├── registry.rs
├── paths.rs
├── resource.rs
├── theme.rs
├── manifest.rs
└── validation.rs
```

## Theme File

A theme is represented as TOML data.

Example:

```toml
[theme]
name = "My Theme"
version = "1.0"
description = "A custom Conduit theme"

[colors]
background = "#101010"
foreground = "#f0f0f0"
cursor = "#ffffff"
selection = "#303030"

[ansi]
black = "#000000"
red = "#ff5555"
green = "#50fa7b"
yellow = "#f1fa8c"
blue = "#6272a4"
magenta = "#ff79c6"
cyan = "#8be9fd"
white = "#f8f8f2"

[appearance]
opacity = 1.0
```

The schema can expand as Conduit gains additional visual capabilities.

## Theme Components

Themes may control:

* Terminal colors
* ANSI colors
* Background
* Foreground
* Cursor
* Selection
* UI colors
* Menus
* Tabs
* Sidebar
* Status bar
* Borders
* Notifications
* Flow View
* Syntax highlighting
* Font configuration
* Opacity
* Visual effects

## Theme Validation

Before a theme enters the registry, Conduit validates:

* TOML syntax
* Required fields
* Data types
* Color formats
* Numeric ranges
* Supported properties
* Schema version

Invalid themes should not replace an active valid theme.

## Live Theme Discovery

The theme watcher monitors theme directories.

When a theme is:

* Added
* Modified
* Renamed
* Deleted

the theme manager updates the registry.

## Live Theme Reload

If the active theme changes on disk:

```text
Theme File Changed
       ↓
Parse
       ↓
Validate
       ↓
Build Theme State
       ↓
Update Theme Manager
       ↓
Refresh UI
       ↓
Refresh Renderer
```

The terminal process itself should continue running.

## Adding a Theme

A user can install a theme simply by placing the theme file in the theme directory.

For example:

```text
cp my-theme.toml ~/.config/conduit/themes/
```

Conduit should detect the new file automatically.

## Removing a Theme

Removing a theme should remove it from the available theme registry.

If the deleted theme is currently active, Conduit should preserve the currently rendered state temporarily or fall back to a valid theme according to policy.

It should not crash because a theme file disappeared.

## Theme Selection

Themes can be selected through:

* Settings
* Theme manager
* Command palette
* TUI
* CLI
* Workspace configuration
* Profile configuration

## Theme Manager

The GUI theme manager should provide:

* Theme list
* Search
* Preview
* Apply
* Reload
* Import
* Export
* Remove
* Validation status

## Preview

A theme preview can display:

* Terminal colors
* ANSI colors
* Cursor
* Selection
* Tabs
* Menus
* Sidebar
* Status bar
* Flow View

Previewing a theme should not modify the active theme until the user applies it.

## Workspace Themes

A workspace can select its own theme.

Example:

```toml
[workspace]
theme = "cyberpunk"
```

This allows different workspaces to have different visual environments.

## Profile Themes

Profiles can select a default theme.

Workspace-specific selection can override the profile when permitted by configuration precedence.

## Theme Precedence

Theme selection may be supplied by:

```text
Global Configuration
       ↓
Profile
       ↓
Workspace
       ↓
Session / Runtime Override
```

The configuration engine determines the effective theme.

## Theme Inheritance

Future versions may support theme inheritance.

For example:

```text
Default Dark
     ↓
Custom Dark
     ↓
Project Theme
```

Only explicitly overridden properties would need to be defined by the child theme.

Circular inheritance must be rejected.

## Theme Names

Theme identifiers should be stable and suitable for configuration references.

The display name can be different from the internal identifier.

For example:

```toml
[theme]
id = "solarized-dark"
name = "Solarized Dark"
```

## Theme Metadata

Themes may include:

* Identifier
* Display name
* Version
* Author
* Description
* License metadata
* Schema version
* Preview information

Metadata should not be required to contain executable code.

## Security

Theme files are configuration resources, not executable plugins.

A theme should not be able to:

* Execute commands
* Access arbitrary files
* Open network connections
* Modify processes

Theme parsing must remain data-only.

## CLI

Example commands:

```text
conduit theme list
conduit theme show <name>
conduit theme apply <name>
conduit theme reload <name>
conduit theme validate <file>
conduit theme import <file>
conduit theme remove <name>
```

## TUI

The TUI should provide equivalent theme selection and management functionality.

## Diagnostics

Theme failures should integrate with diagnostics.

Possible errors include:

* Invalid TOML
* Invalid color
* Missing required field
* Unsupported property
* Schema mismatch
* Failed live reload

## Renderer Integration

The theme manager provides visual state to the renderer.

A theme change should normally result in:

```text
Theme Manager
      ↓
Appearance State
      ↓
Renderer
      ↓
New Frame
```

No terminal process restart should be necessary.

## Design Goal

Themes should be installable, discoverable, editable, and reloadable without modifying Conduit itself.

The principle is:

> If it is a valid theme resource, Conduit should be able to find it.
