# Conduit Security Overview

## Overview

Security is a cross-cutting part of Conduit rather than a single feature.

Conduit executes shells, displays untrusted terminal output, handles files and environment variables, manages plugins, connects to remote systems, interacts with the clipboard, renders graphics, and exposes configuration interfaces.

The security architecture therefore establishes boundaries between these components and provides configurable controls without requiring users to sacrifice normal terminal functionality.

The primary implementation lives under:

```text
src/security/
```

## Security Goals

Conduit should:

* Treat terminal output as untrusted input.
* Minimize unnecessary privileges.
* Isolate plugins from the terminal core.
* Protect clipboard contents.
* Control terminal hyperlinks.
* Restrict filesystem access where appropriate.
* Protect configuration and resource files.
* Provide explicit security profiles.
* Support a dedicated Safe Mode.
* Fail safely when security checks cannot be completed.
* Provide useful diagnostics without unnecessarily exposing sensitive data.
* Allow security settings to be changed through the central configuration system.
* Minimize the amount of application state that must be restarted when security settings change.

## Threat Model

Conduit should consider threats originating from:

* Terminal applications.
* Shell commands.
* Remote SSH sessions.
* Malicious scripts.
* Compromised repositories.
* Malicious terminal escape sequences.
* Untrusted plugins.
* Malicious or malformed themes.
* Workspace files.
* Configuration files.
* Clipboard content.
* Hyperlinks.
* Terminal graphics.
* Network-connected applications.
* Compromised dependencies.

The security model should not assume that a program is trustworthy simply because it is running locally.

## Trust Boundaries

Important trust boundaries include:

```text
User
  ↓
Conduit UI
  ↓
Conduit core
  ↓
Shell / application
  ↓
Terminal output

Plugin ── restricted API ──> Conduit

Remote host ── SSH ──> Conduit session

Terminal output ── security policy ──> Host resources
```

## Security Manager

The security manager should provide a centralized policy interface.

Responsibilities include:

* Security profiles
* Permission checks
* Clipboard policy
* Hyperlink policy
* Plugin permissions
* Output filtering
* Safe Mode
* Sandboxing integration
* Security events
* Audit information
* Policy changes

Other subsystems should consult the security manager instead of implementing independent security policies.

## Security Profiles

Conduit should support configurable profiles such as:

* Standard
* Hardened
* Paranoid
* Minimal
* Custom

Profiles should be configuration resources rather than hard-coded application modes.

Users should be able to create their own security profiles.

## Defense in Depth

Conduit should not rely on a single security mechanism.

For example, plugin security can combine:

1. Permission declaration.
2. Permission validation.
3. API-level restrictions.
4. Process isolation.
5. Resource limits.
6. Runtime monitoring.
7. Failure isolation.

A failure of one layer should not automatically remove all security boundaries.

## Secure Defaults

Default configuration should favor safe behavior while preserving normal terminal operation.

Security-sensitive operations should require additional user interaction when appropriate.

Examples include:

* Opening external hyperlinks.
* Reading sensitive clipboard data.
* Installing plugins.
* Granting elevated plugin permissions.
* Executing potentially dangerous integration actions.

## Logging

Security events should be available to the diagnostics system.

Logs should avoid unnecessarily recording:

* Passwords
* Private keys
* Clipboard contents
* Authentication tokens
* Full sensitive command lines

Sensitive values should be redacted.

## Live Security Changes

Security configuration should integrate with the live-reload architecture.

Changes should be applied immediately when technically safe.

If a security change requires a component restart, Conduit should restart only the affected component.

Security settings should never become weaker merely because a live reload fails.

## Failure Handling

When a security decision cannot be evaluated reliably, Conduit should prefer the more restrictive behavior.

Examples:

* Unknown plugin permission → deny.
* Invalid security profile → retain the previous valid profile.
* Invalid security configuration → reject the change.
* Failed sandbox initialization → do not start the restricted component.
* Invalid hyperlink policy → do not open the link automatically.

## Integration

Security integrates with:

* Terminal core
* Protocol parsers
* Clipboard
* Hyperlinks
* Plugins
* Filesystem
* Remote sessions
* Configuration
* Resources
* Renderer
* Graphics
* Shell integration
* Diagnostics
* Event bus
* Flow View
* Safe Mode

## Design Principle

Conduit's security architecture should make the safe path the easy path while allowing advanced users to deliberately configure additional capabilities.

Security should protect the user without unnecessarily preventing legitimate terminal workflows.
