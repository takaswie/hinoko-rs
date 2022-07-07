// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@FwIsoRxSingle`] methods.
///
/// # Implementors
///
/// [`FwIsoRxSingle`][struct@crate::FwIsoRxSingle]
pub trait FwIsoRxSingleExtManual {
    /// Start IR context.
    /// ## `cycle_match`
    /// The isochronous cycle
    ///      to start packet processing. The first element should be the second part of
    ///      isochronous cycle, up to 3. The second element should be the cycle part of
    ///      isochronous cycle, up to 7999.
    /// ## `sync_code`
    /// The value of sy field in isochronous packet header for packet processing, up to 15.
    /// ## `tags`
    /// The value of tag field in isochronous header for packet processing.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    #[doc(alias = "hinoko_fw_iso_rx_single_start")]
    fn start(
        &self,
        cycle_match: Option<&[u16; 2]>,
        sync: u32,
        tags: FwIsoCtxMatchFlag,
    ) -> Result<(), Error>;

    /// Retrieve payload of IR context for a handled packet corresponding to index at the event of
    /// interrupt.
    /// ## `index`
    /// the index inner available packets at the event of interrupt.
    ///
    /// # Returns
    ///
    ///
    /// ## `payload`
    /// The array with data
    ///      frame for payload of IR context.
    #[doc(alias = "hinoko_fw_iso_rx_single_get_payload")]
    #[doc(alias = "get_payload")]
    fn payload(&self, index: u32) -> &[u8];

    /// Emitted when Linux FireWire subsystem generates interrupt event. There are three cases
    /// for Linux FireWire subsystem to generate the event:
    ///
    /// - When OHCI 1394 controller generates hardware interrupt as a result to process the
    ///   isochronous packet for the buffer chunk marked to generate hardware interrupt.
    /// - When the size of accumulated context header for packets since the last event reaches
    ///   the size of memory page (usually 4,096 bytes).
    /// - When application calls [`FwIsoCtxExt::flush_completions()`][crate::prelude::FwIsoCtxExt::flush_completions()] explicitly.
    ///
    /// The handler of signal can retrieve context payload of received packet by call of
    /// [`FwIsoRxSingleExtManual::payload()`][crate::prelude::FwIsoRxSingleExtManual::payload()].
    /// ## `sec`
    /// sec part of isochronous cycle when interrupt occurs, up to 7.
    /// ## `cycle`
    /// cycle part of of isochronous cycle when interrupt occurs, up to 7999.
    /// ## `header`
    /// The headers of IR context
    ///     for packets handled in the event of interrupt. The content is different
    ///     depending on header_size parameter of [`FwIsoRxSingleExt::allocate()`][crate::prelude::FwIsoRxSingleExt::allocate()].
    /// ## `count`
    /// the number of packets to handle.
    #[doc(alias = "interrupted")]
    fn connect_interrupted<F: Fn(&Self, u32, u32, &[u8], u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<FwIsoRxSingle>> FwIsoRxSingleExtManual for O {
    fn start(
        &self,
        cycle_match: Option<&[u16; 2]>,
        sync: u32,
        tags: FwIsoCtxMatchFlag,
    ) -> Result<(), Error> {
        unsafe {
            let ptr: *const [u16; 2] = match cycle_match {
                Some(data) => data,
                None => std::ptr::null_mut(),
            };
            let mut error = std::ptr::null_mut();

            ffi::hinoko_fw_iso_rx_single_start(
                self.as_ref().to_glib_none().0,
                ptr,
                sync,
                tags.into_glib(),
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn payload(&self, index: u32) -> &[u8] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const u8;
            let mut size = std::mem::MaybeUninit::uninit();

            ffi::hinoko_fw_iso_rx_single_get_payload(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                size.as_mut_ptr(),
            );

            std::slice::from_raw_parts(data, size.assume_init() as usize)
        }
    }

    fn connect_interrupted<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, u32, u32, &[u8], u32) + 'static,
    {
        unsafe extern "C" fn interrupted_trampoline<P, F>(
            this: *mut ffi::HinokoFwIsoRxSingle,
            sec: c_uint,
            cycle: c_uint,
            header: *const u8,
            header_length: c_uint,
            count: c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FwIsoRxSingle>,
            F: Fn(&P, u32, u32, &[u8], u32) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &FwIsoRxSingle::from_glib_borrow(this).unsafe_cast_ref(),
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
