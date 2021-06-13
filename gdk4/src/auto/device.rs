// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::DeviceTool;
use crate::Display;
use crate::InputSource;
use crate::ModifierType;
use crate::Seat;
use crate::Surface;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GdkDevice")]
    pub struct Device(Object<ffi::GdkDevice>);

    match fn {
        type_ => || ffi::gdk_device_get_type(),
    }
}

pub const NONE_DEVICE: Option<&Device> = None;

pub trait DeviceExt: 'static {
    #[doc(alias = "gdk_device_get_caps_lock_state")]
    #[doc(alias = "get_caps_lock_state")]
    fn is_caps_locked(&self) -> bool;

    #[doc(alias = "gdk_device_get_device_tool")]
    #[doc(alias = "get_device_tool")]
    fn device_tool(&self) -> Option<DeviceTool>;

    #[doc(alias = "gdk_device_get_direction")]
    #[doc(alias = "get_direction")]
    fn direction(&self) -> pango::Direction;

    #[doc(alias = "gdk_device_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display>;

    #[doc(alias = "gdk_device_get_has_cursor")]
    #[doc(alias = "get_has_cursor")]
    fn has_cursor(&self) -> bool;

    #[doc(alias = "gdk_device_get_modifier_state")]
    #[doc(alias = "get_modifier_state")]
    fn modifier_state(&self) -> ModifierType;

    #[doc(alias = "gdk_device_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[doc(alias = "gdk_device_get_num_lock_state")]
    #[doc(alias = "get_num_lock_state")]
    fn is_num_locked(&self) -> bool;

    #[doc(alias = "gdk_device_get_num_touches")]
    #[doc(alias = "get_num_touches")]
    fn num_touches(&self) -> u32;

    #[doc(alias = "gdk_device_get_product_id")]
    #[doc(alias = "get_product_id")]
    fn product_id(&self) -> Option<glib::GString>;

    #[doc(alias = "gdk_device_get_scroll_lock_state")]
    #[doc(alias = "get_scroll_lock_state")]
    fn is_scroll_locked(&self) -> bool;

    #[doc(alias = "gdk_device_get_seat")]
    #[doc(alias = "get_seat")]
    fn seat(&self) -> Option<Seat>;

    #[doc(alias = "gdk_device_get_source")]
    #[doc(alias = "get_source")]
    fn source(&self) -> InputSource;

    #[doc(alias = "gdk_device_get_surface_at_position")]
    #[doc(alias = "get_surface_at_position")]
    fn surface_at_position(&self) -> (Option<Surface>, f64, f64);

    #[doc(alias = "gdk_device_get_vendor_id")]
    #[doc(alias = "get_vendor_id")]
    fn vendor_id(&self) -> Option<glib::GString>;

    #[doc(alias = "gdk_device_has_bidi_layouts")]
    fn has_bidi_layouts(&self) -> bool;

    #[doc(alias = "n-axes")]
    fn n_axes(&self) -> u32;

    fn set_seat<P: IsA<Seat>>(&self, seat: Option<&P>);

    fn tool(&self) -> Option<DeviceTool>;

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "tool-changed")]
    fn connect_tool_changed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "caps-lock-state")]
    fn connect_caps_lock_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "direction")]
    fn connect_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "has-bidi-layouts")]
    fn connect_has_bidi_layouts_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "modifier-state")]
    fn connect_modifier_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "n-axes")]
    fn connect_n_axes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "num-lock-state")]
    fn connect_num_lock_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "scroll-lock-state")]
    fn connect_scroll_lock_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "seat")]
    fn connect_seat_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "tool")]
    fn connect_tool_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Device>> DeviceExt for O {
    fn is_caps_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_get_caps_lock_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn device_tool(&self) -> Option<DeviceTool> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_device_tool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn direction(&self) -> pango::Direction {
        unsafe {
            from_glib(ffi::gdk_device_get_direction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_device_get_display(self.as_ref().to_glib_none().0)) }
    }

    fn has_cursor(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_get_has_cursor(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn modifier_state(&self) -> ModifierType {
        unsafe {
            from_glib(ffi::gdk_device_get_modifier_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_device_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn is_num_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_get_num_lock_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn num_touches(&self) -> u32 {
        unsafe { ffi::gdk_device_get_num_touches(self.as_ref().to_glib_none().0) }
    }

    fn product_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_product_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_scroll_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_get_scroll_lock_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn seat(&self) -> Option<Seat> {
        unsafe { from_glib_none(ffi::gdk_device_get_seat(self.as_ref().to_glib_none().0)) }
    }

    fn source(&self) -> InputSource {
        unsafe { from_glib(ffi::gdk_device_get_source(self.as_ref().to_glib_none().0)) }
    }

    fn surface_at_position(&self) -> (Option<Surface>, f64, f64) {
        unsafe {
            let mut win_x = mem::MaybeUninit::uninit();
            let mut win_y = mem::MaybeUninit::uninit();
            let ret = from_glib_none(ffi::gdk_device_get_surface_at_position(
                self.as_ref().to_glib_none().0,
                win_x.as_mut_ptr(),
                win_y.as_mut_ptr(),
            ));
            let win_x = win_x.assume_init();
            let win_y = win_y.assume_init();
            (ret, win_x, win_y)
        }
    }

    fn vendor_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_vendor_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_bidi_layouts(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_device_has_bidi_layouts(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn n_axes(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"n-axes\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `n-axes` getter")
        }
    }

    fn set_seat<P: IsA<Seat>>(&self, seat: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"seat\0".as_ptr() as *const _,
                seat.to_value().to_glib_none().0,
            );
        }
    }

    fn tool(&self) -> Option<DeviceTool> {
        unsafe {
            let mut value = glib::Value::from_type(<DeviceTool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"tool\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tool` getter")
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_tool_changed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tool_changed_trampoline<
            P: IsA<Device>,
            F: Fn(&P, &DeviceTool) + 'static,
        >(
            this: *mut ffi::GdkDevice,
            tool: *mut ffi::GdkDeviceTool,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &Device::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(tool),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tool_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_caps_lock_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_lock_state_trampoline<
            P: IsA<Device>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::caps-lock-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_lock_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_direction_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_has_bidi_layouts_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_bidi_layouts_trampoline<
            P: IsA<Device>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-bidi-layouts\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_bidi_layouts_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_modifier_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modifier_state_trampoline<
            P: IsA<Device>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modifier-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modifier_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_n_axes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_axes_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-axes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_axes_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_num_lock_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_num_lock_state_trampoline<
            P: IsA<Device>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::num-lock-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_num_lock_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_scroll_lock_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scroll_lock_state_trampoline<
            P: IsA<Device>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scroll-lock-state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scroll_lock_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_seat_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seat_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seat\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_seat_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_tool_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tool_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDevice,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tool\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tool_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Device")
    }
}
