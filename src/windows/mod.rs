pub mod lifecycle;
pub mod manager;
pub mod placement;
pub mod restoration;
pub mod window;

pub use lifecycle::{
    WindowLifecycle,
    WindowLifecycleError,
    WindowLifecyclePhase,
};

pub use manager::{
    WindowManager,
    WindowManagerError,
};

pub use placement::{
    PlacementAnchor,
    PlacementMode,
    WindowGeometry,
    WindowPlacement,
};

pub use restoration::{
    WindowRestoreData,
    WindowRestorePolicy,
};

pub use window::{
    Window,
    WindowId,
    WindowState,
    WindowType,
};
