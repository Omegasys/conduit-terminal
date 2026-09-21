use super::renderer::RenderError;

/// X11 connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X11ConnectionState {
    Disconnected,
    Connected,
    Error,
    Closed,
}

impl Default for X11ConnectionState {
    fn default() -> Self {
        Self::Disconnected
    }
}

/// X11 window geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X11WindowGeometry {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl X11WindowGeometry {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

/// Logical X11 window abstraction.
#[derive(Debug, Clone)]
pub struct X11Window {
    window_id: u64,
    geometry: X11WindowGeometry,
    mapped: bool,
    fullscreen: bool,
    maximized: bool,
    override_redirect: bool,
}

impl X11Window {
    pub fn new(
        window_id: u64,
        geometry: X11WindowGeometry,
    ) -> Self {
        Self {
            window_id,
            geometry,
            mapped: false,
            fullscreen: false,
            maximized: false,
            override_redirect: false,
        }
    }

    pub fn window_id(&self) -> u64 {
        self.window_id
    }

    pub fn geometry(&self) -> X11WindowGeometry {
        self.geometry
    }

    pub fn mapped(&self) -> bool {
        self.mapped
    }

    pub fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn maximized(&self) -> bool {
        self.maximized
    }

    pub fn override_redirect(&self) -> bool {
        self.override_redirect
    }

    pub fn map(&mut self) {
        self.mapped = true;
    }

    pub fn unmap(&mut self) {
        self.mapped = false;
    }

    pub fn set_geometry(&mut self, geometry: X11WindowGeometry) {
        self.geometry = geometry;
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.maximized = maximized;
    }

    pub fn set_override_redirect(&mut self, enabled: bool) {
        self.override_redirect = enabled;
    }
}

/// X11 display connection abstraction.
#[derive(Debug, Clone)]
pub struct X11Display {
    display_name: String,
    state: X11ConnectionState,
    screen_count: u32,
    composite_available: bool,
    damage_available: bool,
    xrandr_available: bool,
}

impl X11Display {
    pub fn new<S>(display_name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            display_name: display_name.into(),
            state: X11ConnectionState::Disconnected,
            screen_count: 0,
            composite_available: false,
            damage_available: false,
            xrandr_available: false,
        }
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn state(&self) -> X11ConnectionState {
        self.state
    }

    pub fn screen_count(&self) -> u32 {
        self.screen_count
    }

    pub fn composite_available(&self) -> bool {
        self.composite_available
    }

    pub fn damage_available(&self) -> bool {
        self.damage_available
    }

    pub fn xrandr_available(&self) -> bool {
        self.xrandr_available
    }

    pub fn connect(&mut self) -> Result<(), RenderError> {
        self.state = X11ConnectionState::Connected;
        self.screen_count = 1;
        self.composite_available = true;
        self.damage_available = true;
        self.xrandr_available = true;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.state = X11ConnectionState::Closed;
        self.screen_count = 0;
    }
}
