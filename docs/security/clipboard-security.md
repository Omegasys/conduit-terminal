# Conduit Clipboard Security

## Overview

The clipboard is a sensitive boundary between terminal applications and the user's desktop.

Terminal applications may attempt to:

* Read clipboard contents.
* Write clipboard contents.
* Replace clipboard data.
* Trigger clipboard-related terminal sequences.

Conduit therefore provides explicit clipboard security controls.

## Security Goals

Clipboard security should:

* Prevent unauthorized clipboard reads.
* Make sensitive clipboard transfers visible to the user.
* Protect clipboard contents from malicious terminal applications.
* Support legitimate copy and paste workflows.
* Integrate with terminal protocol features.
* Support security profiles.

## Clipboard Directions

Conduit should distinguish between:

* Terminal → clipboard
* Clipboard → terminal
* Terminal application → clipboard
* Host application → terminal

These operations should not necessarily share the same permission.

## Clipboard Reads

Reading clipboard contents is more sensitive than merely receiving user input.

Conduit should provide configurable policies such as:

* Disabled.
* Ask every time.
* Allow for trusted sessions.
* Allow.

The default policy should avoid silently exposing clipboard contents to terminal applications.

## Clipboard Writes

Terminal applications may request that Conduit place data into the clipboard.

Conduit should allow users to control this separately from clipboard reads.

Possible policies include:

* Disabled.
* Ask.
* Allow.

## Paste Protection

Conduit should detect potentially dangerous paste operations.

Features may include:

* Multiline paste warnings.
* Control-character detection.
* Shell-sensitive character detection.
* Paste preview.
* Confirmation dialogs.
* Configurable trusted applications.

## Sensitive Data

Conduit should avoid storing clipboard contents unnecessarily.

If clipboard history is enabled, users should be able to configure:

* Retention.
* Maximum entries.
* Encryption where available.
* Automatic deletion.
* Sensitive-content exclusion.

## Clipboard History

Clipboard history should remain separate from shell history.

The clipboard subsystem should never automatically insert clipboard contents into command history.

## Protocol Integration

Clipboard-related terminal protocols should route through the clipboard security manager.

```text
Terminal sequence
      ↓
Protocol parser
      ↓
Clipboard request
      ↓
Security policy
      ↓
Allow / deny / confirm
      ↓
Host clipboard
```

## Remote Sessions

Remote sessions require additional care.

A remote application requesting clipboard access should not automatically receive unrestricted access to the local clipboard.

Remote-session clipboard policy should be independently configurable.

## Logging

Clipboard contents should never be written to ordinary diagnostic logs.

Security logs should record the event without recording the sensitive data itself.

## Live Configuration

Clipboard policies should support live changes whenever possible.

A policy becoming more restrictive should take effect immediately.

## Testing

Tests should cover:

* Clipboard reads.
* Clipboard writes.
* Remote clipboard requests.
* Multiline paste.
* Control characters.
* Permission prompts.
* Denials.
* Security profile changes.
* Clipboard history behavior.

## Design Principle

The clipboard should be treated as a sensitive host resource.

Terminal applications should receive only the clipboard access explicitly permitted by the user's security policy.
