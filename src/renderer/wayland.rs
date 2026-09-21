use super::renderer::RenderError;

/// Wayland surface state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaylandSurfaceState {
    Uninitialized,
    Connected,
    Configured,
    Suspended,
    Closed,
}

impl Default for WaylandSurfaceState {
    fn default() -> Self {
        Self::Uninitialized
    }
}

/// Wayland surface dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaylandSurfaceSize {
    width: u32,
    height: u32,
    scale_factor: u32,
}

impl WaylandSurfaceSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            scale_factor: 1,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn scale_factor(&self) -> u32 {
        self.scale_factor
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_scale_factor(&mut self, scale_factor: u32) {
        self.scale_factor = scale_factor.max(1);
    }

    pub fn physical_width(&self) -> u32 {
        self.width.saturating_mul(self.scale_factor)
    }

    pub fn physical_height(&self) -> u32 {
        self.height.saturating_mul(self.scale_factor)
    }
}

/// Logical Wayland surface description.
#[derive(Debug, Clone)]
pub struct WaylandSurface {
    state: WaylandSurfaceState,
    size: WaylandSurfaceSize,
    opaque: bool,
    transparent: bool,
    fullscreen: bool,
    maximized: bool,
    decorated: bool,
}

impl WaylandSurface {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            state: WaylandSurfaceState::Uninitialized,
            size: WaylandSurfaceSize::new(width, height),
            opaque: false,
            transparent: true,
            fullscreen: false,
            maximized: false,
            decorated: true,
        }
    }

    pub fn state(&self) -> WaylandSurfaceState {
        self.state
    }

    pub fn size(&self) -> WaylandSurfaceSize {
        self.size
    }

    pub fn opaque(&self) -> bool {
        self.opaque
    }

    pub fn transparent(&self) -> bool {
        self.transparent
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn maximized(&self) -> bool {
        self.maximized
    }

    pub fn decorated(&self) -> bool {
        self.decorated
    }

    pub fn connect(&mut self) {
        self.state = WaylandSurfaceState::Connected;
    }

    pub fn configure(&mut self) {
        self.state = WaylandSurfaceState::Configured;
    }

    pub fn suspend(&mut self) {
        self.state = WaylandSurfaceState::Suspended;
    }

    pub fn close(&mut self) {
        self.state = WaylandSurfaceState::Closed;
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<(), RenderError> {
        if width == 0 || height == 0 {
            return Err(RenderError::InvalidDimensions);
        }

        self.size.set_size(width, height);
        Ok(())
    }

    pub fn set_scale_factor(&mut self, factor: u32) {
        self.size.set_scale_factor(factor);
    }

    pub fn set_opaque(&mut self, opaque: bool) {
        self.opaque = opaque;
        self.transparent = !opaque;
    }

    pub fn set_transparent(&mut self, transparent: bool) {
        self.transparent = transparent;
        self.opaque = !transparent;
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.maximized = maximized;
    }

    pub fn set_decorated(&mut self, decorated: bool) {
        self.decorated = decorated;
    }
}

/// Wayland display connection abstraction.
#[derive(Debug, Clone)]
pub struct WaylandDisplay {
    name: String,
    connected: bool,
    compositor_available: bool,
    fractional_scaling: bool,
}

impl WaylandDisplay {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            connected: false,
            compositor_available: false,
            fractional_scaling: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn compositor_available(&self) -> bool {
        self.compositor_available
    }

    pub fn fractional_scaling(&self) -> bool {
        self.fractional_scaling
    }

    pub fn connect(&mut self) -> Result<(), RenderError> {
        self.connected = true;
        self.compositor_available = true;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        self.compositor_available = false;
    }

    pub fn set_fractional_scaling(&mut self, enabled: bool) {
        self.fractional_scaling = enabled;
    }
}
