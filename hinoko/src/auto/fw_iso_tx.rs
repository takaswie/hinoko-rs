// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::FwIsoCtx;
use crate::FwScode;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "HinokoFwIsoTx")]
    pub struct FwIsoTx(Object<ffi::HinokoFwIsoTx, ffi::HinokoFwIsoTxClass>) @implements FwIsoCtx;

    match fn {
        type_ => || ffi::hinoko_fw_iso_tx_get_type(),
    }
}

impl FwIsoTx {
    pub const NONE: Option<&'static FwIsoTx> = None;

    #[doc(alias = "hinoko_fw_iso_tx_new")]
    pub fn new() -> FwIsoTx {
        unsafe { from_glib_full(ffi::hinoko_fw_iso_tx_new()) }
    }
}

impl Default for FwIsoTx {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FwIsoTxExt: 'static {
    #[doc(alias = "hinoko_fw_iso_tx_allocate")]
    fn allocate(
        &self,
        path: &str,
        scode: FwScode,
        channel: u32,
        header_size: u32,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "hinoko_fw_iso_tx_map_buffer")]
    fn map_buffer(
        &self,
        maximum_bytes_per_payload: u32,
        payloads_per_buffer: u32,
    ) -> Result<(), glib::Error>;
}

impl<O: IsA<FwIsoTx>> FwIsoTxExt for O {
    fn allocate(
        &self,
        path: &str,
        scode: FwScode,
        channel: u32,
        header_size: u32,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::hinoko_fw_iso_tx_allocate(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                scode.into_glib(),
                channel,
                header_size,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn map_buffer(
        &self,
        maximum_bytes_per_payload: u32,
        payloads_per_buffer: u32,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::hinoko_fw_iso_tx_map_buffer(
                self.as_ref().to_glib_none().0,
                maximum_bytes_per_payload,
                payloads_per_buffer,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for FwIsoTx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FwIsoTx")
    }
}
