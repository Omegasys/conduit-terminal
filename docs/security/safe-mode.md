# Conduit Safe Mode

## Overview

Safe Mode is a restricted operating mode designed to start Conduit with potentially problematic integrations disabled.

It provides a recovery mechanism when:

* A plugin crashes.
* A configuration is invalid.
* A theme causes rendering problems.
* A resource causes repeated reload failures.
* A graphics extension causes instability.
* A user needs to troubleshoot a security problem.

Safe Mode should be implemented as a controlled security and recovery configuration rather than a completely separate version of Conduit.

## Goals

Safe Mode should:

* Start with a minimal trusted configuration.
* Disable nonessential extensions.
* Prevent problematic plugins from loading.
* Reduce attack surface.
* Allow users to inspect and repair configuration.
* Preserve access to essential terminal functionality.
* Provide clear diagnostics explaining what was disabled.

## Safe Mode Configuration

Safe Mode should use a minimal configuration path and should not depend on potentially broken user configuration components.

The exact bootstrap configuration should be defined by the security architecture.

## Disabled Components

Depending on the Safe Mode policy, Conduit may disable:

* Third-party plugins.
* Automatic plugin loading.
* Terminal graphics.
* External integrations.
* Automatic hyperlink opening.
* Clipboard integration.
* Custom themes.
* Custom profiles.
* Workspace automation.
* Experimental features.

The terminal core should remain available.

## Theme Handling

Safe Mode should use a known-good built-in theme.

User-installed themes should not automatically load.

This prevents a broken or malicious theme resource from preventing Conduit from starting.

## Plugin Handling

Third-party plugins should not automatically load in Safe Mode.

Built-in security-critical components should remain available.

Plugin diagnostics should identify which plugins were skipped.

## Configuration Recovery

Safe Mode should provide access to configuration repair tools.

The configuration editor should be able to:

* Inspect invalid settings.
* Show validation errors.
* Restore defaults.
* Disable problematic sections.
* Repair resource references.
* Save corrected configuration.

## Resource Recovery

Safe Mode should prevent problematic resources from being automatically activated.

The resource manager should retain enough diagnostic information to identify the problematic resource.

## Renderer Recovery

If a rendering backend repeatedly fails, Safe Mode may use a conservative software rendering path where available.

The renderer should not repeatedly restart a known-broken component indefinitely.

## Security Restrictions

Safe Mode should apply a restrictive security policy.

Examples include:

* No untrusted plugins.
* Restricted clipboard.
* Restricted hyperlinks.
* Restricted graphics.
* Restricted external integrations.

## Command-Line Activation

Conduit should provide a command-line mechanism for entering Safe Mode.

The exact syntax should be defined by the CLI subsystem.

Safe Mode should also be usable automatically after repeated startup failures where configured.

## Recovery Flow

A typical recovery sequence is:

```text
Conduit startup
      ↓
Failure detected
      ↓
Safe Mode available
      ↓
Minimal configuration
      ↓
Problematic component disabled
      ↓
Conduit starts
      ↓
Diagnostics identify problem
      ↓
User repairs configuration
      ↓
Normal mode restored
```

## Crash Recovery

Conduit may detect repeated crashes associated with a particular component.

The recovery system should avoid repeatedly launching the same failing component without intervention.

Possible recovery actions include:

* Disable plugin.
* Disable theme.
* Disable renderer backend.
* Disable resource.
* Fall back to default configuration.

## Safe Mode Indicators

The UI should clearly indicate when Safe Mode is active.

The indicator should provide access to:

* Active restrictions.
* Disabled components.
* Diagnostics.
* Recovery tools.
* Option to return to normal mode.

## Live Reload

Safe Mode should not automatically become unrestricted merely because a configuration file changes.

Leaving Safe Mode should be an explicit operation.

## Security

Safe Mode itself should have minimal dependencies and a small trusted configuration surface.

The recovery mechanism should not require loading the component that caused the original failure.

## Testing

Safe Mode should be tested against:

* Invalid configuration.
* Broken themes.
* Broken plugins.
* Renderer failures.
* Resource validation failures.
* Repeated startup crashes.
* Invalid security profiles.
* Corrupted workspace data.
* Failed live reloads.

## Design Principle

Safe Mode is Conduit's recovery path.

It should provide enough functionality to diagnose and repair the application while minimizing the number of components that must be trusted during recovery.
