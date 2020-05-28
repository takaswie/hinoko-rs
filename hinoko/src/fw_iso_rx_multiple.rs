use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use hinoko_sys;

use FwIsoCtxMatchFlag;
use FwIsoRxMultiple;

pub trait FwIsoRxMultipleExtManual {
    fn get_property_channels(&self) -> Option<Vec<u8>>;
    fn connect_property_channels_notify<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self) + 'static;

    fn start(
        &self,
        cycle_match: Option<&[u16; 2]>,
        sync: u32,
        tags: FwIsoCtxMatchFlag,
        chunks_per_irq: u32,
    ) -> Result<(), glib::Error>;

    fn get_payload(&self, index: u32) -> Result<&[u8], glib::Error>;
}

impl<O: IsA<FwIsoRxMultiple>> FwIsoRxMultipleExtManual for O {
    fn get_property_channels(&self) -> Option<Vec<u8>> {
        unsafe {
            let mut value = Value::from_type(<glib::ByteArray as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"channels\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );

            match value.get::<glib::ByteArray>() {
                Ok(content) => match content {
                    Some(data) => {
                        let mut channels = Vec::<u8>::new();
                        channels.extend_from_slice(&data);
                        Some(channels)
                    }
                    None => None,
                },
                Err(_) => None,
            }
        }
    }

    fn connect_property_channels_notify<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self) + 'static,
    {
        unsafe extern "C" fn notify_channels_trampoline<P, F>(
            this: *mut hinoko_sys::HinokoFwIsoRxMultiple,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FwIsoRxMultiple>,
            F: Fn(&P) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(&FwIsoRxMultiple::from_glib_borrow(this).unsafe_cast_ref())
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
    ) -> Result<(), glib::Error> {
        unsafe {
            let ptr: *const [u16; 2] = match cycle_match {
                Some(data) => data,
                None => std::ptr::null_mut(),
            };
            let mut error = std::ptr::null_mut();

            hinoko_sys::hinoko_fw_iso_rx_multiple_start(
                self.as_ref().to_glib_none().0,
                ptr,
                sync,
                tags.to_glib(),
                chunks_per_irq,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_payload(&self, index: u32) -> Result<&[u8], glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let mut data = std::ptr::null_mut() as *const u8;
            let mut size = std::mem::MaybeUninit::uninit();

            hinoko_sys::hinoko_fw_iso_rx_multiple_get_payload(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                size.as_mut_ptr(),
                &mut error,
            );

            if error.is_null() {
                Ok(std::slice::from_raw_parts(
                    data,
                    size.assume_init() as usize,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
