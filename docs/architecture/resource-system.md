# Conduit Resource System

The Conduit resource system provides automatic discovery and management of external resources such as themes, profiles, and workspaces.

The system is designed around a simple principle:

If Conduit knows how to load a resource, Conduit should be able to discover it automatically.

## Resource Types

The initial resource system supports:

* Themes
* Profiles
* Workspaces

The architecture can be extended to additional resource types later.

## Resource Directory

The resource implementation is located under:

```text
src/resources/
```

Important components include:

```text
manager.rs
discovery.rs
registry.rs
paths.rs
resource.rs
theme.rs
profile.rs
workspace.rs
manifest.rs
validation.rs
```

## Discovery

The discovery system searches configured resource directories.

For a theme, for example:

```text
Resource Directory
→ Find theme files
→ Parse TOML
→ Validate
→ Register
→ Notify subscribers
```

The user does not need to manually register every theme with Conduit.

## User Themes

A user can place a theme in:

```text
~/.config/conduit/themes/
```

For example:

```text
~/.config/conduit/themes/my-theme.toml
```

If the file is valid, Conduit can automatically make it available.

## User Profiles

Profiles can be stored in:

```text
~/.config/conduit/profiles/
```

For example:

```text
~/.config/conduit/profiles/development.toml
```

## User Workspaces

Workspaces can be stored in:

```text
~/.config/conduit/workspaces/
```

For example:

```text
~/.config/conduit/workspaces/server.toml
```

## System Resources

System-wide resources can be installed under:

```text
/usr/share/conduit/themes/
/usr/share/conduit/profiles/
/usr/share/conduit/workspaces/
```

Additional local application resources may be stored under:

```text
~/.local/share/conduit/
```

## Resource Precedence

When multiple resources have the same identifier, Conduit uses a defined precedence order.

A typical order is:

User
→ Local
→ System
→ Built-in

This allows a user to override a system resource without modifying the system installation.

## Registry

The registry maintains the set of currently available resources.

The registry can track:

* Resource identifier
* Resource type
* Source path
* Version
* Validation status
* Active state
* Metadata

The registry should represent the current state of discoverable resources.

## Resource Manager

The resource manager coordinates:

* Discovery
* Registration
* Loading
* Validation
* Removal
* Reloading
* Notifications

It acts as the main entry point for resource operations.

## Validation

Resources must be validated before they become active.

For example, a theme may need to provide valid:

* Color values
* Font definitions
* Cursor settings
* Appearance properties

Invalid resources should not replace currently active valid resources.

## Invalid Resources

If a resource cannot be parsed or validated, Conduit should:

* Reject the resource
* Preserve the previous valid state
* Report the error
* Make the error visible through diagnostics

An invalid theme should not cause the entire terminal to stop working.

## Live Discovery

The resource system can monitor resource directories.

For example:

```text
~/.config/conduit/themes/
```

can be watched continuously.

When a user creates:

```text
my-theme.toml
```

Conduit can detect the new file.

## Resource Changes

The resource watcher should detect:

* Created files
* Modified files
* Deleted files
* Renamed files
* Replaced files

The resource manager then determines what action is required.

## Theme Reload

For a changed theme:

Theme File
→ Detect Change
→ Parse
→ Validate
→ Update Registry
→ Theme Event
→ Appearance Update
→ Renderer Refresh

A full application restart should normally not be required.

## Profile Reload

For a changed profile:

Profile File
→ Detect Change
→ Parse
→ Validate
→ Update Registry
→ Profile Event
→ Configuration Engine
→ Affected Components

Only affected settings should be changed.

## Workspace Reload

Workspace changes can be applied according to the workspace's current state.

If the workspace is active, Conduit can determine whether the change can be applied incrementally.

If the workspace is inactive, the updated version can simply be loaded the next time it is opened.

## Resource Removal

If an unused resource is deleted, Conduit removes it from the registry.

If the currently active resource is deleted, Conduit should avoid leaving the application in an invalid state.

For example, an active theme can remain in memory until another valid theme is selected.

## Resource Names

Resources should have stable identifiers.

A filename can provide the default identifier, but resource metadata may define a more explicit identifier when supported.

Identifiers should be deterministic and safe to use in configuration.

## Resource Manifests

The resource system can support manifests for resources that require metadata beyond the resource file itself.

A manifest can describe:

* Identifier
* Name
* Version
* Resource type
* Compatibility
* Author
* Description
* Dependencies

Simple resources should not require unnecessary metadata.

## Security

Resources are different from executable plugins.

A theme or profile is primarily configuration data and should therefore have a smaller trust boundary.

However, resource files can still influence application behavior and must be parsed safely.

Executable code should use the plugin system rather than the resource system.

## Resource and Plugin Separation

The distinction is:

Resources
→ Data / Configuration

Plugins
→ Executable Extensions

A theme should not become executable merely because it is automatically discovered.

## Resource Search Paths

The final set of resource search paths can be configured through Conduit's configuration engine.

Typical paths include:

```text
~/.config/conduit/themes/
~/.config/conduit/profiles/
~/.config/conduit/workspaces/

~/.local/share/conduit/themes/
~/.local/share/conduit/profiles/
~/.local/share/conduit/workspaces/

 /usr/share/conduit/themes/
 /usr/share/conduit/profiles/
 /usr/share/conduit/workspaces/
```

## Diagnostics

The resource system should expose diagnostic information such as:

* Number of discovered resources
* Resource paths
* Invalid resources
* Validation errors
* Active resources
* Resource reload status
* Resource watcher status

## CLI Integration

The CLI can provide commands such as:

```bash
conduit theme list
conduit theme inspect
conduit theme validate
conduit profile list
conduit workspace list
```

## GUI Integration

The GUI can display discovered resources directly in:

* Theme settings
* Profile settings
* Workspace manager
* Resource diagnostics

New resources should appear without requiring the user to manually edit an internal registry.

## TUI Integration

The TUI can expose the same resource registry and allow users to select themes, profiles, and workspaces.

## Design Principle

The resource system should make Conduit feel extensible without requiring users to modify source code.

Adding a valid resource should be as simple as:

Create File
→ Place File in Resource Directory
→ Conduit Discovers It
→ Validate
→ Use It
