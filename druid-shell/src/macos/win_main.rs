use cocoa::base::{selector, nil, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSWindowStyleMask,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps};

use objc::runtime::Object;
use cairo::QuartzSurface;

pub fn request_quit() {}

pub struct RunLoop {}

impl RunLoop {
    pub fn new() -> RunLoop {
        RunLoop {}
    }

    pub fn run(&self) {
        unsafe {
            let app = NSApp();

            let context: *mut Object = msg_send![class!(NSGraphicsContext), currentContext];
            let context = msg_send![context, CGContext];
            QuartzSurface::create_for_cg_context(context, 100, 100);
            app.run();
        }
    }
}
