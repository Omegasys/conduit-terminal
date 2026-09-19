# Conduit Configuration Editor

## Overview

Conduit includes a graphical configuration editor for users who want direct control over the underlying TOML configuration.

The editor provides the power of manual configuration while using the same schema, validation, transaction, and live-reload systems as the rest of Conduit.

## Architecture

The editor is implemented under:

```text
src/gui/config_editor/
├── editor.rs
├── syntax.rs
├── autocomplete.rs
├── validation.rs
├── diagnostics.rs
├── documentation.rs
├── preview.rs
├── apply.rs
└── revert.rs
```

## Configuration File

The primary configuration file is:

```text
~/.config/conduit/config.toml
```

The editor can also work with supported profiles, workspaces, themes, and other resource files.

## Editor Modes

The configuration editor can provide several modes.

### Structured Mode

Provides forms and controls for common settings.

### Text Mode

Provides direct TOML editing.

### Split Mode

Shows structured settings and the corresponding TOML together.

## Syntax Highlighting

The editor should provide TOML syntax highlighting for:

* Tables
* Keys
* Strings
* Numbers
* Booleans
* Arrays
* Comments

## Autocomplete

Autocomplete can provide:

* Known configuration keys
* Valid values
* Profiles
* Themes
* Workspaces
* Commands
* Resource identifiers

Autocomplete should be schema-aware.

## Schema Integration

The editor reads the same configuration schema used by the configuration engine.

This prevents the editor from maintaining a separate definition of valid settings.

## Inline Diagnostics

Invalid configuration should be highlighted directly in the editor.

Diagnostics can identify:

* Syntax errors
* Unknown keys
* Invalid values
* Missing fields
* Conflicts
* Deprecated settings
* Security concerns

## Documentation

The editor can display contextual documentation for settings.

For example, selecting:

```text
terminal.scrollback_lines
```

can show:

* Description
* Type
* Default
* Allowed range
* Related settings
* Reload behavior

## Preview

Settings that support live preview can be previewed before being permanently committed.

Examples include:

* Themes
* Fonts
* Colors
* Cursor styles
* UI appearance

## Apply

Applying changes uses the normal configuration transaction system.

```text
Editor
  ↓
Parse
  ↓
Validate
  ↓
Security Check
  ↓
Diff
  ↓
Apply
  ↓
Live Reload
```

## Revert

Users can revert unsaved editor changes.

The editor should also support restoring the last known valid configuration when practical.

## External Changes

If another application modifies the configuration while it is open, the editor should detect the conflict.

Possible behavior:

* Reload external changes.
* Show a comparison.
* Let the user choose which version to keep.
* Merge changes where safely possible.

The editor should avoid silently overwriting external modifications.

## Atomic Saving

Configuration files should preferably be saved atomically.

A safe process is:

```text
Write Temporary File
       ↓
Parse
       ↓
Validate
       ↓
Replace Original
```

## Configuration History

The editor can optionally provide access to recent configuration versions.

This can assist with:

* Recovering from mistakes
* Comparing changes
* Undoing configuration changes
* Diagnosing regressions

Retention should be configurable.

## Profiles

The editor can edit profile files directly.

Users should be able to see which settings come from:

* Defaults
* Global configuration
* Profile
* Workspace
* Runtime override

## Workspaces

Workspace configuration can be edited through the same interface.

The editor should understand workspace-specific schemas and references.

## Themes

Theme files can be edited with:

* TOML syntax highlighting
* Color previews
* Validation
* Live preview
* Apply
* Revert

## Keybindings

Keybinding configuration should receive specialized editor assistance.

For example:

* Key syntax validation
* Command completion
* Conflict detection
* Context selection

## Security Settings

Security-sensitive settings should be clearly identified.

The editor should explain when a setting:

* Requires elevated permission
* Affects sandboxing
* Changes plugin permissions
* Changes clipboard policy
* Changes remote behavior

## Unsaved Changes

The editor should clearly indicate unsaved changes.

Closing the editor should provide an appropriate choice when modifications have not been saved.

## Live Editing

Where safe, changes can be applied while editing.

This can provide immediate feedback without requiring the user to close Settings.

## Error Recovery

If a configuration change fails:

```text
Failed Configuration
       ↓
Rollback
       ↓
Previous Valid State
       ↓
Show Diagnostic
```

The editor should preserve the invalid text so the user can correct it.

## CLI and TUI

The same configuration engine should be usable outside the GUI.

The CLI can provide validation:

```text
conduit config validate
```

and the TUI can provide a keyboard-driven configuration editor.

## Design Goal

The configuration editor should make advanced customization accessible without hiding the underlying configuration format.

The principle is:

> Give users both the controls and the configuration file.
