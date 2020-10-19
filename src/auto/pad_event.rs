// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::translate::*;
use std::fmt;
use std::mem;
use Event;

glib_wrapper! {
    pub struct PadEvent(Object<gdk_sys::GdkPadEvent, PadEventClass>) @extends Event;

    match fn {
        get_type => || gdk_sys::gdk_pad_event_get_type(),
    }
}

impl PadEvent {
    pub fn get_axis_value(&self) -> (u32, f64) {
        unsafe {
            let mut index = mem::MaybeUninit::uninit();
            let mut value = mem::MaybeUninit::uninit();
            gdk_sys::gdk_pad_event_get_axis_value(
                self.to_glib_none().0,
                index.as_mut_ptr(),
                value.as_mut_ptr(),
            );
            let index = index.assume_init();
            let value = value.assume_init();
            (index, value)
        }
    }

    pub fn get_button(&self) -> u32 {
        unsafe { gdk_sys::gdk_pad_event_get_button(self.to_glib_none().0) }
    }

    pub fn get_group_mode(&self) -> (u32, u32) {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let mut mode = mem::MaybeUninit::uninit();
            gdk_sys::gdk_pad_event_get_group_mode(
                self.to_glib_none().0,
                group.as_mut_ptr(),
                mode.as_mut_ptr(),
            );
            let group = group.assume_init();
            let mode = mode.assume_init();
            (group, mode)
        }
    }
}

impl fmt::Display for PadEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PadEvent")
    }
}
