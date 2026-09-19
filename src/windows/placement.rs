//! Window geometry and placement.
//!
//! Placement is kept independent from the native window backend so the
//! same restoration information can work across X11, Wayland, and other
//! future platforms.

/// Window placement strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementMode {
    Automatic,
    Center,
    Remembered,
    Cascade,
    Tile,
    Manual,
}

/// Anchor used when calculating a placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

/// Basic window geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl WindowGeometry {
    pub fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            x,
            y,
            width: width.max(1),
            height: height.max(1),
        }
    }

    pub fn centered(
        screen_width: u32,
        screen_height: u32,
        width: u32,
        height: u32,
    ) -> Self {
        let width = width.min(screen_width.max(1));
        let height = height.min(screen_height.max(1));

        let x = ((screen_width.saturating_sub(width)) / 2) as i32;
        let y = ((screen_height.saturating_sub(height)) / 2) as i32;

        Self::new(x, y, width, height)
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x
            && y >= self.y
            && x < self.x + self.width as i32
            && y < self.y + self.height as i32
    }

    pub fn right(&self) -> i32 {
        self.x + self.width as i32
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.height as i32
    }
}

impl Default for WindowGeometry {
    fn default() -> Self {
        Self::new(100, 100, 1280, 800)
    }
}

/// Placement request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPlacement {
    pub mode: PlacementMode,
    pub anchor: PlacementAnchor,
    pub geometry: Option<WindowGeometry>,
}

impl Default for WindowPlacement {
    fn default() -> Self {
        Self {
            mode: PlacementMode::Automatic,
            anchor: PlacementAnchor::Center,
            geometry: None,
        }
    }
}

impl WindowPlacement {
    pub fn resolve(
        &self,
        screen_width: u32,
        screen_height: u32,
    ) -> WindowGeometry {
        if self.mode == PlacementMode::Remembered
            || self.mode == PlacementMode::Manual
        {
            if let Some(geometry) = self.geometry {
                return geometry;
            }
        }

        let width = self
            .geometry
            .map(|geometry| geometry.width)
            .unwrap_or(1280);

        let height = self
            .geometry
            .map(|geometry| geometry.height)
            .unwrap_or(800);

        match self.anchor {
            PlacementAnchor::Center => {
                WindowGeometry::centered(
                    screen_width,
                    screen_height,
                    width,
                    height,
                )
            }

            PlacementAnchor::TopLeft => {
                WindowGeometry::new(
                    0,
                    0,
                    width.min(screen_width),
                    height.min(screen_height),
                )
            }

            PlacementAnchor::TopRight => {
                WindowGeometry::new(
                    screen_width.saturating_sub(width) as i32,
                    0,
                    width.min(screen_width),
                    height.min(screen_height),
                )
            }

            PlacementAnchor::BottomLeft => {
                WindowGeometry::new(
                    0,
                    screen_height.saturating_sub(height) as i32,
                    width.min(screen_width),
                    height.min(screen_height),
                )
            }

            PlacementAnchor::BottomRight => {
                WindowGeometry::new(
                    screen_width.saturating_sub(width) as i32,
                    screen_height.saturating_sub(height) as i32,
                    width.min(screen_width),
                    height.min(screen_height),
                )
            }
        }
    }
}
