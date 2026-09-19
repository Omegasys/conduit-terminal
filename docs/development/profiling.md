# Conduit Profiling

## Overview

Profiling provides detailed measurements of Conduit's runtime behavior.

Profiling should help developers identify performance bottlenecks without requiring assumptions about which component is responsible.

## Profiling Targets

Profiling can examine:

* CPU usage
* Memory usage
* Allocation behavior
* Rendering
* GPU activity
* Event processing
* Terminal parsing
* PTY throughput
* Plugin execution
* Startup
* Workspace switching

## Profiling Modes

Conduit can support several profiling modes.

### CPU Profiling

Measures time spent executing functions and components.

### Memory Profiling

Measures memory consumption and allocation behavior.

### Allocation Profiling

Identifies allocation-heavy code paths.

### Rendering Profiling

Measures:

* Frame time
* Draw calls
* Glyph operations
* Texture operations
* Dirty regions

### Event Profiling

Measures event frequency and processing time.

### Startup Profiling

Measures initialization stages and startup latency.

## Instrumentation

Internal components can expose profiling spans.

Examples:

* `pty.read`
* `protocol.parse`
* `terminal.update`
* `renderer.frame`
* `event.dispatch`
* `config.reload`
* `resource.validate`

Profiling identifiers should remain stable enough to compare measurements across development versions.

## Sampling

Sampling profiling should be preferred when possible for low-overhead measurements.

Sampling can identify frequently executing code without instrumenting every operation.

## Instrumented Profiling

Instrumented profiling can provide detailed measurements for specific operations.

It should be enabled selectively because instrumentation can affect performance.

## Event Tracing

Flow View can consume profiling information to show component timing.

For example:

`PTY Read → Parser → Terminal State → Renderer`

can display the time spent at each stage.

## Memory Profiling

Memory profiling should identify major allocations such as:

* Scrollback
* Glyph caches
* Image caches
* Workspace state
* Plugin state
* Resource data

Sensitive content should not be captured unnecessarily.

## Renderer Profiling

Renderer profiling should distinguish:

* CPU preparation
* GPU submission
* GPU execution when measurable
* Synchronization
* Frame presentation

## Terminal Parser Profiling

Parser profiling should measure:

* Bytes processed
* Sequences processed
* Processing time
* Error rates
* Graphics payload processing

Large or malformed inputs should remain subject to normal security limits during profiling.

## Plugin Profiling

Plugin execution can be measured separately from core application work.

This helps identify plugins that consume excessive resources.

Profiling should respect plugin sandbox boundaries.

## Comparative Profiling

Developers should be able to compare two builds or configurations.

Examples:

* Before optimization
* After optimization
* GPU renderer vs software renderer
* Different scrollback sizes
* Different protocol configurations

## Profiling Sessions

A profiling session can contain:

* Start time
* End time
* Build information
* Configuration identifiers
* Enabled profiling categories
* Collected measurements

Profiling sessions should be exportable for analysis.

## Overhead

Profiling overhead should be documented.

A profiler should not be assumed to represent normal runtime behavior perfectly.

High-detail profiling may change:

* CPU usage
* Memory usage
* Timing
* Event scheduling

## Privacy

Profiling should avoid collecting:

* Passwords
* Clipboard contents
* Command contents
* Authentication tokens
* Private environment values

Profiles should identify activity using metadata rather than raw sensitive content whenever possible.

## Performance Budgets

Developers can establish performance budgets for critical operations.

Examples:

* Startup time
* Input latency
* Frame time
* Parser throughput
* Memory usage

Budgets should be treated as engineering targets rather than absolute guarantees across all hardware.

## Profiling Commands

Example commands include:

`conduit profile start`

`conduit profile stop`

`conduit profile status`

`conduit profile export`

`conduit profile renderer`

`conduit profile terminal`

## Design Principles

Profiling should be:

* Low overhead when possible
* Targeted
* Repeatable
* Privacy-aware
* Security-aware
* Comparable
* Integrated with diagnostics

Profiling should help developers measure actual behavior rather than optimize based solely on assumptions.
