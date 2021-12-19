use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use hinoko_sys;
use libc::*;

use crate::{FwIsoCtxMatchFlag, FwIsoTx};

pub trait FwIsoTxExtManual {
    fn register_packet(
        &self,
        tags: FwIsoCtxMatchFlag,
        sy: u32,
        header: Option<&[u8]>,
        payload: Option<&[u8]>,
        schedule_interrupt: bool
    ) -> Result<(), glib::Error>;

    fn start(&self, cycle_match: Option<&[u16; 2]>) -> Result<(), glib::Error>;

    fn connect_interrupted<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, u32, u32, &[u8], u32) + 'static;
}

impl<O: IsA<FwIsoTx>> FwIsoTxExtManual for O {
    fn register_packet(
        &self,
        tags: FwIsoCtxMatchFlag,
        sy: u32,
        header: Option<&[u8]>,
        payload: Option<&[u8]>,
        schedule_interrupt: bool
    ) -> Result<(), glib::Error> {
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
            let _ = hinoko_sys::hinoko_fw_iso_tx_register_packet(
                self.as_ref().to_glib_none().0,
                tags.to_glib(),
                sy,
                header_ptr,
                header_length,
                payload_ptr,
                payload_length,
                schedule_interrupt.to_glib(),
                &mut error
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn start(&self, cycle_match: Option<&[u16; 2]>) -> Result<(), glib::Error> {
        unsafe {
            let ptr: *const [u16; 2] = match cycle_match {
                Some(data) => data,
                None => std::ptr::null_mut(),
            };
            let mut error = std::ptr::null_mut();

            hinoko_sys::hinoko_fw_iso_tx_start(
                self.as_ref().to_glib_none().0,
                ptr,
                &mut error,
            );

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
            this: *mut hinoko_sys::HinokoFwIsoTx,
            sec: c_uint,
            cycle: c_uint,
            header: *const u8,
            header_length: c_uint,
            count: c_uint,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FwIsoTx>,
            F: Fn(&P, u32, u32, &[u8], u32) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &FwIsoTx::from_glib_borrow(this).unsafe_cast_ref(),
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
