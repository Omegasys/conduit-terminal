# Conduit Sandboxing

## Overview

Sandboxing provides an additional security boundary between Conduit components and the host system.

Sandboxing is especially important for:

* Plugins
* External helper processes
* Untrusted integrations
* Protocol decoders
* Graphics decoders
* Preview components

The sandbox architecture lives under:

```text
src/security/
└── sandbox/
```

## Goals

The sandbox system should:

* Minimize privileges.
* Restrict filesystem access.
* Restrict network access where appropriate.
* Limit process capabilities.
* Limit resource consumption.
* Isolate failures.
* Provide explicit permission boundaries.
* Integrate with Linux security mechanisms where available.

## Platform Integration

Conduit should use appropriate host security mechanisms rather than attempting to implement a complete operating-system sandbox itself.

Possible Linux mechanisms include:

* Namespaces
* Seccomp
* Capability restrictions
* cgroups
* Landlock
* AppArmor
* SELinux
* Bubblewrap-style isolation where appropriate

Availability should depend on the host system.

## Sandbox Levels

Conduit may expose several sandbox levels:

* None
* Basic
* Restricted
* Strong
* Custom

The exact capabilities of each level should be defined by the configuration schema.

## Filesystem Access

Sandboxed components should receive only the filesystem access they require.

Access may be:

* Read-only.
* Read-write.
* Path-specific.
* Temporary.
* Completely disabled.

A plugin should not automatically receive access to the user's entire home directory.

## Network Access

Network access should be independently controllable.

Possible policies include:

* No network.
* Local network only.
* Specific destinations.
* Normal network access.

Plugins should declare network requirements before receiving access.

## Process Execution

Sandboxed components should not automatically be allowed to launch arbitrary processes.

Process execution should require an appropriate permission.

When allowed, Conduit should consider:

* Executable path.
* Arguments.
* Environment.
* Working directory.
* Resource limits.

## Resource Limits

Sandboxed components should be subject to resource limits where appropriate.

Potential limits include:

* Memory.
* CPU.
* Number of processes.
* File descriptors.
* Disk usage.
* Network traffic.
* Runtime duration.

## Plugin Sandboxing

Plugins should be sandbox candidates by default.

A plugin should receive only the capabilities granted by its permission policy.

```text
Plugin
  ↓
Permission request
  ↓
Security manager
  ↓
Sandbox policy
  ↓
Restricted plugin process
```

## Protocol Decoders

Resource-intensive decoders such as terminal graphics decoders may optionally execute in restricted helper processes.

This can reduce the impact of a decoder vulnerability.

## Failure Handling

If sandbox initialization fails for a component requiring sandboxing:

* The component should not silently run without its sandbox.
* The failure should be reported.
* The component should remain disabled until the user changes the policy or the sandbox becomes available.

## Compatibility

Sandbox availability varies across Linux distributions and security configurations.

Conduit should detect available mechanisms and report them through diagnostics.

The absence of one mechanism should not automatically be treated as a fatal error if another appropriate mechanism can provide the required boundary.

## Configuration

Sandbox policies should be managed through the central configuration system.

Changes should use the live-reload system where possible.

Changes that require process recreation should restart only the sandboxed component.

## Testing

Sandbox testing should verify:

* Filesystem restrictions.
* Network restrictions.
* Process restrictions.
* Resource limits.
* Permission enforcement.
* Failure behavior.
* Escape attempts.
* Policy changes.

## Design Principle

Sandboxing should reduce the amount of trust required between Conduit components.

A component should receive the smallest practical set of capabilities necessary to perform its job.
