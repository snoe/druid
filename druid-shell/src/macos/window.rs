use std::any::Any;

use std::ffi::OsString;

use std::os::raw::c_void;

use dialog::{FileDialogOptions, FileDialogType};
use menu::Menu;

use cocoa::base::{id, selector, nil, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSWindowStyleMask,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps};

use cairo::QuartzSurface;
use core_graphics::context::CGContextRef;

use objc::runtime::Object;

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
pub struct WindowHandle {
    app : Option<id>,
    window : Option<id>
}

impl WindowHandle {
    pub fn show(&self) {
        unsafe {
            self.window.unwrap().makeKeyAndOrderFront_(nil);
        }
    }

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

#[derive(PartialEq, Debug)]
pub enum MouseType {
    Down,
    Up
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MouseButton {}

#[derive(Debug)]
pub struct MouseEvent {
    pub x : u32,
    pub y : u32,
    pub ty : MouseType,
    pub mods : u32,
    pub which : MouseButton
}

pub struct WindowBuilder {
    title: String,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            title: "".to_string(),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn set_handler(&self, handler: Box<WinHandler>) {
    }

    pub fn set_menu(&self, menu: Menu) {
    }

    pub fn build(&self) -> Result<WindowHandle, Error> {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);

            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            // create Menu Bar
            let menubar = NSMenu::new(nil).autorelease();
            let app_menu_item = NSMenuItem::new(nil).autorelease();
            menubar.addItem_(app_menu_item);
            app.setMainMenu_(menubar);


            // create Application menu
            let app_menu = NSMenu::new(nil).autorelease();
            let quit_prefix = NSString::alloc(nil).init_str("Quit ");
            let quit_title =
                quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
            let quit_action = selector("terminate:");
            let quit_key = NSString::alloc(nil).init_str("q");
            let quit_item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
                .autorelease();
            app_menu.addItem_(quit_item);
            app_menu_item.setSubmenu_(app_menu);

            // create Window
            let window = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),
                NSSize::new(200., 200.)),
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreBuffered,
                NO)
                .autorelease();
            window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
            window.center();
            let title = NSString::alloc(nil).init_str(&self.title);
            window.setTitle_(title);
            let current_app = NSRunningApplication::currentApplication(nil);
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
           /* 	
	cairo_surface_t*	theSurface = cairo_quartz_surface_create_for_cg_context( macContext, self.bounds.size.width, self.bounds.size.height );
	cairo_t*			theContext = cairo_create( theSurface );
	
	// === Use theSurface and theContext in platform-independent code here:
	
	cairo_set_line_width( theContext, 1 );
	cairo_set_source_rgb( theContext, 0, 0, 0 );
	cairo_rectangle( theContext, 10, 10, 100, 100 );
	cairo_stroke( theContext );
	
	// === End of platform-independent code.
	
	cairo_destroy(theContext);
	cairo_surface_destroy( theSurface );
    */

            Ok(WindowHandle {app : Some(app), window : Some(window)})
        }
    }
}

pub const M_ALT : u32 = 34;
pub const VK_MENU : i32 = 34;
pub const VK_F4 : i32 = 34;
pub const VK_F10 : i32 = 34;
