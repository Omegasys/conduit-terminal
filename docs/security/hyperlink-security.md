# Conduit Hyperlink Security

## Overview

Terminal applications can emit hyperlinks through terminal control sequences.

While hyperlinks are useful, automatically opening a link can allow untrusted terminal output to trigger external applications or navigate the user to unintended destinations.

Conduit therefore treats terminal-generated hyperlinks as untrusted.

## Security Goals

Hyperlink security should:

* Clearly identify terminal-generated links.
* Prevent unintended automatic navigation.
* Support user confirmation.
* Integrate with the system browser safely.
* Protect against malformed or misleading links.
* Work consistently across supported protocols.

## Link Representation

Links should be represented separately from ordinary terminal text.

The terminal cell model may store:

* URI.
* Display range.
* Link identifier.
* Source session.
* Security state.

## Link Sources

Hyperlinks may originate from:

* OSC sequences.
* xterm extensions.
* Kitty-compatible mechanisms.
* iTerm2-compatible mechanisms.
* Automatically detected URLs.

Conduit should normalize these into a common hyperlink representation.

## Opening Links

Conduit should not assume that a terminal-provided link is trustworthy.

Depending on policy, opening may:

* Be disabled.
* Require confirmation.
* Open normally.
* Be allowed only for trusted sessions.

## URI Validation

Before opening a hyperlink, Conduit should validate:

* URI syntax.
* Scheme.
* Host representation.
* Potentially dangerous control characters.
* Encoded characters.
* Length limits.

Special schemes should receive additional policy handling.

## Supported Schemes

Security policy should distinguish ordinary web links from potentially sensitive schemes.

Examples include:

* `http`
* `https`
* `file`
* `ssh`
* `mailto`
* Custom application schemes

Users should be able to configure which schemes may be opened.

## Displayed Text vs Destination

The visible text of a hyperlink should not be treated as proof of its destination.

Conduit should provide a way to inspect the actual destination before opening it.

## Remote Sessions

Links originating from remote sessions should remain subject to the same host-side security policy.

A remote system should not automatically gain permission to open arbitrary local resources.

## Link Context

Conduit may expose additional context before opening a link:

* Source session.
* Remote/local status.
* Actual destination.
* Protocol.
* Workspace.
* Security profile.

## Logging

Security diagnostics may record hyperlink events without unnecessarily recording sensitive URL parameters.

Authentication tokens or credentials embedded in URLs should be redacted where possible.

## Live Configuration

Hyperlink policies should support live changes.

A newly restrictive policy should immediately affect future link operations.

## Testing

Tests should cover:

* Valid links.
* Invalid links.
* Dangerous schemes.
* Remote links.
* Encoded URLs.
* Malformed control sequences.
* Confirmation behavior.
* Policy changes.
* Destination inspection.

## Design Principle

A terminal hyperlink is a request from terminal output, not an instruction from the user.

Conduit should therefore preserve the user's control over whether and where terminal-generated links are opened.
