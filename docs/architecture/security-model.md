# Conduit Security Model

## Overview

Security is a cross-cutting architectural concern in Conduit.

The security model is designed around the assumption that terminal input, terminal output, plugins, remote systems, files, escape sequences, and external resources may be untrusted.

Security controls should therefore exist throughout the application rather than being isolated in a single security component.

## Security Architecture

Security functionality is primarily implemented under:

```text
src/security/
├── sandbox.rs
├── permissions.rs
├── clipboard.rs
├── hyperlinks.rs
├── escape_sequences.rs
├── plugins.rs
├── filesystem.rs
├── environment.rs
├── safe_mode.rs
├── hardened_mode.rs
├── audit.rs
└── policy.rs
```

The Security Center provides a user-facing interface for these controls.

## Security Principles

Conduit follows several principles:

* Least privilege
* Explicit permissions
* Isolation where practical
* Secure defaults
* Clear user visibility
* Fail safely
* Minimize unnecessary data retention
* Separate trusted configuration from untrusted content
* Avoid unnecessary privilege escalation
* Preserve terminal availability when a nonessential component fails

## Threat Model

Potential threats include:

* Malicious terminal escape sequences
* Malicious hyperlinks
* Clipboard attacks
* Malicious terminal graphics
* Untrusted plugins
* Malicious configuration files
* Compromised remote systems
* Malicious commands
* Environment-variable attacks
* Local filesystem attacks
* Process injection attempts
* Dependency vulnerabilities

Conduit does not attempt to make arbitrary shell commands safe.

Once a user intentionally executes a command, the command is subject to the permissions of the operating system and configured environment.

## Trust Boundaries

Important trust boundaries include:

```text
User
 ↓
Conduit UI
 ↓
Terminal Core
 ↓
Shell / Process
```

and:

```text
Remote System
 ↓
Connection
 ↓
Terminal Parser
 ↓
Terminal State
 ↓
Renderer
```

and:

```text
Plugin
 ↓
Plugin API
 ↓
Permission System
 ↓
Conduit Core
```

These boundaries should be explicit in the architecture.

## Escape Sequence Security

Terminal escape sequences can change terminal state and trigger special behavior.

Conduit should validate and safely process supported sequences.

Security-sensitive sequences include those related to:

* Clipboard
* Hyperlinks
* Window manipulation
* Titles
* Shell integration
* Graphics
* Device queries

Unsupported or malformed sequences should not be allowed to corrupt application state.

## Hyperlink Security

Terminal output may contain URLs.

Conduit should not automatically execute or open arbitrary links.

The hyperlink system should provide:

* URL parsing
* Scheme validation
* User confirmation where appropriate
* Dangerous-scheme restrictions
* Visual indication of clickable links

## Clipboard Security

Clipboard access is a significant trust boundary.

Conduit should provide protections for:

* Automatic clipboard writes
* Clipboard reads
* OSC clipboard sequences
* Large pastes
* Multiline pastes
* Sensitive clipboard contents

Users should be able to restrict terminal applications from interacting with the clipboard.

## Paste Protection

Multiline paste warnings can help prevent accidental execution of multiple commands.

Possible controls include:

* Warn before multiline paste
* Show pasted content
* Require confirmation
* Disable protection
* Apply protection by security profile

## Plugin Security

Plugins should operate according to explicit permissions.

Permissions may include:

* Filesystem
* Network
* Process execution
* Terminal access
* Clipboard
* Environment
* UI
* Workspace
* Session
* Secrets

The plugin system should not assume that installed code is trustworthy merely because it exists in a plugin directory.

## Filesystem Security

Conduit should avoid unnecessarily broad filesystem access.

Filesystem operations should use:

* Explicit paths
* Permission checks
* Safe path handling
* Symlink-aware policies where appropriate
* User confirmation for destructive actions

Plugins and integrations should receive only the filesystem access required for their function.

## Environment Security

Environment variables can contain sensitive information.

Conduit should avoid exposing the complete environment to:

* Plugins
* Diagnostics
* Logs
* Recordings
* Crash reports

