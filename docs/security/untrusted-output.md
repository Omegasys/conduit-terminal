# Conduit Untrusted Terminal Output

## Overview

Everything received from a terminal application or remote session should be considered untrusted input.

This includes:

* Printable text.
* ANSI sequences.
* VT sequences.
* OSC commands.
* DCS sequences.
* Graphics protocols.
* Hyperlinks.
* Clipboard requests.
* Title changes.
* Notifications.
* Device-control requests.

Conduit must parse terminal output without assuming that the originating application is trustworthy.

## Security Boundary

The terminal output path should include explicit security controls:

```text
PTY
 ↓
Protocol parser
 ↓
Validation
 ↓
Security policy
 ↓
Terminal state / host action
```

Rendering ordinary text is generally different from performing a host-side action.

## Terminal Output vs Host Actions

Conduit should distinguish between:

* Rendering text.
* Changing terminal state.
* Requesting a host resource.
* Triggering an external application.
* Accessing user data.

Host-side actions should receive additional security processing.

## Escape Sequences

Escape sequences should be parsed according to known protocol grammars.

Conduit should not execute arbitrary data merely because it resembles a control sequence.

## Parser Limits

The parser should enforce limits on:

* Sequence length.
* Parameter count.
* String payload size.
* Nesting where applicable.
* Processing time.
* Graphics resources.

## Malformed Input

Malformed terminal output should not crash or permanently lock the parser.

The parser should:

1. Detect invalid state.
2. Recover where possible.
3. Discard unsafe data where necessary.
4. Return to a known parser state.
5. Record diagnostics when appropriate.

## Excessive Output

A malicious or malfunctioning process may generate enormous output.

Conduit should protect against:

* Memory exhaustion.
* Renderer overload.
* Scrollback exhaustion.
* Event flooding.
* Graphics resource exhaustion.
* Notification flooding.

## Terminal Titles

Applications may request title changes.

Title requests should remain subject to configuration and security policy.

## Clipboard Requests

Clipboard operations must pass through the clipboard security subsystem.

Terminal output must not directly access the host clipboard.

## Hyperlinks

Hyperlinks must pass through hyperlink validation and policy before external navigation.

## Notifications

Terminal-generated notifications should be rate-limited and policy-controlled.

## Graphics

Graphics data should pass through resource limits and decoder validation.

This applies to:

* Sixel.
* Kitty graphics.
* iTerm2 graphics.
* Other supported terminal graphics protocols.

## Remote Output

Output from SSH and other remote sessions should be treated with the same security model as local terminal output.

Remote origin should be recorded as session metadata where useful, but it should not automatically grant or deny a capability.

## Security Events

Security-relevant terminal events should be observable through the event bus.

Examples include:

* Blocked clipboard request.
* Blocked hyperlink.
* Oversized OSC payload.
* Graphics resource limit.
* Unsupported dangerous operation.
* Parser recovery.

## Diagnostics

Diagnostics should help users understand what happened without exposing sensitive terminal contents.

Logs should support redaction.

## Safe Failure

When Conduit cannot safely determine what a control sequence requests, it should avoid performing the potentially dangerous host-side action.

The terminal should continue operating whenever possible.

## Design Principle

Terminal output is data.

It may describe an action or request an action, but it must never automatically acquire the authority to perform that action simply by being emitted by a terminal application.
