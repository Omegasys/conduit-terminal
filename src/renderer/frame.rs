use std::time::{Duration, Instant};

use crate::colors::Rgba;

use super::cursor::Cursor;
use super::renderer::{
    BlendState,
    ClearOptions,
    Frame,
    RenderCommand,
    ScissorRect,
    Vertex,
    Viewport,
};

/// Timing information for a rendered frame.
#[derive(Debug, Clone, Copy)]
pub struct FrameTiming {
    started: Instant,
    cpu_duration: Duration,
    gpu_duration: Option<Duration>,
}

impl FrameTiming {
    pub fn new(started: Instant) -> Self {
        Self {
            started,
            cpu_duration: Duration::ZERO,
            gpu_duration: None,
        }
    }

    pub fn started(&self) -> Instant {
        self.started
    }

    pub fn cpu_duration(&self) -> Duration {
        self.cpu_duration
    }

    pub fn gpu_duration(&self) -> Option<Duration> {
        self.gpu_duration
    }

    pub fn finish_cpu(&mut self) {
        self.cpu_duration = self.started.elapsed();
    }

    pub fn set_gpu_duration(&mut self, duration: Duration) {
        self.gpu_duration = Some(duration);
    }
}

/// Per-frame statistics.
#[derive(Debug, Clone, Copy, Default)]
pub struct FrameStatistics {
    pub draw_calls: u64,
    pub vertices: u64,
    pub indices: u64,
    pub text_glyphs: u64,
    pub textures: u64,
}

impl FrameStatistics {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn record_draw(
        &mut self,
        vertices: usize,
        indices: usize,
    ) {
        self.draw_calls += 1;
        self.vertices += vertices as u64;
        self.indices += indices as u64;
    }

    pub fn record_glyphs(&mut self, count: usize) {
        self.text_glyphs += count as u64;
    }

    pub fn record_texture(&mut self) {
        self.textures += 1;
    }
}

/// High-level frame builder.
#[derive(Debug)]
pub struct RenderFrame {
    frame: Frame,
    timing: FrameTiming,
    statistics: FrameStatistics,
    finished: bool,
}

impl RenderFrame {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            frame: Frame::new(width, height),
            timing: FrameTiming::new(Instant::now()),
            statistics: FrameStatistics::default(),
            finished: false,
        }
    }

    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    pub fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }

    pub fn timing(&self) -> FrameTiming {
        self.timing
    }

    pub fn statistics(&self) -> FrameStatistics {
        self.statistics
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn clear(&mut self, color: Rgba) {
        self.frame
            .push(RenderCommand::Clear(ClearOptions::color(color)));
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.frame.push(RenderCommand::SetViewport(viewport));
    }

    pub fn set_scissor(&mut self, scissor: Option<ScissorRect>) {
        self.frame.push(RenderCommand::SetScissor(scissor));
    }

    pub fn set_blend(&mut self, blend: BlendState) {
        self.frame.push(RenderCommand::SetBlend(blend));
    }

    pub fn draw_vertices(
        &mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) {
        self.statistics
            .record_draw(vertices.len(), indices.len());

        self.frame.push(RenderCommand::DrawVertices {
            vertices,
            indices,
        });
    }

    pub fn draw_texture(
        &mut self,
        texture: super::renderer::TextureId,
        position: [f32; 2],
        size: [f32; 2],
        tint: Rgba,
    ) {
        self.statistics.record_texture();

        self.frame.push(RenderCommand::DrawTexture {
            texture,
            position,
            size,
            tint,
        });
    }

    pub fn draw_text<S>(
        &mut self,
        position: [f32; 2],
        text: S,
        color: Rgba,
        size: f32,
    ) where
        S: Into<String>,
    {
        let text = text.into();

        self.statistics.record_glyphs(text.chars().count());

        self.frame.push(RenderCommand::DrawText {
            position,
            text,
            color,
            size,
        });
    }

    pub fn draw_cursor(
        &mut self,
        cursor: &Cursor,
        cell_width: f32,
        cell_height: f32,
    ) {
        if !cursor.visible() {
            return;
        }

        let position = [
            cursor.position()[0] as f32 * cell_width,
            cursor.position()[1] as f32 * cell_height,
        ];

        let size = [
            cursor.size()[0] as f32 * cell_width,
            cursor.size()[1] as f32 * cell_height,
        ];

        let color = cursor.color();

        let vertices = vec![
            Vertex::new(position, color),
            Vertex::new(
                [position[0] + size[0], position[1]],
                color,
            ),
            Vertex::new(
                [position[0] + size[0], position[1] + size[1]],
                color,
            ),
            Vertex::new(
                [position[0], position[1] + size[1]],
                color,
            ),
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        self.draw_vertices(vertices, indices);
    }

    pub fn finish(&mut self) {
        if self.finished {
            return;
        }

        self.timing.finish_cpu();
        self.finished = true;
    }
}
