// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of[`struct@FwIsoIrMultiple`] methods.
///
/// # Implementors
///
/// [`FwIsoIrMultiple`][struct@crate::FwIsoIrMultiple]
pub trait FwIsoIrMultipleExtManual {
    /// The array with elements to express isochronous channels to be listened to.
    fn channels(&self) -> Option<Vec<u8>>;

    fn connect_channels_notify<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self) + 'static;

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
    /// ## `chunks_per_irq`
    /// The number of chunks per interval of interrupt. When 0 is given, application
    ///         should call [`FwIsoCtxExt::flush_completions()`][crate::prelude::FwIsoCtxExt::flush_completions()] voluntarily to generate
    ///         [`interrupted`][struct@crate::FwIsoIrMultiple#interrupted] event.
    #[doc(alias = "hinoko_fw_iso_ir_multiple_start")]
    fn start(
        &self,
        cycle_match: Option<&[u16; 2]>,
        sync: u32,
        tags: FwIsoCtxMatchFlag,
        chunks_per_irq: u32,
    ) -> Result<(), Error>;

    /// Retrieve data for packet indicated by the index parameter. The data has isochronous packet header
    /// in its first quadlet, timestamp in its last quadlet. The rest is data of isochronous packet.
    /// ## `index`
    /// the index of packet available in this interrupt.
    ///
    /// # Returns
    ///
    ///
    /// ## `payload`
    /// The array with data frame for payload of
    ///      IR context.
    #[doc(alias = "hinoko_fw_iso_ir_multiple_get_payload")]
    fn payload(&self, index: u32) -> &[u8];
}

impl<O: IsA<FwIsoIrMultiple>> FwIsoIrMultipleExtManual for O {
    fn channels(&self) -> Option<Vec<u8>> {
        unsafe {
            let mut value = Value::from_type(<glib::ByteArray as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"channels\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );

            match value.get::<glib::ByteArray>() {
                Ok(data) => Some(data.to_vec()),
                Err(_) => None,
            }
        }
    }

    fn connect_channels_notify<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self) + 'static,
    {
        unsafe extern "C" fn notify_channels_trampoline<P, F>(
            this: *mut ffi::HinokoFwIsoIrMultiple,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FwIsoIrMultiple>,
            F: Fn(&P) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(&FwIsoIrMultiple::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::channels\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_channels_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }

    fn start(
        &self,
        cycle_match: Option<&[u16; 2]>,
        sync: u32,
        tags: FwIsoCtxMatchFlag,
        chunks_per_irq: u32,
    ) -> Result<(), Error> {
        unsafe {
            let ptr: *const [u16; 2] = match cycle_match {
                Some(data) => data,
                None => std::ptr::null_mut(),
            };
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::hinoko_fw_iso_ir_multiple_start(
                self.as_ref().to_glib_none().0,
                ptr,
                sync,
                tags.into_glib(),
                chunks_per_irq,
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

    fn payload(&self, index: u32) -> &[u8] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const u8;
            let mut size = std::mem::MaybeUninit::uninit();

            ffi::hinoko_fw_iso_ir_multiple_get_payload(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                size.as_mut_ptr(),
            );

            std::slice::from_raw_parts(data, size.assume_init() as usize)
        }
    }
}
