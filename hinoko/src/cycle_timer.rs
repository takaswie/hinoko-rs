// SPDX-License-Identifier: MIT
use crate::*;

impl CycleTimer {
    /// Get the value of cycle timer in 1394 OHCI controller. The first element of array expresses the
    /// value of sec field, up to 127. The second element of array expresses the value of cycle field,
    /// up to 7999. The third element of array expresses the value of offset field, up to 3071.
    ///
    /// # Returns
    ///
    ///
    /// ## `cycle_timer`
    /// The value of cycle timer register of 1394 OHCI, including three elements; second, cycle, and
    /// offset.
    #[doc(alias = "hinoko_cycle_timer_get_cycle_timer")]
    #[doc(alias = "get_cycle_timer")]
    pub fn cycle_timer(&mut self) -> [u16; 3] {
        unsafe {
            let mut cycle_timer = [0; 3];

            ffi::hinoko_cycle_timer_get_cycle_timer(self.to_glib_none_mut().0, &mut cycle_timer);

            cycle_timer
        }
    }
}
