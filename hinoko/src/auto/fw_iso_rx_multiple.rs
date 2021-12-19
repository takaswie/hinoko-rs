// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use hinoko_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use FwIsoCtx;

glib_wrapper! {
    pub struct FwIsoRxMultiple(Object<hinoko_sys::HinokoFwIsoRxMultiple, hinoko_sys::HinokoFwIsoRxMultipleClass, FwIsoRxMultipleClass>) @extends FwIsoCtx;

    match fn {
        get_type => || hinoko_sys::hinoko_fw_iso_rx_multiple_get_type(),
    }
}

impl FwIsoRxMultiple {
    pub fn new() -> FwIsoRxMultiple {
        unsafe {
            from_glib_full(hinoko_sys::hinoko_fw_iso_rx_multiple_new())
        }
    }
}

impl Default for FwIsoRxMultiple {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FW_ISO_RX_MULTIPLE: Option<&FwIsoRxMultiple> = None;

pub trait FwIsoRxMultipleExt: 'static {
    fn allocate(&self, path: &str, channels: &[u8]) -> Result<(), glib::Error>;

    fn map_buffer(&self, bytes_per_chunk: u32, chunks_per_buffer: u32) -> Result<(), glib::Error>;

    fn release(&self);

    fn stop(&self);

    fn unmap_buffer(&self);

    fn connect_interrupted<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FwIsoRxMultiple>> FwIsoRxMultipleExt for O {
    fn allocate(&self, path: &str, channels: &[u8]) -> Result<(), glib::Error> {
        let channels_length = channels.len() as u32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_rx_multiple_allocate(self.as_ref().to_glib_none().0, path.to_glib_none().0, channels.to_glib_none().0, channels_length, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn map_buffer(&self, bytes_per_chunk: u32, chunks_per_buffer: u32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_rx_multiple_map_buffer(self.as_ref().to_glib_none().0, bytes_per_chunk, chunks_per_buffer, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn release(&self) {
        unsafe {
            hinoko_sys::hinoko_fw_iso_rx_multiple_release(self.as_ref().to_glib_none().0);
        }
    }

    fn stop(&self) {
        unsafe {
            hinoko_sys::hinoko_fw_iso_rx_multiple_stop(self.as_ref().to_glib_none().0);
        }
    }

    fn unmap_buffer(&self) {
        unsafe {
            hinoko_sys::hinoko_fw_iso_rx_multiple_unmap_buffer(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_interrupted<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn interrupted_trampoline<P, F: Fn(&P, u32) + 'static>(this: *mut hinoko_sys::HinokoFwIsoRxMultiple, count: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<FwIsoRxMultiple>
        {
            let f: &F = &*(f as *const F);
            f(&FwIsoRxMultiple::from_glib_borrow(this).unsafe_cast_ref(), count)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"interrupted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(interrupted_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for FwIsoRxMultiple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FwIsoRxMultiple")
    }
}
