extern crate webview_sys;
extern crate raw_window_handle;
extern crate gtk;
extern crate gdkx11;
extern crate gtk_sys;

use self::webview_sys::ParentType;
use self::raw_window_handle::RawWindowHandle;
use self::gtk::prelude::*;
use self::gtk::gdk::Display;
use self::gdkx11::{X11Display, X11Window};



pub struct Parent {
    pub inner: ParentType,
}

pub trait FromRawWindowHandle {
    fn from_raw_window_handle(window_handle: RawWindowHandle) -> Parent;
}


#[cfg(all(target_family = "unix", not(target_os = "macos")))]
impl FromRawWindowHandle for Parent {
    fn from_raw_window_handle(window_handle: RawWindowHandle) -> Parent {
        if let RawWindowHandle::Xlib(window_handle) = window_handle {
            unsafe {
                gtk::init().unwrap();
                let display: X11Display = Display::default().unwrap().downcast().unwrap();
                let gw = X11Window::foreign_new_for_display(&display, window_handle.window).upcast();
                let gtk = gtk::Window::new(gtk::WindowType::Toplevel);
                gtk.connect_realize(move |widget| {
                    widget.set_window(gw);
                });
                gtk.set_has_window(true);
                gtk.realize();
                let widget = gtk.default_widget().unwrap();
                Parent {
                    inner: widget.as_ptr(),
                }
            } 
        } else {
            panic!("XCB Not compiled!! There's something wrong in the X Server Config.")
        }
    }
}


