//! Pane splitting and split geometry.

use super::pane::PaneId;

/// Direction in which a pane is split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

impl SplitDirection {
    pub fn opposite(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

/// Relative size of a split.
///
/// A value of `0.5` represents an even split.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SplitRatio(f32);

impl SplitRatio {
    pub const MIN: f32 = 0.05;
    pub const MAX: f32 = 0.95;

    pub fn new(value: f32) -> Self {
        Self(
            value.clamp(
                Self::MIN,
                Self::MAX,
            ),
        )
    }

    pub fn value(self) -> f32 {
        self.0
    }

    pub fn inverse(self) -> Self {
        Self::new(1.0 - self.0)
    }
}

impl Default for SplitRatio {
    fn default() -> Self {
        Self::new(0.5)
    }
}

/// A split operation between two panes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaneSplit {
    pub source: PaneId,
    pub new_pane: PaneId,
    pub direction: SplitDirection,
    pub ratio: SplitRatio,
}

impl PaneSplit {
    pub fn new(
        source: PaneId,
        new_pane: PaneId,
        direction: SplitDirection,
    ) -> Self {
        Self {
            source,
            new_pane,
            direction,
            ratio: SplitRatio::default(),
        }
    }

    pub fn with_ratio(
        mut self,
        ratio: SplitRatio,
    ) -> Self {
        self.ratio = ratio;
        self
    }
}
