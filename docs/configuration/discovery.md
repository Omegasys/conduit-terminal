# Conduit Resource Discovery

## Overview

Resource discovery is the mechanism through which Conduit automatically finds supported resources on the filesystem.

This allows users to add themes, profiles, workspaces, and other supported resources without rebuilding or modifying Conduit.

## Discovery Architecture

Discovery is implemented through:

```text
src/resources/discovery.rs
src/resources/paths.rs
src/resources/registry.rs
src/live/resource_watcher.rs
```

The general process is:

```text
Resource Directory
       ↓
Filesystem Scan
       ↓
Identify Resource
       ↓
Parse
       ↓
Validate
       ↓
Registry
       ↓
Available to Conduit
```

## Discovery Paths

User resources are normally searched under:

```text
~/.config/conduit/
├── themes/
├── profiles/
└── workspaces/
```

System resources may be located under:

```text
/usr/share/conduit/
├── themes/
├── profiles/
└── workspaces/
```

Additional user data locations may be supported under:

```text
~/.local/share/conduit/
```

## File Types

Discovery should only process supported resource formats.

For TOML resources, Conduit can search for:

```text
*.toml
```

The resource type can be determined from:

* Directory
* Manifest
* Explicit type field
* Schema
* File structure

Directory context should be preferred where it provides an unambiguous resource type.

## Recursive Discovery

Whether subdirectories are searched recursively depends on the resource type.

Simple resources may use:

```text
themes/*.toml
```

More complex resources can use:

```text
workspaces/project/workspace.toml
```

The discovery policy should prevent accidentally loading unrelated configuration files.

## Resource Identification

Discovery should determine:

* Resource type
* Resource identifier
* File path
* Version
* Metadata

An identifier should remain stable even if the display name changes.

## Duplicate Resources

If multiple resources use the same identifier, Conduit resolves them according to precedence.

The registry should record the conflict rather than silently hiding it.

Diagnostics should identify:

* Conflicting identifier
* Resource paths
* Selected resource
* Precedence reason

## Initial Discovery

At startup, Conduit performs an initial resource scan.

The startup process should:

1. Determine resource paths.
2. Check available directories.
3. Scan supported files.
4. Parse candidates.
5. Validate candidates.
6. Register valid resources.
7. Report invalid resources.

Discovery should be efficient enough to avoid noticeably delaying startup.

## Live Discovery

After initial discovery, Conduit can monitor resource directories.

For example:

```text
~/.config/conduit/themes/
```

If a user adds:

```text
my-theme.toml
```

Conduit detects the file and loads the new theme.

## New Resource

The process for a new resource is:

```text
New File
   ↓
Filesystem Event
   ↓
Resource Discovery
   ↓
Parse
   ↓
Validate
   ↓
Register
   ↓
Notify UI
```

The resource becomes available without restarting Conduit.

## Modified Resource

When an existing resource changes:

```text
File Changed
   ↓
Parse
   ↓
Validate
   ↓
Compare
   ↓
Reload
```

If validation fails, the previous valid version remains active.

## Deleted Resource

When a resource is deleted:

```text
File Deleted
   ↓
Discovery Update
   ↓
Remove from Registry
   ↓
Handle Active Resource
```

If the resource is currently active, Conduit should apply an appropriate fallback policy rather than crashing.

## Renamed Resource

A rename can be treated as:

```text
Old Resource Removed
        +
New Resource Discovered
```

If the resource identifier remains unchanged, Conduit can treat it as a path change.

## Resource Watcher

The live resource watcher should monitor relevant directories for:

* Create
* Modify
* Delete
* Rename

Filesystem events should be debounced where necessary to avoid processing incomplete writes.

## Atomic Writes

Resource writers should preferably use atomic replacement.

For example:

```text
Write Temporary File
       ↓
Validate
       ↓
Rename Into Place
```

This reduces the chance that Conduit observes a partially written resource.

## Validation

Discovery must never assume that a file is valid merely because it has the correct extension.

Every discovered resource should pass resource-specific validation.

## Invalid Resources

Invalid resources should:

* Remain visible to diagnostics.
* Not become active.
* Not replace valid resources.
* Produce useful error information.

## Resource Registry Updates

The registry should be updated incrementally.

Conduit should not need to rebuild the entire resource registry when one theme changes.

## Discovery Events

Discovery produces events such as:

* ResourceDiscovered
* ResourceChanged
* ResourceRemoved
* ResourceInvalid
* ResourceReloaded

These events integrate with the central event bus.

## GUI

The GUI can display newly discovered resources automatically.

For example, adding a theme file should update the theme selection list without reopening Settings.

## TUI

The TUI should similarly update resource lists while running.

## CLI

The CLI can expose discovery diagnostics:

```text
conduit resource scan
conduit resource list
conduit resource validate <file>
conduit resource paths
```

## Security

Discovery should not execute discovered resources.

Resource discovery should be data-only.

Executable plugins require the separate plugin discovery and security pipeline.

## Performance

Discovery should avoid excessive filesystem work.

Possible optimizations include:

* Directory caching
* Incremental updates
* Event-driven watching
* Debouncing
* Lazy loading
* Validation caching

## Design Goal

Resource discovery should make customization feel native to Conduit.

The principle is:

> Put a supported resource in the right directory, and Conduit should know how to find it.
