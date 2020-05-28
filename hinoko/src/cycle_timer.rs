use glib::translate::*;

use CycleTimer;

impl CycleTimer {
    pub fn get_cycle_timer(&mut self) -> [u16; 3] {
        unsafe {
            let mut cycle_timer = [0; 3];

            hinoko_sys::hinoko_cycle_timer_get_cycle_timer(
                self.to_glib_none_mut().0,
                &mut cycle_timer,
            );

            cycle_timer
        }
    }
}
