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
    /// An object to transmit isochronous packet for single channel.
    ///
    /// A [`FwIsoTx`][crate::FwIsoTx] transmits isochronous packets for single channel by IT context in 1394 OHCI.
    /// The content of packet is split to two parts; context header and context payload in a manner of
    /// Linux FireWire subsystem.
    ///
    /// # Implements
    ///
    /// [`FwIsoTxExt`][trait@crate::prelude::FwIsoTxExt], [`FwIsoCtxExt`][trait@crate::prelude::FwIsoCtxExt], [`FwIsoTxExtManual`][trait@crate::prelude::FwIsoTxExtManual], [`FwIsoCtxExtManual`][trait@crate::prelude::FwIsoCtxExtManual]
    #[doc(alias = "HinokoFwIsoTx")]
    pub struct FwIsoTx(Object<ffi::HinokoFwIsoTx, ffi::HinokoFwIsoTxClass>) @implements FwIsoCtx;

    match fn {
        type_ => || ffi::hinoko_fw_iso_tx_get_type(),
    }
}

impl FwIsoTx {
    pub const NONE: Option<&'static FwIsoTx> = None;

    /// Instantiate [`FwIsoTx`][crate::FwIsoTx] object and return the instance.
    ///
    /// # Returns
    ///
    /// an instance of [`FwIsoTx`][crate::FwIsoTx].
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

/// Trait containing the part of[`struct@FwIsoTx`] methods.
///
/// # Implementors
///
/// [`FwIsoTx`][struct@crate::FwIsoTx]
pub trait FwIsoTxExt: 'static {
    /// Allocate an IT context to 1394 OHCI controller. A local node of the node corresponding to the
    /// given path is used as the controller, thus any path is accepted as long as process has enough
    /// permission for the path.
    /// ## `path`
    /// A path to any Linux FireWire character device.
    /// ## `scode`
    /// A [`FwScode`][crate::FwScode] to indicate speed of isochronous communication.
    /// ## `channel`
    /// An isochronous channel to transfer, up to 63.
    /// ## `header_size`
    /// The number of bytes for header of IT context.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successful, otherwise FALSE.
    #[doc(alias = "hinoko_fw_iso_tx_allocate")]
    fn allocate(
        &self,
        path: &str,
        scode: FwScode,
        channel: u32,
        header_size: u32,
    ) -> Result<(), glib::Error>;

    /// Map intermediate buffer to share payload of IT context with 1394 OHCI controller.
    /// ## `maximum_bytes_per_payload`
    /// The number of bytes for payload of IT context.
    /// ## `payloads_per_buffer`
    /// The number of payloads of IT context in buffer.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successful, otherwise FALSE.
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
