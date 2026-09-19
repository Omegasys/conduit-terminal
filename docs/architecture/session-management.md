# Conduit Session Management

## Overview

A Conduit session represents an active terminal environment and the process or connection associated with it.

Sessions are independent from windows, tabs, and panes.

This separation allows a session to move between panes, tabs, workspaces, or windows without changing the underlying terminal process.

## Session Architecture

Session functionality is primarily implemented through:

```text
src/core/session.rs
src/core/process.rs
src/core/pty.rs
src/core/lifecycle.rs
src/remote/
src/connections/
src/workspaces/
src/tabs/
src/panes/
```

## Session Model

A session can contain:

* Session identifier
* Terminal state
* PTY
* Shell or remote process
* Environment
* Working directory
* Dimensions
* Connection information
* Command state
* Exit status
* Metadata
* Lifecycle state

## Local Sessions

A local session normally consists of:

```text
Conduit
   ↓
PTY
   ↓
Shell
   ↓
Commands / Programs
```

The PTY provides the terminal interface between Conduit and the shell.

## Remote Sessions

Remote sessions use the connection subsystem.

Examples include:

* SSH
* Serial
* Container shells
* Other supported remote connections

The session abstraction should remain consistent regardless of where the process is running.

## Session Lifecycle

A typical session lifecycle is:

```text
Create
  ↓
Initialize
  ↓
Start PTY / Connection
  ↓
Start Shell / Remote Process
  ↓
Running
  ↓
Exit / Disconnect
  ↓
Exited
  ↓
Close / Restore / Reconnect
```

## Session State

Sessions should expose their current state to the rest of Conduit.

Possible states include:

* Creating
* Starting
* Running
* Suspended
* Disconnected
* Reconnecting
* Exited
* Failed
* Closing
* Closed

State changes are published through the event bus.

## PTY Management

The PTY layer handles:

* PTY creation
* Input
* Output
* Resize
* Signal delivery
* Process association
* Cleanup

The PTY implementation should remain independent of the GUI.

## Process Management

Process management handles:

* Process creation
* Environment setup
* Working directory
* Signals
* Exit status
* Process termination
* Child-process cleanup

Conduit should avoid terminating a shell simply because a UI component is recreated.

## Session Dimensions

Terminal dimensions are determined by the visible terminal surface.

When a pane changes size:

```text
Pane Resize
   ↓
Terminal Resize Event
   ↓
PTY Resize
   ↓
Shell / Application Receives SIGWINCH
```

The terminal state and process should remain the same.

## Session and Pane Separation

A session is not a pane.

A pane is a view of a session.

This allows the architecture to support future functionality such as:

* Moving sessions between panes
* Reattaching sessions
* Multiple views
* Workspace switching
* Session persistence

Whether multiple simultaneous views of the same session are permitted should be controlled by session policy.

## Session Restoration

Conduit can store session metadata for restoration.

Restorable information may include:

* Shell
* Working directory
* Environment configuration
* Workspace
* Tab
* Pane layout
* Profile
* Theme
* Connection profile

Restoring an active process is more complex than restoring its metadata.

Conduit should distinguish between:

```text
Restore Layout
```

and:

```text
Restore Running Process
```

The former is generally straightforward; the latter depends on the process and platform.

## Remote Reconnection

Remote sessions may support controlled reconnection.

A disconnected SSH session can transition into:

```text
Disconnected
    ↓
Reconnect Requested
    ↓
Authentication
    ↓
Connection Established
    ↓
Session Recreated
```

Credentials should never be stored in ordinary session metadata unless explicitly supported by a secure credential mechanism.

## Session Metadata

Conduit may maintain metadata under:

```text
~/.local/share/conduit/sessions/
```

Metadata can include:

* Creation time
* Last activity
* Exit status
* Working directory
* Shell
* Connection type
* Host identifier
* Workspace association
* Duration

Sensitive information should be excluded or redacted according to the security configuration.

## Session Events

Important events include:

* SessionCreated
* SessionStarted
* SessionOutput
* SessionInput
* SessionResized
* SessionActivity
* SessionDisconnected
* SessionReconnected
* SessionExited
* SessionClosed
* SessionError

These events are consumed by UI, diagnostics, history, workspace, notification, and Flow View systems.

## Session Security

Sessions should respect Conduit's security policy.

Controls may include:

* Environment restrictions
* Clipboard restrictions
* Filesystem restrictions
* Hyperlink restrictions
* Plugin access
* Remote authentication policy
* Recording policy
* Safe mode

## Session Recording

Sessions may optionally be recorded.

Recording can capture:

* Terminal output
* Input
* Timing
* Window dimensions
* Metadata

Recording should be explicitly configurable and should provide privacy controls.

## Session and History Integration

Shell history remains owned by the shell.

Conduit can integrate with:

* Bash history
* Zsh history
* Fish history
* PowerShell history

Conduit can additionally maintain metadata about commands and sessions without replacing the shell's native history mechanism.

## Session Closure

Closing a session should distinguish between:

* Closing its visual representation
* Terminating its shell
* Disconnecting a remote connection
* Terminating child processes

Conduit should clearly communicate destructive actions.

## Design Goal

Sessions are the persistent execution layer beneath Conduit's windows, tabs, panes, and workspaces.

The architectural principle is:

> Views may change. The session should not have to.
