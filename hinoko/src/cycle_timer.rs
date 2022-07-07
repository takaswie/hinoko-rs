// SPDX-License-Identifier: MIT
use crate::*;

impl CycleTimer {
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
