use super::mouse::MouseEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureDirection {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gesture {
    pub direction: GestureDirection,
    pub distance: f64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureState {
    Idle,
    Tracking,
    Completed,
    Cancelled,
}

#[derive(Debug, Default)]
pub struct GestureRecognizer {
    state: GestureState,
    start_x: f64,
    start_y: f64,
    current_x: f64,
    current_y: f64,
    start_time_ms: Option<u64>,
}

impl GestureRecognizer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin(
        &mut self,
        x: f64,
        y: f64,
        timestamp_ms: u64,
    ) {
        self.state = GestureState::Tracking;
        self.start_x = x;
        self.start_y = y;
        self.current_x = x;
        self.current_y = y;
        self.start_time_ms = Some(timestamp_ms);
    }

    pub fn update(
        &mut self,
        event: &MouseEvent,
    ) {
        if self.state == GestureState::Tracking {
            self.current_x = event.x;
            self.current_y = event.y;
        }
    }

    pub fn finish(
        &mut self,
        timestamp_ms: u64,
    ) -> Option<Gesture> {
        if self.state != GestureState::Tracking {
            return None;
        }

        let dx = self.current_x - self.start_x;
        let dy = self.current_y - self.start_y;

        let distance =
            (dx * dx + dy * dy).sqrt();

        let direction = if distance == 0.0 {
            GestureDirection::None
        } else if dx.abs() >= dy.abs() {
            if dx > 0.0 {
                GestureDirection::Right
            } else {
                GestureDirection::Left
            }
        } else if dy > 0.0 {
            GestureDirection::Down
        } else {
            GestureDirection::Up
        };

        let start_time =
            self.start_time_ms.unwrap_or(timestamp_ms);

        let duration_ms =
            timestamp_ms.saturating_sub(start_time);

        self.state = GestureState::Completed;

        Some(Gesture {
            direction,
            distance,
            duration_ms,
        })
    }

    pub fn cancel(&mut self) {
        self.state = GestureState::Cancelled;
        self.start_time_ms = None;
    }

    pub fn reset(&mut self) {
        self.state = GestureState::Idle;
        self.start_time_ms = None;
    }

    pub fn state(&self) -> GestureState {
        self.state
    }

    pub fn distance(&self) -> f64 {
        let dx = self.current_x - self.start_x;
        let dy = self.current_y - self.start_y;

        (dx * dx + dy * dy).sqrt()
    }
}
