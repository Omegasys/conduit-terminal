use crate::colors::{
    Color,
    ColorCapabilities,
    ColorConversion,
    Rgba,
    SemanticColor,
    TrueColor,
};

use super::panes::FlowPaneId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMode {
    Normal,
    Debug,
    Minimal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderLayer {
    Background,
    Terminal,
    Cursor,
    Selection,
    Overlay,
    Debug,
}

#[derive(Clone, Copy, Debug)]
pub struct RenderCell {
    pub foreground: Color,
    pub background: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Default for RenderCell {
    fn default() -> Self {
        Self {
            foreground: Color::rgb(216, 222, 233),
            background: Color::rgb(46, 52, 64),
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

impl RenderCell {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
            ..Self::default()
        }
    }

    pub fn with_bold(mut self, value: bool) -> Self {
        self.bold = value;
        self
    }

    pub fn with_italic(mut self, value: bool) -> Self {
        self.italic = value;
        self
    }

    pub fn with_underline(mut self, value: bool) -> Self {
        self.underline = value;
        self
    }
}

#[derive(Clone, Debug)]
pub struct PaneRenderState {
    pane: FlowPaneId,
    width: u16,
    height: u16,
    cells: Vec<RenderCell>,
}

impl PaneRenderState {
    pub fn new(pane: FlowPaneId, width: u16, height: u16) -> Self {
        let count = width as usize * height as usize;

        Self {
            pane,
            width,
            height,
            cells: vec![RenderCell::default(); count],
        }
    }

    pub fn pane(&self) -> FlowPaneId {
        self.pane
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn cell(&self, x: u16, y: u16) -> Option<&RenderCell> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = y as usize * self.width as usize + x as usize;
        self.cells.get(index)
    }

    pub fn cell_mut(&mut self, x: u16, y: u16) -> Option<&mut RenderCell> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = y as usize * self.width as usize + x as usize;
        self.cells.get_mut(index)
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;

        self.cells
            .resize(width as usize * height as usize, RenderCell::default());
    }
}

#[derive(Clone, Debug)]
pub struct TerminalRenderer {
    mode: RenderMode,
    capabilities: ColorCapabilities,
    default_foreground: Color,
    default_background: Color,
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self {
            mode: RenderMode::Normal,
            capabilities: ColorCapabilities::truecolor(),
            default_foreground: Color::rgb(216, 222, 233),
            default_background: Color::rgb(46, 52, 64),
        }
    }

    pub fn mode(&self) -> RenderMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: RenderMode) {
        self.mode = mode;
    }

    pub fn capabilities(&self) -> ColorCapabilities {
        self.capabilities
    }

    pub fn set_capabilities(&mut self, capabilities: ColorCapabilities) {
        self.capabilities = capabilities;
    }

    pub fn set_default_colors(
        &mut self,
        foreground: Color,
        background: Color,
    ) {
        self.default_foreground = foreground;
        self.default_background = background;
    }

    pub fn default_foreground(&self) -> Color {
        self.default_foreground
    }

    pub fn default_background(&self) -> Color {
        self.default_background
    }

    pub fn resolve_color(&self, color: Color) -> Rgba {
        ColorConversion::to_rgba(color)
    }

    pub fn resolve_semantic(
        &self,
        color: SemanticColor,
    ) -> Color {
        match color {
            SemanticColor::TerminalForeground => self.default_foreground,
            SemanticColor::TerminalBackground => self.default_background,
            SemanticColor::Cursor => Color::rgb(136, 192, 208),
            SemanticColor::Selection => Color::rgba(67, 76, 94, 180),
            SemanticColor::Accent => Color::rgb(136, 192, 208),
            _ => self.default_foreground,
        }
    }

    pub fn truecolor(&self, color: Rgba) -> TrueColor {
        TrueColor::rgba(
            color.red(),
            color.green(),
            color.blue(),
            color.alpha(),
        )
    }
}
