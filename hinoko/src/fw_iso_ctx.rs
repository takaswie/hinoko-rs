use glib::object::IsA;
use glib::translate::*;

use CycleTimer;
use FwIsoCtx;

pub trait FwIsoCtxExtManual {
    fn get_cycle_timer(
        &self,
        clock_id: i32,
        cycle_timer: &mut CycleTimer,
    ) -> Result<(), glib::Error>;
}

impl<O: IsA<FwIsoCtx>> FwIsoCtxExtManual for O {
    fn get_cycle_timer(
        &self,
        clock_id: i32,
        cycle_timer: &mut CycleTimer,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            hinoko_sys::hinoko_fw_iso_ctx_get_cycle_timer(
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
