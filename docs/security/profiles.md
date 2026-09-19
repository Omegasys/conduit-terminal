# Conduit Security Profiles

## Overview

Security profiles provide reusable collections of security policies.

Profiles allow users to change the security posture of Conduit without manually changing every individual setting.

Security profiles integrate with the general Conduit profile and configuration systems.

## Built-In Profiles

Conduit may provide several built-in security profiles:

### Standard

Designed for ordinary terminal use.

It should provide normal terminal functionality while protecting sensitive host resources.

### Hardened

Provides stronger restrictions for untrusted workflows.

Possible restrictions include:

* More restrictive clipboard access.
* Stricter hyperlink handling.
* Reduced plugin permissions.
* More restrictive graphics limits.
* Additional confirmation prompts.

### Paranoid

Provides maximum practical isolation while retaining the terminal itself.

Possible restrictions include:

* Disabled clipboard integration.
* Disabled automatic hyperlinks.
* Disabled plugins.
* Disabled terminal graphics.
* Restricted notifications.
* Strong sandboxing.

### Minimal

Provides a small terminal feature set with optional integrations disabled.

### Custom

Allows users to configure individual security controls.

## Profile Structure

Security profiles should be configuration resources.

For example:

```toml
[security]
clipboard = "ask"
hyperlinks = "confirm"
plugins = "restricted"
graphics = "limited"
sandbox = "strong"
notifications = "restricted"
```

The actual options should be defined by the configuration schema.

## Profile Locations

Security profiles can be stored alongside other Conduit resources.

Possible locations include:

```text
~/.config/conduit/profiles/
~/.local/share/conduit/profiles/
/usr/share/conduit/profiles/
```

## Automatic Discovery

Valid security profiles should be automatically discovered.

The resource manager should:

1. Discover the file.
2. Parse it.
3. Validate it.
4. Register it.
5. Make it available to the security manager.

Invalid profiles should not replace active valid profiles.

## Live Changes

Security profile changes should be applied through the live configuration system.

Changes that can be safely applied immediately should not require a restart.

## Profile Switching

Switching profiles may affect:

* Plugins.
* Clipboard.
* Hyperlinks.
* Graphics.
* Notifications.
* Sandboxing.
* Remote sessions.
* Terminal protocol features.

Conduit should determine which components require reinitialization.

## Restrictive Changes

When switching to a more restrictive profile, restrictions should take effect as soon as possible.

Existing resources should be handled according to the policy.

For example, disabling graphics should prevent new graphics operations and may optionally remove existing graphics resources.

## Expansive Changes

Moving to a less restrictive profile should not silently grant previously denied capabilities to untrusted components.

Operations requiring authorization should still pass through their normal permission checks.

## Session-Specific Profiles

Users may assign security profiles to individual sessions.

This is particularly useful when using:

* Local shells.
* Development environments.
* Remote systems.
* Untrusted repositories.
* Temporary sessions.

## Workspace Profiles

A workspace can select a default security profile.

Workspace policy should remain subordinate to global restrictions where appropriate.

## Diagnostics

The diagnostics system should show:

* Active security profile.
* Profile source.
* Effective permissions.
* Sandbox availability.
* Restricted capabilities.
* Pending policy changes.

## Security Precedence

Conduit should calculate effective security policy from multiple sources.

A restrictive policy should not be accidentally overridden by a less restrictive lower-level configuration.

## Design Principle

Security profiles provide a convenient way to change Conduit's security posture while preserving explicit, inspectable policies.

The effective policy should always be visible to the user.
