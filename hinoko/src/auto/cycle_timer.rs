// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CycleTimer(Boxed<ffi::HinokoCycleTimer>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::hinoko_cycle_timer_get_type(), ptr as *mut _) as *mut ffi::HinokoCycleTimer,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::hinoko_cycle_timer_get_type(), ptr as *mut _),
        type_ => || ffi::hinoko_cycle_timer_get_type(),
    }
}

impl CycleTimer {
    #[doc(alias = "hinoko_cycle_timer_new")]
    pub fn new() -> CycleTimer {
        unsafe { from_glib_none(ffi::hinoko_cycle_timer_new()) }
    }

    #[doc(alias = "hinoko_cycle_timer_get_clock_id")]
    #[doc(alias = "get_clock_id")]
    pub fn clock_id(&mut self) -> i32 {
        unsafe {
            let mut clock_id = mem::MaybeUninit::uninit();
            ffi::hinoko_cycle_timer_get_clock_id(self.to_glib_none_mut().0, clock_id.as_mut_ptr());
            let clock_id = clock_id.assume_init();
            clock_id
        }
    }

    #[doc(alias = "hinoko_cycle_timer_get_timestamp")]
    #[doc(alias = "get_timestamp")]
    pub fn timestamp(&mut self) -> (i64, i32) {
        unsafe {
            let mut tv_sec = mem::MaybeUninit::uninit();
            let mut tv_nsec = mem::MaybeUninit::uninit();
            ffi::hinoko_cycle_timer_get_timestamp(
                self.to_glib_none_mut().0,
                tv_sec.as_mut_ptr(),
                tv_nsec.as_mut_ptr(),
            );
            let tv_sec = tv_sec.assume_init();
            let tv_nsec = tv_nsec.assume_init();
            (tv_sec, tv_nsec)
        }
    }
}

impl Default for CycleTimer {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for CycleTimer {}