unless explicitly authorized.

## Remote Security

Remote sessions should preserve the security properties of their underlying connection.

For SSH, Conduit should integrate with established host-key and authentication mechanisms rather than silently bypassing them.

Remote connection profiles should not store secrets in plaintext configuration.

## Profiles

Security profiles allow users to select predefined configurations.

Possible profiles include:

```text
Standard
Hardened
Paranoid
Minimal
Custom
```

These are configuration presets rather than guarantees of a particular security level.

Users should be able to inspect the individual controls that each profile changes.

## Safe Mode

Safe Mode should disable or restrict functionality that may introduce additional risk.

Possible restrictions include:

* Plugins
* Terminal graphics
* Automatic hyperlink actions
* Clipboard integration
* External integrations
* Experimental components

Safe Mode should remain useful as a diagnostic and recovery environment.

## Hardened Mode

Hardened Mode can enable stronger restrictions while preserving ordinary terminal functionality where possible.

Examples include:

* Restricted plugin permissions
* Stronger clipboard protections
* Reduced external integration
* Stricter hyperlink handling
* Restricted terminal graphics
* Additional audit information

## Permission Manager

The permission manager maintains effective permissions for security-sensitive components.

A permission may have states such as:

* Allowed
* Denied
* Prompt
* Restricted

Permission decisions should be visible through the Security Center.

## Security Events

Security-relevant events should be available through the event system.

Examples include:

* PermissionRequested
* PermissionGranted
* PermissionDenied
* SuspiciousEscapeSequence
* ClipboardBlocked
* HyperlinkBlocked
* PluginViolation
* SandboxFailure
* SecurityPolicyChanged

## Audit System

The audit subsystem can record security-relevant events.

Audit information should be configurable.

Possible controls include:

* Enable or disable auditing
* Retention
* Sensitive-data redaction
* Export
* Clear audit history

Audit logs should themselves be protected from unauthorized access.

## Diagnostics

Security information can appear in the diagnostics system.

Diagnostics should distinguish between:

* Configuration problems
* Permission failures
* Sandbox failures
* Plugin errors
* Security policy actions
* Terminal parsing errors

Sensitive information should be redacted before diagnostics are exported.

## Configuration Security

Configuration files can influence application behavior and should therefore be validated.

Conduit should:

1. Parse configuration.
2. Validate against the schema.
3. Check security-sensitive settings.
4. Apply permitted changes.
5. Roll back invalid changes.

A malformed configuration should not leave the application in an undefined state.

## Resource Security

Themes, profiles, and workspaces are data resources but may still contain dangerous or unexpected values.

Resource validation should prevent malformed resources from corrupting application state.

Resources should not automatically execute arbitrary code.

## Live Reload Security

Live reload introduces an additional trust boundary because files can change while Conduit is running.

Before applying a changed resource or configuration:

```text
File Change
   ↓
Detect
   ↓
Parse
   ↓
Validate
   ↓
Security Check
   ↓
Apply
```

Invalid or unsafe changes should be rejected without replacing the currently valid state.

## Failure Handling

Security failures should prefer containment over application-wide termination.

For example:

```text
Plugin Violation
      ↓
Disable Plugin
      ↓
Record Diagnostic
      ↓
Notify User
      ↓
Continue Conduit
```

A critical security failure may require a larger shutdown depending on the affected component.

## Privacy

Security and privacy overlap but are not identical.

Conduit should minimize unnecessary collection and retention of:

* Commands
* Environment variables
* Remote host information
* Clipboard contents
* Terminal recordings
* Diagnostic data
* Plugin data

Users should have control over persistent data where practical.

## Security Updates

The project should monitor:

* Rust dependencies
* Rendering libraries
* Protocol implementations
* Plugin APIs
* Packaging components
* Platform integrations

Security fixes should be documented through the project's security process.

## Design Goal

Conduit should provide strong security boundaries without preventing users from exercising control over their own terminal environment.

The central principle is:

> Every boundary should have an explicit trust model, and every privileged capability should have a clear reason to exist.
