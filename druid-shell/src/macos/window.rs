use std::any::Any;

use std::ffi::OsString;

use dialog::{FileDialogOptions, FileDialogType};

use Error;

use paint;

pub struct IdleHandle {
}

impl IdleHandle {

    pub fn add_idle<F>(&self, callback: F)
        where F: FnOnce(&Any) + Send + 'static
    {
    }
}

pub trait WinHandler {
    fn as_any(&self) -> &Any;
    fn destroy(&self);
    fn mouse(&self, event: &MouseEvent);
    fn mouse_move(&self, x: i32, y: i32, _mods: u32);
    fn mouse_hwheel(&self, delta: i32, mods: u32);
    fn mouse_wheel(&self, delta: i32, mods: u32);
    fn keydown(&self, vk_code: i32, mods: u32) -> bool;
    fn char(&self, ch: u32, mods: u32);
    fn command(&self, id: u32);
    fn connect(&self, handle: &WindowHandle);
    fn paint(&self, paint_ctx: &mut paint::PaintCtx) -> bool;
}

#[derive(Clone,Default)]
pub struct WindowHandle {}

impl WindowHandle {
    pub fn close(&self) {
    }

    pub fn invalidate(&self) {
    }

    pub fn file_dialog(&self, ty: FileDialogType, options: FileDialogOptions) -> Result<OsString, Error> {
        Ok(OsString::from("foo"))
    }

    pub fn pixels_to_px_xy<T: Into<f64>>(&self, x: T, y: T) -> (f32, f32) {
        (1.0,1.0)
    }
}


#[derive(PartialEq)]
pub enum MouseType {
    Down,
    Up
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MouseButton {}

pub struct MouseEvent {
    pub x : u32,
    pub y : u32,
    pub ty : MouseType,
    pub mods : u32,
    pub which : MouseButton

}

pub const M_ALT : u32 = 34;
pub const VK_MENU : i32 = 34;
pub const VK_F4 : i32 = 34;
pub const VK_F10 : i32 = 34;
