// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::IsA;
use glib::translate::*;
use hinoko_sys;
use std::fmt;
use std::ptr;
use FwIsoCtx;
use FwScode;

glib_wrapper! {
    pub struct FwIsoTx(Object<hinoko_sys::HinokoFwIsoTx, hinoko_sys::HinokoFwIsoTxClass, FwIsoTxClass>) @extends FwIsoCtx;

    match fn {
        get_type => || hinoko_sys::hinoko_fw_iso_tx_get_type(),
    }
}

impl FwIsoTx {
    pub fn new() -> FwIsoTx {
        unsafe {
            from_glib_full(hinoko_sys::hinoko_fw_iso_tx_new())
        }
    }
}

impl Default for FwIsoTx {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FW_ISO_TX: Option<&FwIsoTx> = None;

pub trait FwIsoTxExt: 'static {
    fn allocate(&self, path: &str, scode: FwScode, channel: u32, header_size: u32) -> Result<(), glib::Error>;

    fn map_buffer(&self, maximum_bytes_per_payload: u32, payloads_per_buffer: u32) -> Result<(), glib::Error>;

    fn release(&self);

    fn stop(&self);

    fn unmap_buffer(&self);
}

impl<O: IsA<FwIsoTx>> FwIsoTxExt for O {
    fn allocate(&self, path: &str, scode: FwScode, channel: u32, header_size: u32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_tx_allocate(self.as_ref().to_glib_none().0, path.to_glib_none().0, scode.to_glib(), channel, header_size, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn map_buffer(&self, maximum_bytes_per_payload: u32, payloads_per_buffer: u32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinoko_sys::hinoko_fw_iso_tx_map_buffer(self.as_ref().to_glib_none().0, maximum_bytes_per_payload, payloads_per_buffer, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn release(&self) {
        unsafe {
            hinoko_sys::hinoko_fw_iso_tx_release(self.as_ref().to_glib_none().0);
        }
    }

    fn stop(&self) {
        unsafe {
            hinoko_sys::hinoko_fw_iso_tx_stop(self.as_ref().to_glib_none().0);
        }
    }

    fn unmap_buffer(&self) {
        unsafe {
            hinoko_sys::hinoko_fw_iso_tx_unmap_buffer(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for FwIsoTx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FwIsoTx")
    }
}
