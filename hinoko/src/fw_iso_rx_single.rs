use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use hinoko_sys;
use libc::*;

use FwIsoCtxMatchFlag;
use FwIsoRxSingle;

pub trait FwIsoRxSingleExtManual {
    fn start(
        &self,
        cycle_match: Option<&[u16; 2]>,
        sync: u32,
        tags: FwIsoCtxMatchFlag,
        packets_per_irq: u32,
    ) -> Result<(), glib::Error>;

    fn get_payload(&self, index: u32) -> Result<&[u8], glib::Error>;
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
        packets_per_irq: u32,
    ) -> Result<(), glib::Error> {
        unsafe {
            let ptr: *const [u16; 2] = match cycle_match {
                Some(data) => data,
                None => std::ptr::null_mut(),
            };
            let mut error = std::ptr::null_mut();

            hinoko_sys::hinoko_fw_iso_rx_single_start(
                self.as_ref().to_glib_none().0,
                ptr,
                sync,
                tags.to_glib(),
                packets_per_irq,
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

            hinoko_sys::hinoko_fw_iso_rx_single_get_payload(
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

    fn connect_interrupted<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, u32, u32, &[u8], u32) + 'static,
    {
        unsafe extern "C" fn interrupted_trampoline<P, F>(
            this: *mut hinoko_sys::HinokoFwIsoRxSingle,
            sec: c_uint,
            cycle: c_uint,
            header: *const u8,
            header_length: c_uint,
            count: c_uint,
            f: glib_sys::gpointer,
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
