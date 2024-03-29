// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@FwIsoCtx`] methods.
///
/// # Implementors
///
/// [`FwIsoCtx`][struct@crate::FwIsoCtx], [`FwIsoIrMultiple`][struct@crate::FwIsoIrMultiple], [`FwIsoIrSingle`][struct@crate::FwIsoIrSingle], [`FwIsoIt`][struct@crate::FwIsoIt]
pub trait FwIsoCtxExtManual {
    /// Retrieve the value of cycle time register. This method call is available once any isochronous
    /// context is created.
    /// ## `clock_id`
    /// The numeric ID of clock source for the reference timestamp. One CLOCK_REALTIME(0),
    ///       CLOCK_MONOTONIC(1), and CLOCK_MONOTONIC_RAW(4) is available in UAPI of Linux kernel.
    /// ## `cycle_time`
    /// A [`hinawa::CycleTime`][crate::hinawa::CycleTime] to store data of cycle time.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    #[doc(alias = "hinoko_fw_iso_ctx_read_cycle_time")]
    fn read_cycle_time(&self, clock_id: i32, cycle_time: &mut CycleTime) -> Result<(), Error>;
}

impl<O: IsA<FwIsoCtx>> FwIsoCtxExtManual for O {
    fn read_cycle_time(&self, clock_id: i32, cycle_time: &mut CycleTime) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::hinoko_fw_iso_ctx_read_cycle_time(
                self.as_ref().to_glib_none().0,
                clock_id,
                &mut cycle_time.to_glib_none_mut().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
