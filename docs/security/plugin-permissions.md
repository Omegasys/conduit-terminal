# Conduit Plugin Permissions

## Overview

Conduit plugins extend application functionality while creating an additional trust boundary.

A plugin should never receive unrestricted access to Conduit or the host system simply because it is installed.

Plugin permissions define what a plugin is allowed to access.

## Permission Architecture

The permission flow should be:

```text
Plugin manifest
      ↓
Requested permissions
      ↓
Validation
      ↓
User / policy decision
      ↓
Granted permissions
      ↓
Plugin runtime
```

## Permission Categories

Permissions may include:

* Terminal access.
* Session access.
* Workspace access.
* Configuration access.
* Theme access.
* Filesystem access.
* Network access.
* Process execution.
* Clipboard access.
* Notifications.
* Remote connections.
* System integration.
* Plugin management.

Permissions should be as granular as practical.

## Manifest

Plugins should declare their requirements in a manifest.

For example:

```toml
[plugin]
name = "example-plugin"
version = "1.0.0"

[permissions]
terminal = true
filesystem = "read-only"
network = false
clipboard = false
```

The exact manifest schema should be defined by the plugin system.

## Least Privilege

Plugins should receive only the permissions they need.

A plugin that provides a theme-management feature should not automatically receive:

* Network access.
* Shell execution.
* Clipboard access.
* Arbitrary filesystem write access.

## Permission States

A permission may have states such as:

* Denied.
* Granted.
* Granted for session.
* Granted for workspace.
* Granted permanently.
* Prompt.

## Security Profiles

Security profiles can provide global plugin policies.

For example, a hardened profile may prevent plugins from requesting network access regardless of individual plugin configuration.

## Permission Changes

Permission changes should be applied through the configuration engine.

Where possible:

* Restricting permissions should happen immediately.
* Expanding permissions should require appropriate authorization.
* Components should be restarted only when necessary.

## Sandboxing

Permissions should complement sandboxing.

```text
Permission policy
      +
Sandbox policy
      ↓
Effective plugin capability
```

Granting a permission should not automatically override operating-system restrictions.

## Plugin API

The plugin API should expose controlled interfaces rather than direct internal structures.

This allows Conduit to enforce permissions at API boundaries.

For example:

```text
Plugin
  ↓
Plugin API
  ↓
Permission check
  ↓
Conduit subsystem
```

## Failed Permission Checks

When a plugin requests a denied operation:

* The operation should fail safely.
* The plugin should receive an appropriate error.
* The event should optionally appear in diagnostics.
* The plugin should not be able to bypass the permission system.

## Plugin Installation

Installing a plugin should not automatically mean granting all requested permissions.

Conduit should present requested capabilities before enabling a plugin.

## Plugin Updates

Permission requirements should be reevaluated when a plugin is updated.

A new version requesting additional capabilities should not silently inherit permission for those capabilities.

## Logging

Plugin security events may include:

* Permission requested.
* Permission granted.
* Permission denied.
* Permission changed.
* Sandbox failure.
* Plugin termination.

Sensitive plugin data should not be logged unnecessarily.

## Testing

Tests should cover:

* Permission requests.
* Permission denial.
* Permission inheritance.
* Permission changes.
* Sandbox interaction.
* Plugin updates.
* API enforcement.
* Attempted permission bypasses.

## Design Principle

Plugins should be treated as extensions with explicitly bounded authority.

Installation gives a plugin the opportunity to request capabilities, not automatic ownership of them.
