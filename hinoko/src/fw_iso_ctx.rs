// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@FwIsoCtx`] methods.
///
/// # Implementors
///
/// [`FwIsoCtx`][struct@crate::FwIsoCtx], [`FwIsoRxMultiple`][struct@crate::FwIsoRxMultiple], [`FwIsoRxSingle`][struct@crate::FwIsoRxSingle], [`FwIsoTx`][struct@crate::FwIsoTx]
pub trait FwIsoCtxExtManual {
    /// Retrieve the value of cycle timer register. This method call is available
    /// once any isochronous context is created.
    /// ## `clock_id`
    /// The numeric ID of clock source for the reference timestamp. One CLOCK_REALTIME(0),
    ///       CLOCK_MONOTONIC(1), and CLOCK_MONOTONIC_RAW(4) is available in UAPI of Linux kernel.
    /// ## `cycle_timer`
    /// A [`CycleTimer`][crate::CycleTimer] to store data of cycle timer.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    #[doc(alias = "hinoko_fw_iso_ctx_get_cycle_timer")]
    fn get_cycle_timer(&self, clock_id: i32, cycle_timer: &mut CycleTimer) -> Result<(), Error>;
}

impl<O: IsA<FwIsoCtx>> FwIsoCtxExtManual for O {
    fn get_cycle_timer(&self, clock_id: i32, cycle_timer: &mut CycleTimer) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            ffi::hinoko_fw_iso_ctx_get_cycle_timer(
                self.as_ref().to_glib_none().0,
                clock_id,
                &mut cycle_timer.to_glib_none_mut().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
