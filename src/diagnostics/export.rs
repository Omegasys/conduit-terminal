use std::fmt::Write;

use super::status::{
    DiagnosticHealth,
    DiagnosticsSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticExportFormat {
    Text,
    Json,
}

#[derive(Debug, Clone)]
pub struct DiagnosticReport {
    pub generated_at: std::time::SystemTime,
    pub snapshot: DiagnosticsSnapshot,
}

impl DiagnosticReport {
    pub fn new(snapshot: DiagnosticsSnapshot) -> Self {
        Self {
            generated_at: std::time::SystemTime::now(),
            snapshot,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DiagnosticExporter;

impl DiagnosticExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export(
        &self,
        report: &DiagnosticReport,
        format: DiagnosticExportFormat,
    ) -> String {
        match format {
            DiagnosticExportFormat::Text => {
                self.export_text(report)
            }
            DiagnosticExportFormat::Json => {
                self.export_json(report)
            }
        }
    }

    fn export_text(
        &self,
        report: &DiagnosticReport,
    ) -> String {
        let snapshot = &report.snapshot;
        let mut output = String::new();

        let _ = writeln!(
            output,
            "Conduit Diagnostic Report"
        );
        let _ = writeln!(
            output,
            "========================="
        );
        let _ = writeln!(
            output,
            "Overall health: {}",
            health_name(snapshot.overall_health())
        );
        let _ = writeln!(
            output,
            "Uptime: {:.3}s",
            snapshot.uptime.as_secs_f64()
        );
        let _ = writeln!(output);

        let _ = writeln!(output, "[Performance]");
        let _ = writeln!(
            output,
            "Frames: {}",
            snapshot.performance.frame_count()
        );
        let _ = writeln!(
            output,
            "Dropped frames: {}",
            snapshot.performance.dropped_frames()
        );
        let _ = writeln!(
            output,
            "Average frame time: {:?}",
            snapshot.performance.average_frame_time()
        );
        let _ = writeln!(
            output,
            "Estimated FPS: {:?}",
            snapshot.performance.estimated_fps()
        );
        let _ = writeln!(output);

        let _ = writeln!(output, "[Renderer]");
        let _ = writeln!(
            output,
            "Backend: {}",
            snapshot.renderer.backend().unwrap_or("unknown")
        );
        let _ = writeln!(
            output,
            "Initialized: {}",
            snapshot.renderer.initialized()
        );
        let _ = writeln!(
            output,
            "Hardware acceleration: {}",
            snapshot.renderer.hardware_accelerated()
        );
        let _ = writeln!(
            output,
            "Draw calls: {}",
            snapshot.renderer.draw_calls()
        );
        let _ = writeln!(
            output,
            "Textures: {}",
            snapshot.renderer.texture_count()
        );
        let _ = writeln!(output);

        let _ = writeln!(output, "[Protocols]");
        let _ = writeln!(
            output,
            "Processed sequences: {}",
            snapshot.protocols.processed_sequences()
        );
        let _ = writeln!(
            output,
            "Rejected sequences: {}",
            snapshot.protocols.rejected_sequences()
        );
        let _ = writeln!(
            output,
            "Malformed sequences: {}",
            snapshot.protocols.malformed_sequences()
        );
        let _ = writeln!(output);

        let _ = writeln!(output, "[Memory]");
        write_optional(
            &mut output,
            "Process bytes",
            snapshot.memory.process_bytes(),
        );
        write_optional(
            &mut output,
            "Resident bytes",
            snapshot.memory.resident_bytes(),
        );
        write_optional(
            &mut output,
            "Terminal buffer bytes",
            snapshot.memory.terminal_buffer_bytes(),
        );
        write_optional(
            &mut output,
            "Scrollback bytes",
            snapshot.memory.scrollback_bytes(),
        );
        write_optional(
            &mut output,
            "Recording bytes",
            snapshot.memory.recording_bytes(),
        );
        write_optional(
            &mut output,
            "Texture bytes",
            snapshot.memory.texture_bytes(),
        );
        let _ = writeln!(output);

        let _ = writeln!(output, "[GPU]");
        let _ = writeln!(
            output,
            "Available: {}",
            snapshot.gpu.available()
        );
        let _ = writeln!(
            output,
            "Backend: {}",
            snapshot.gpu.backend().unwrap_or("unknown")
        );
        let _ = writeln!(
            output,
            "Adapter: {}",
            snapshot.gpu.adapter().unwrap_or("unknown")
        );
        let _ = writeln!(output);

        let _ = writeln!(output, "[PTY]");
        let _ = writeln!(
            output,
            "Active: {}",
            snapshot.pty.active()
        );
        let _ = writeln!(
            output,
            "Created: {}",
            snapshot.pty.created()
        );
        let _ = writeln!(
            output,
            "Closed: {}",
            snapshot.pty.closed()
        );
        let _ = writeln!(
            output,
            "Failed: {}",
            snapshot.pty.failed()
        );
        let _ = writeln!(output);

        let _ = writeln!(output, "[Statuses]");

        for status in &snapshot.statuses {
            let _ = writeln!(
                output,
                "{}: {} - {}",
                status.name,
                health_name(status.health),
                status.message
            );
        }

        output
    }

    fn export_json(
        &self,
        report: &DiagnosticReport,
    ) -> String {
        let snapshot = &report.snapshot;

        format!(
            concat!(
                "{{",
                "\"health\":\"{}\",",
                "\"uptime_seconds\":{},",
                "\"performance\":{{",
                "\"frames\":{},",
                "\"dropped_frames\":{},",
                "\"commands\":{},",
                "\"events\":{},",
                "\"estimated_fps\":{}",
                "}},",
                "\"renderer\":{{",
                "\"backend\":\"{}\",",
                "\"initialized\":{},",
                "\"hardware_accelerated\":{},",
                "\"draw_calls\":{},",
                "\"textures\":{}",
                "}},",
                "\"protocols\":{{",
                "\"processed\":{},",
                "\"rejected\":{},",
                "\"malformed\":{}",
                "}},",
                "\"plugins\":{{",
                "\"discovered\":{},",
                "\"loaded\":{},",
                "\"active\":{},",
                "\"failed\":{}",
                "}},",
                "\"resources\":{{",
                "\"loaded\":{},",
                "\"active\":{},",
                "\"failed\":{},",
                "\"missing\":{}",
                "}},",
                "\"pty\":{{",
                "\"active\":{},",
                "\"created\":{},",
                "\"closed\":{},",
                "\"failed\":{},",
                "\"bytes_read\":{},",
                "\"bytes_written\":{}",
                "}}",
                "}}"
            ),
            health_name(snapshot.overall_health()),
            snapshot.uptime.as_secs_f64(),
            snapshot.performance.frame_count(),
            snapshot.performance.dropped_frames(),
            snapshot.performance.command_count(),
            snapshot.performance.event_count(),
            snapshot
                .performance
                .estimated_fps()
                .map(|value| value.to_string())
                .unwrap_or_else(|| "null".to_owned()),
            escape(snapshot.renderer.backend().unwrap_or("unknown")),
            snapshot.renderer.initialized(),
            snapshot.renderer.hardware_accelerated(),
            snapshot.renderer.draw_calls(),
            snapshot.renderer.texture_count(),
            snapshot.protocols.processed_sequences(),
            snapshot.protocols.rejected_sequences(),
            snapshot.protocols.malformed_sequences(),
            snapshot.plugins.discovered(),
            snapshot.plugins.loaded(),
            snapshot.plugins.active(),
            snapshot.plugins.failed(),
            snapshot.resources.loaded(),
            snapshot.resources.active(),
            snapshot.resources.failed(),
            snapshot.resources.missing(),
            snapshot.pty.active(),
            snapshot.pty.created(),
            snapshot.pty.closed(),
            snapshot.pty.failed(),
            snapshot.pty.bytes_read(),
            snapshot.pty.bytes_written(),
        )
    }
}

fn write_optional(
    output: &mut String,
    name: &str,
    value: Option<u64>,
) {
    let _ = writeln!(
        output,
        "{name}: {}",
        value
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown".to_owned())
    );
}

fn health_name(health: DiagnosticHealth) -> &'static str {
    match health {
        DiagnosticHealth::Healthy => "healthy",
        DiagnosticHealth::Warning => "warning",
        DiagnosticHealth::Degraded => "degraded",
        DiagnosticHealth::Error => "error",
        DiagnosticHealth::Unknown => "unknown",
    }
}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
