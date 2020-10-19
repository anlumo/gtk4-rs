// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::translate::*;
use Monitor;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct ToplevelLayout(Shared<gdk_sys::GdkToplevelLayout>);

    match fn {
        ref => |ptr| gdk_sys::gdk_toplevel_layout_ref(ptr),
        unref => |ptr| gdk_sys::gdk_toplevel_layout_unref(ptr),
        get_type => || gdk_sys::gdk_toplevel_layout_get_type(),
    }
}

impl ToplevelLayout {
    pub fn new(min_width: i32, min_height: i32) -> ToplevelLayout {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gdk_sys::gdk_toplevel_layout_new(min_width, min_height)) }
    }

    pub fn copy(&self) -> Option<ToplevelLayout> {
        unsafe { from_glib_full(gdk_sys::gdk_toplevel_layout_copy(self.to_glib_none().0)) }
    }

    fn equal(&self, other: &ToplevelLayout) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_toplevel_layout_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    pub fn get_fullscreen(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_toplevel_layout_get_fullscreen(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_fullscreen_monitor(&self) -> Option<Monitor> {
        unsafe {
            from_glib_none(gdk_sys::gdk_toplevel_layout_get_fullscreen_monitor(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_maximized(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_toplevel_layout_get_maximized(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_min_height(&self) -> i32 {
        unsafe { gdk_sys::gdk_toplevel_layout_get_min_height(self.to_glib_none().0) }
    }

    pub fn get_min_width(&self) -> i32 {
        unsafe { gdk_sys::gdk_toplevel_layout_get_min_width(self.to_glib_none().0) }
    }

    pub fn get_resizable(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_toplevel_layout_get_resizable(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_fullscreen(&self, fullscreen: bool, monitor: Option<&Monitor>) {
        unsafe {
            gdk_sys::gdk_toplevel_layout_set_fullscreen(
                self.to_glib_none().0,
                fullscreen.to_glib(),
                monitor.to_glib_none().0,
            );
        }
    }

    pub fn set_maximized(&self, maximized: bool) {
        unsafe {
            gdk_sys::gdk_toplevel_layout_set_maximized(self.to_glib_none().0, maximized.to_glib());
        }
    }

    pub fn set_resizable(&self, resizable: bool) {
        unsafe {
            gdk_sys::gdk_toplevel_layout_set_resizable(self.to_glib_none().0, resizable.to_glib());
        }
    }
}

impl PartialEq for ToplevelLayout {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for ToplevelLayout {}
