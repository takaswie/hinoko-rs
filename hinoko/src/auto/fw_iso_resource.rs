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
use FwScode;

glib_wrapper! {
    pub struct FwIsoResource(Object<hinoko_sys::HinokoFwIsoResource, hinoko_sys::HinokoFwIsoResourceClass, FwIsoResourceClass>);

    match fn {
        get_type => || hinoko_sys::hinoko_fw_iso_resource_get_type(),
    }
}

impl FwIsoResource {
    pub fn new() -> FwIsoResource {
        unsafe {
            from_glib_full(hinoko_sys::hinoko_fw_iso_resource_new())
        }
    }

    pub fn calculate_bandwidth(bytes_per_payload: u32, scode: FwScode) -> u32 {
        unsafe {
            hinoko_sys::hinoko_fw_iso_resource_calculate_bandwidth(bytes_per_payload, scode.to_glib())
        }
    }
}

impl Default for FwIsoResource {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FW_ISO_RESOURCE: Option<&FwIsoResource> = None;

pub trait FwIsoResourceExt: 'static {
    fn allocate_once_async(&self, channel_candidates: &[u8], bandwidth: u32) -> Result<(), glib::Error>;

    fn allocate_once_sync(&self, channel_candidates: &[u8], bandwidth: u32) -> Result<(), glib::Error>;

    fn create_source(&self) -> Result<glib::Source, glib::Error>;

    fn deallocate_once_async(&self, channel: u32, bandwidth: u32) -> Result<(), glib::Error>;

    fn deallocate_once_sync(&self, channel: u32, bandwidth: u32) -> Result<(), glib::Error>;

    fn open(&self, path: &str, open_flag: i32) -> Result<(), glib::Error>;

    fn connect_allocated<F: Fn(&Self, u32, u32, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_deallocated<F: Fn(&Self, u32, u32, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FwIsoResource>> FwIsoResourceExt for O {
    fn allocate_once_async(&self, channel_candidates: &[u8], bandwidth: u32) -> Result<(), glib::Error> {
        let channel_candidates_count = channel_candidates.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_resource_allocate_once_async(self.as_ref().to_glib_none().0, channel_candidates.to_glib_none().0, channel_candidates_count, bandwidth, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn allocate_once_sync(&self, channel_candidates: &[u8], bandwidth: u32) -> Result<(), glib::Error> {
        let channel_candidates_count = channel_candidates.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_resource_allocate_once_sync(self.as_ref().to_glib_none().0, channel_candidates.to_glib_none().0, channel_candidates_count, bandwidth, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn create_source(&self) -> Result<glib::Source, glib::Error> {
        unsafe {
            let mut gsrc = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_resource_create_source(self.as_ref().to_glib_none().0, &mut gsrc, &mut error);
            if error.is_null() { Ok(from_glib_full(gsrc)) } else { Err(from_glib_full(error)) }
        }
    }

    fn deallocate_once_async(&self, channel: u32, bandwidth: u32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_resource_deallocate_once_async(self.as_ref().to_glib_none().0, channel, bandwidth, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn deallocate_once_sync(&self, channel: u32, bandwidth: u32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_resource_deallocate_once_sync(self.as_ref().to_glib_none().0, channel, bandwidth, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn open(&self, path: &str, open_flag: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_resource_open(self.as_ref().to_glib_none().0, path.to_glib_none().0, open_flag, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_allocated<F: Fn(&Self, u32, u32, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn allocated_trampoline<P, F: Fn(&P, u32, u32, &glib::Error) + 'static>(this: *mut hinoko_sys::HinokoFwIsoResource, channel: libc::c_uint, bandwidth: libc::c_uint, error: *mut glib_sys::GError, f: glib_sys::gpointer)
            where P: IsA<FwIsoResource>
        {
            let f: &F = &*(f as *const F);
            f(&FwIsoResource::from_glib_borrow(this).unsafe_cast_ref(), channel, bandwidth, &from_glib_borrow(error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"allocated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(allocated_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_deallocated<F: Fn(&Self, u32, u32, &glib::Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn deallocated_trampoline<P, F: Fn(&P, u32, u32, &glib::Error) + 'static>(this: *mut hinoko_sys::HinokoFwIsoResource, channel: libc::c_uint, bandwidth: libc::c_uint, error: *mut glib_sys::GError, f: glib_sys::gpointer)
            where P: IsA<FwIsoResource>
        {
            let f: &F = &*(f as *const F);
            f(&FwIsoResource::from_glib_borrow(this).unsafe_cast_ref(), channel, bandwidth, &from_glib_borrow(error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"deallocated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(deallocated_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for FwIsoResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FwIsoResource")
    }
}
