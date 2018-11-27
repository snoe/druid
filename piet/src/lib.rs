#[cfg(target_os = "windows")]
#[macro_use]
extern crate direct2d;

#[cfg(target_os = "windows")]
extern crate directwrite;

#[cfg(target_os = "windows")]
pub mod windows {
    pub mod math;
    pub mod render_target;
    pub mod brush;
    pub mod write;
}

#[cfg(target_os = "windows")]
pub use windows::math;
#[cfg(target_os = "windows")]
pub use windows::render_target;
#[cfg(target_os = "windows")]
pub use windows::brush;
#[cfg(target_os = "windows")]
pub use windows::write;
#[cfg(target_os = "windows")]
pub use windows::render_target::RenderTarget;
#[cfg(target_os = "windows")]
pub use direct2d::Factory;

#[cfg(target_os = "macos")]
pub mod macos {
    mod stroke_style;
    pub mod math;
    pub mod render_target;
    pub mod brush;
    pub mod write;
}

#[cfg(target_os = "macos")]
pub use macos::math;
#[cfg(target_os = "macos")]
pub use macos::render_target;
#[cfg(target_os = "macos")]
pub use macos::brush;
#[cfg(target_os = "macos")]
pub use macos::write;
#[cfg(target_os = "macos")]
pub use macos::render_target::RenderTarget;
#[cfg(target_os = "macos")]
pub struct Factory {}
