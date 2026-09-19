# Conduit History System

## Overview

Conduit distinguishes between **shell command history** and **Conduit activity metadata**.

Conduit should integrate with existing shell history systems rather than attempting to replace them.

This preserves normal shell behavior while allowing Conduit to provide a richer history interface.

## History Architecture

The history system can be divided into several layers:

```text
Shell History
      ↓
Shell Integration
      ↓
Conduit History Interface
      ↓
Optional Metadata Storage
      ↓
Search / Command Palette / UI
```

The command subsystem provides history-related interaction while a dedicated history subsystem can handle storage and indexing.

## Native Shell History

Each shell remains responsible for its own native history.

Examples include:

```text
Bash
~/.bash_history
```

Zsh, Fish, and PowerShell use their own history mechanisms and configuration.

Conduit should not silently replace or relocate those files.

## Shell Integration

Conduit can observe command boundaries through shell integration.

Possible information includes:

* Command start
* Command completion
* Exit status
* Duration
* Working directory
* Session
* Timestamp

The exact information available depends on the shell and integration mechanism.

## Conduit History

Conduit can maintain an optional metadata database or indexed history layer.

Possible storage location:

```text
~/.local/share/conduit/history/
```

This layer can associate commands with Conduit-specific context.

Examples include:

* Session
* Workspace
* Tab
* Pane
* Working directory
* Exit status
* Duration
* Timestamp
* Shell
* Connection type

## History Records

A Conduit history record may conceptually contain:

```text
Command
Timestamp
Session ID
Workspace ID
Pane ID
Working Directory
Shell
Exit Status
Duration
Connection Type
Privacy Classification
```

Sensitive command content should not automatically be retained if the user has disabled command recording or configured redaction.

## Privacy

Command history can contain highly sensitive information.

Conduit should therefore provide controls for:

* Disable Conduit history
* Disable command content storage
* Redact sensitive commands
* Exclude specific directories
* Exclude specific command patterns
* Clear history
* Set retention periods
* Encrypt history where supported
* Prevent history from being captured in recordings

## Sensitive Commands

Conduit should avoid storing secrets contained in commands.

Potential examples include:

* Passwords
* API tokens
* Access tokens
* Private keys
* Authentication credentials

Automatic detection can provide an additional safeguard, but users should not have to rely exclusively on automatic detection.

## History Search

The history system integrates with the search subsystem.

Users should be able to search by:

* Command
* Shell
* Directory
* Workspace
* Session
* Date
* Exit status
* Duration
* Connection type

Search may support:

* Plain text
* Case sensitivity
* Regex
* Whole-word matching
* Filters

## Command Palette

The command palette can provide history-aware command lookup.

For example:

```text
Recent Commands
Frequent Commands
Commands from Workspace
Commands from Session
Failed Commands
Commands from Directory
```

These are presentation and query modes rather than separate history databases.

## GUI History

The GUI can provide a dedicated history interface.

Possible features include:

* Search
* Filtering
* Sorting
* Copy command
* Re-run command
* Open originating session
* Open originating workspace
* Show metadata
* Delete entry
* Clear history

## TUI History

The TUI should provide equivalent keyboard-driven functionality.

## CLI History

Example commands:

```text
conduit history
conduit history search <query>
conduit history show <id>
conduit history clear
conduit history export <file>
```

## Import and Export

The history subsystem can support importing and exporting supported formats.

Export should allow users to preserve their data independently of Conduit.

Import should never overwrite native shell history without explicit user action.

## Indexing

Large history collections should use an index to make searches efficient.

The indexing layer may maintain:

* Command indexes
* Timestamp indexes
* Workspace indexes
* Session indexes
* Directory indexes
* Exit-status indexes

## Retention

Users should be able to configure retention.

Possible policies include:

* Keep everything
* Keep recent history
* Keep for a number of days
* Keep a maximum number of records
* Disable persistent Conduit history

## History and Recording

Terminal recordings and command history are separate systems.

A recording may contain terminal output that includes commands, while history contains structured command metadata.

The two systems should not automatically assume identical retention policies.

## History and Workspaces

Workspace-aware history can help users locate commands within a particular environment.

For example:

```text
Development
    ↓
History
    ↓
Commands associated with Development
```

Switching workspaces should not require copying or moving shell history.

## History and Sessions

A command should be associated with its originating session when possible.

This allows users to answer questions such as:

* Where did I run this?
* Which workspace contained it?
* What directory was I in?
* Did the command succeed?
* How long did it take?

## History Deletion

Deletion should distinguish between:

* Removing Conduit metadata
* Removing indexed records
* Removing native shell history

Conduit should clearly identify which layer is being modified.

## Security

History storage should follow Conduit's security policy.

Files containing command history should use appropriate filesystem permissions.

Sensitive information should not appear in diagnostics, Flow View, crash reports, or plugin APIs unless explicitly authorized.

## Design Goal

The history system should enrich the terminal experience without taking ownership away from the shell.

The principle is:

> The shell owns shell history. Conduit provides context, search, and integration around it.
