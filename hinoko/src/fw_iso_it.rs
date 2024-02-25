// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of[`struct@FwIsoIt`] methods.
///
/// # Implementors
///
/// [`FwIsoIt`][struct@crate::FwIsoIt]
pub trait FwIsoItExtManual {
    /// Register packet data with header and payload for IT context. The content of given header and
    /// payload is appended into data field of isochronous packet to be sent. The caller can schedule
    /// hardware interrupt to generate interrupt event. In detail, please refer to documentation about
    /// [`interrupted`][struct@crate::FwIsoIt#interrupted].
    /// ## `tags`
    /// The value of tag field for isochronous packet to register.
    /// ## `sync_code`
    /// The value of sync field in isochronous packet header for packet processing, up to 15.
    /// ## `header`
    /// The header of IT context for isochronous
    ///     packet. The length of header should be the same as the size of header indicated in
    ///     allocation if it's not null.
    /// ## `payload`
    /// The payload of IT context for isochronous
    ///      packet.
    /// ## `schedule_interrupt`
    /// Whether to schedule hardware interrupt at isochronous cycle for the packet.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successful, otherwise FALSE.
    #[doc(alias = "hinoko_fw_iso_it_register_packet")]
    fn register_packet(
        &self,
        tags: FwIsoCtxMatchFlag,
        sy: u32,
        header: Option<&[u8]>,
        payload: Option<&[u8]>,
        schedule_interrupt: bool,
    ) -> Result<(), Error>;

    /// Start IT context.
    /// ## `cycle_match`
    /// The isochronous cycle
    ///      to start packet processing. The first element should be the second part of
    ///      isochronous cycle, up to 3. The second element should be the cycle part of
    ///      isochronous cycle, up to 7999.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successful, otherwise FALSE.
    #[doc(alias = "hinoko_fw_iso_it_start")]
    fn start(&self, cycle_match: Option<&[u16; 2]>) -> Result<(), Error>;

    /// Emitted when Linux FireWire subsystem generates interrupt event. There are three cases
    /// for Linux FireWire subsystem to generate the event:
    ///
    /// - When 1394 OHCI hardware generates hardware interrupt as a result of processing the
    ///   isochronous packet for the buffer chunk marked to generate hardware interrupt.
    /// - When the number of isochronous packets sent since the last interrupt event reaches
    ///   one quarter of memory page size (usually 4,096 / 4 = 1,024 packets).
    /// - When application calls [`FwIsoCtxExt::flush_completions()`][crate::prelude::FwIsoCtxExt::flush_completions()] explicitly.
    /// ## `sec`
    /// sec part of isochronous cycle when interrupt occurs, up to 7.
    /// ## `cycle`
    /// cycle part of of isochronous cycle when interrupt occurs, up to 7999.
    /// ## `tstamp`
    /// A series of timestamps for
    ///     packets already handled.
    /// ## `count`
    /// the number of handled packets.
    #[doc(alias = "interrupted")]
    fn connect_interrupted<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, u32, u32, &[u8], u32) + 'static;
}

impl<O: IsA<FwIsoIt>> FwIsoItExtManual for O {
    fn register_packet(
        &self,
        tags: FwIsoCtxMatchFlag,
        sy: u32,
        header: Option<&[u8]>,
        payload: Option<&[u8]>,
        schedule_interrupt: bool,
    ) -> Result<(), Error> {
        let (header_ptr, header_length) = match header {
            Some(h) => (h.as_ptr(), h.len() as u32),
            None => (std::ptr::null(), 0),
        };
        let (payload_ptr, payload_length) = match payload {
            Some(p) => (p.as_ptr(), p.len() as u32),
            None => (std::ptr::null(), 0),
        };

        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::hinoko_fw_iso_it_register_packet(
                self.as_ref().to_glib_none().0,
                tags.into_glib(),
                sy,
                header_ptr,
                header_length,
                payload_ptr,
                payload_length,
                schedule_interrupt.into_glib(),
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

    fn start(&self, cycle_match: Option<&[u16; 2]>) -> Result<(), Error> {
        unsafe {
            let ptr: *const [u16; 2] = match cycle_match {
                Some(data) => data,
                None => std::ptr::null_mut(),
            };
            let mut error = std::ptr::null_mut();

            let is_ok =
                ffi::hinoko_fw_iso_it_start(self.as_ref().to_glib_none().0, ptr, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_interrupted<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, u32, u32, &[u8], u32) + 'static,
    {
        unsafe extern "C" fn interrupted_trampoline<P, F>(
            this: *mut ffi::HinokoFwIsoIt,
            sec: c_uint,
            cycle: c_uint,
            header: *const u8,
            header_length: c_uint,
            count: c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FwIsoIt>,
            F: Fn(&P, u32, u32, &[u8], u32) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &FwIsoIt::from_glib_borrow(this).unsafe_cast_ref(),
                sec,
                cycle,
                std::slice::from_raw_parts(header, header_length as usize),
                count,
            )
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"interrupted\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    interrupted_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }
}
