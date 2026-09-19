# Conduit Hot Reload

## Overview

Conduit is designed to apply configuration and resource changes while the application is running.

The goal is to minimize restarts and preserve active terminal sessions.

Hot reload is coordinated by the live system.

## Architecture

The live system contains:

```text
src/live/
├── watcher.rs
├── resource_watcher.rs
├── hot_reload.rs
├── change_detector.rs
├── dependency_graph.rs
├── reload_manager.rs
├── live_state.rs
├── restart_policy.rs
├── component_restart.rs
└── rollback.rs
```

## Reload Pipeline

The general process is:

```text
Change Detected
      ↓
Debounce
      ↓
Parse
      ↓
Validate
      ↓
Calculate Diff
      ↓
Dependency Analysis
      ↓
Select Reload Strategy
      ↓
Apply Change
      ↓
Verify
      ↓
Commit or Roll Back
```

## Reload Levels

Not every change requires the same response.

Conduit uses progressively more invasive reload levels.

### Level 1: Live State Update

The setting can be changed immediately.

Examples:

* Cursor style
* Colors
* Font size
* UI visibility
* Notifications

### Level 2: Component Refresh

A component must rebuild internal state.

Examples:

* Font cache
* Theme state
* UI layout
* Search indexes

### Level 3: Component Restart

A specific component must restart.

Examples may include:

* Renderer backend
* Plugin instance
* Connection subsystem

### Level 4: Window Recreation

Some platform changes may require recreating a window.

Terminal sessions should remain alive whenever possible.

### Level 5: Application Restart

A full restart should be the final option.

Conduit should clearly explain why a restart is required.

## Dependency Graph

The dependency graph determines what a changed setting affects.

For example:

```text
Theme
  ↓
Appearance State
  ↓
Renderer
  ↓
Windows
```

Another example:

```text
Font
  ↓
Font Manager
  ↓
Glyph Cache
  ↓
Renderer
```

## Resource Hot Reload

Themes, profiles, and workspaces can be reloaded when their files change.

For themes:

```text
Theme File
    ↓
Validation
    ↓
Theme Manager
    ↓
Appearance State
    ↓
Renderer
```

## Configuration Hot Reload

Changes to:

```text
~/.config/conduit/config.toml
```

are detected by the configuration watcher.

The new configuration is parsed and validated before being applied.

## Invalid Changes

An invalid change must not destroy the current working configuration.

For example:

```text
Valid Configuration
       ↓
File Changed
       ↓
Invalid Configuration
       ↓
Reject
       ↓
Keep Previous Configuration
```

The error should be displayed through diagnostics.

## Transactional Reload

Changes should be applied transactionally when multiple components are affected.

The system should avoid leaving Conduit half-updated.

Conceptually:

```text
Old State
   ↓
Candidate State
   ↓
Validate
   ↓
Prepare Components
   ↓
Apply
   ↓
Verify
   ↓
Commit
```

If verification fails:

```text
Rollback
   ↓
Old State
```

## Restart Minimization

When a restart is required, Conduit should restart the smallest affected component.

For example:

```text
Renderer Restart
```

should not imply:

```text
Terminate Shell
Terminate Session
Close Tab
Close Workspace
```

unless technically unavoidable.

## Live UI Updates

The GUI should update when configuration changes externally.

For example, modifying the theme file in a text editor should cause the active interface to update automatically.

## External Editors

Users may edit configuration with:

* Vim
* Neovim
* Emacs
* Nano
* VS Code
* Other editors

Conduit should detect external file changes just as it detects changes made through its own configuration editor.

## Configuration Editor Integration

Changes made inside Conduit's configuration editor should pass through exactly the same validation and reload pipeline as external file changes.

## File Watcher Behavior

File watchers should account for common editor behavior.

Some editors:

1. Write a temporary file.
2. Rename it over the original.

Others:

1. Truncate the original.
2. Write new content.

Conduit should debounce and handle both patterns safely.

## Reload Feedback

The UI should indicate when a reload occurs.

Possible information includes:

* Changed resource
* Settings affected
* Reload level
* Success
* Failure
* Rollback
* Restart requirement

## Diagnostics

The diagnostics system should record:

* Reload attempts
* Validation failures
* Component failures
* Rollbacks
* Restart requests

## Reload Conflicts

If multiple changes happen quickly, Conduit should avoid applying stale intermediate states unnecessarily.

For example:

```text
Change A
Change B
Change C
```

may be coalesced into one validated state.

## Concurrent Sessions

Hot reload must avoid interfering with active terminal processes.

A configuration change should not accidentally send signals or resize a session unless the changed setting actually requires it.

## Security

Live reload must treat modified files as potentially untrusted.

The reload pipeline must validate and security-check changes before activation.

## Recovery

If a component cannot reload successfully:

1. Preserve the previous valid state where possible.
2. Attempt rollback.
3. Record diagnostics.
4. Notify the user.
5. Offer a controlled restart if necessary.

## Design Goal

Hot reload is intended to make Conduit feel continuously configurable.

The principle is:

> Change what you want while Conduit keeps everything else running.
