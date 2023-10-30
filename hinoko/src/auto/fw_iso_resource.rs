// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::FwScode;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    /// A set of basic interfaces to listen to events about isochronous resource.
    ///
    /// [`FwIsoResource`][crate::FwIsoResource] includes interfaces to listen to events about allocation and
    /// deallocation of isochronous resource.
    ///
    /// # Implements
    ///
    /// [`FwIsoResourceExt`][trait@crate::prelude::FwIsoResourceExt]
    #[doc(alias = "HinokoFwIsoResource")]
    pub struct FwIsoResource(Interface<ffi::HinokoFwIsoResource, ffi::HinokoFwIsoResourceInterface>);

    match fn {
        type_ => || ffi::hinoko_fw_iso_resource_get_type(),
    }
}

impl FwIsoResource {
    pub const NONE: Option<&'static FwIsoResource> = None;

    /// Calculate the amount of bandwidth expected to be consumed in allocation unit
    /// by given parameters.
    /// ## `bytes_per_payload`
    /// The number of bytes in payload of isochronous packet.
    /// ## `scode`
    /// The speed of transmission.
    ///
    /// # Returns
    ///
    /// The amount of bandwidth expected to be consumed.
    #[doc(alias = "hinoko_fw_iso_resource_calculate_bandwidth")]
    pub fn calculate_bandwidth(bytes_per_payload: u32, scode: FwScode) -> u32 {
        unsafe {
            ffi::hinoko_fw_iso_resource_calculate_bandwidth(bytes_per_payload, scode.into_glib())
        }
    }
}

/// Trait containing all [`struct@FwIsoResource`] methods.
///
/// # Implementors
///
/// [`FwIsoResourceAuto`][struct@crate::FwIsoResourceAuto], [`FwIsoResourceOnce`][struct@crate::FwIsoResourceOnce], [`FwIsoResource`][struct@crate::FwIsoResource]
pub trait FwIsoResourceExt: 'static {
    /// Initiate allocation of isochronous resource without any wait. One of the candidates is actually
    /// allocated for channel. When the allocation finishes, `signal::FwIsoResource::allocated` signal is
    /// emitted to notify the result, channel, and bandwidth.
    /// ## `channel_candidates`
    /// The array with elements for
    ///         numeric number of isochronous channel to be allocated.
    /// ## `bandwidth`
    /// The amount of bandwidth to be allocated.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    #[doc(alias = "hinoko_fw_iso_resource_allocate")]
    fn allocate(&self, channel_candidates: &[u8], bandwidth: u32) -> Result<(), glib::Error>;

    /// Initiate allocation of isochronous resource and wait for `signal::FwIsoResource::allocated`
    /// signal. One of the candidates is actually allocated for channel.
    /// ## `channel_candidates`
    /// The array with elements for
    ///         numeric number for isochronous channel to be allocated.
    /// ## `bandwidth`
    /// The amount of bandwidth to be allocated.
    /// ## `timeout_ms`
    /// The timeout to wait for allocated event.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    #[doc(alias = "hinoko_fw_iso_resource_allocate_wait")]
    fn allocate_wait(
        &self,
        channel_candidates: &[u8],
        bandwidth: u32,
        timeout_ms: u32,
    ) -> Result<(), glib::Error>;

    /// Create [`glib::Source`][crate::glib::Source] for `GLib::MainContext` to dispatch events for isochronous
    /// resource.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, otherwise FALSE.
    ///
    /// ## `source`
    /// A [`glib::Source`][crate::glib::Source]
    #[doc(alias = "hinoko_fw_iso_resource_create_source")]
    fn create_source(&self) -> Result<glib::Source, glib::Error>;

    /// Open Linux FireWire character device to delegate any request for isochronous
    /// resource management to Linux FireWire subsystem.
    /// ## `path`
    /// A path of any Linux FireWire character device.
    /// ## `open_flag`
    /// The flag of open(2) system call. O_RDONLY is forced to fulfil
    ///        internally.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finished successfully, otherwise FALSE.
    #[doc(alias = "hinoko_fw_iso_resource_open")]
    fn open(&self, path: &str, open_flag: i32) -> Result<(), glib::Error>;

    fn generation(&self) -> u32;

    /// Emitted when allocation of isochronous resource finishes.
    /// ## `channel`
    /// The deallocated channel number.
    /// ## `bandwidth`
    /// The deallocated amount of bandwidth.
    /// ## `error`
    /// A [`glib::Error`][crate::glib::Error]. Error can be generated
    ///    with domain of [`FwIsoResourceError`][crate::FwIsoResourceError] and its EVENT code.
    #[doc(alias = "allocated")]
    fn connect_allocated<F: Fn(&Self, u32, u32, Option<&glib::Error>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_allocated(&self, channel: u32, bandwidth: u32, error: Option<&glib::Error>);

    /// Emitted when deallocation of isochronous resource finishes.
    /// ## `channel`
    /// The deallocated channel number.
    /// ## `bandwidth`
    /// The deallocated amount of bandwidth.
    /// ## `error`
    /// A [`glib::Error`][crate::glib::Error]. Error can be generated
    ///    with domain of [`FwIsoResourceError`][crate::FwIsoResourceError] and its EVENT code.
    #[doc(alias = "deallocated")]
    fn connect_deallocated<F: Fn(&Self, u32, u32, Option<&glib::Error>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_deallocated(&self, channel: u32, bandwidth: u32, error: Option<&glib::Error>);

    #[doc(alias = "generation")]
    fn connect_generation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FwIsoResource>> FwIsoResourceExt for O {
    fn allocate(&self, channel_candidates: &[u8], bandwidth: u32) -> Result<(), glib::Error> {
        let channel_candidates_count = channel_candidates.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::hinoko_fw_iso_resource_allocate(
                self.as_ref().to_glib_none().0,
                channel_candidates.to_glib_none().0,
                channel_candidates_count,
                bandwidth,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn allocate_wait(
        &self,
        channel_candidates: &[u8],
        bandwidth: u32,
        timeout_ms: u32,
    ) -> Result<(), glib::Error> {
        let channel_candidates_count = channel_candidates.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::hinoko_fw_iso_resource_allocate_wait(
                self.as_ref().to_glib_none().0,
                channel_candidates.to_glib_none().0,
                channel_candidates_count,
                bandwidth,
                timeout_ms,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_source(&self) -> Result<glib::Source, glib::Error> {
        unsafe {
            let mut source = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::hinoko_fw_iso_resource_create_source(
                self.as_ref().to_glib_none().0,
                &mut source,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib_full(source))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn open(&self, path: &str, open_flag: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::hinoko_fw_iso_resource_open(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                open_flag,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn generation(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "generation")
    }

    fn connect_allocated<F: Fn(&Self, u32, u32, Option<&glib::Error>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn allocated_trampoline<
            P: IsA<FwIsoResource>,
            F: Fn(&P, u32, u32, Option<&glib::Error>) + 'static,
        >(
            this: *mut ffi::HinokoFwIsoResource,
            channel: libc::c_uint,
            bandwidth: libc::c_uint,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                FwIsoResource::from_glib_borrow(this).unsafe_cast_ref(),
                channel,
                bandwidth,
                Option::<glib::Error>::from_glib_borrow(error)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"allocated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    allocated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_allocated(&self, channel: u32, bandwidth: u32, error: Option<&glib::Error>) {
        self.emit_by_name::<()>("allocated", &[&channel, &bandwidth, &error]);
    }

    fn connect_deallocated<F: Fn(&Self, u32, u32, Option<&glib::Error>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn deallocated_trampoline<
            P: IsA<FwIsoResource>,
            F: Fn(&P, u32, u32, Option<&glib::Error>) + 'static,
        >(
            this: *mut ffi::HinokoFwIsoResource,
            channel: libc::c_uint,
            bandwidth: libc::c_uint,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                FwIsoResource::from_glib_borrow(this).unsafe_cast_ref(),
                channel,
                bandwidth,
                Option::<glib::Error>::from_glib_borrow(error)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deallocated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deallocated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_deallocated(&self, channel: u32, bandwidth: u32, error: Option<&glib::Error>) {
        self.emit_by_name::<()>("deallocated", &[&channel, &bandwidth, &error]);
    }

    fn connect_generation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_generation_trampoline<
            P: IsA<FwIsoResource>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HinokoFwIsoResource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwIsoResource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::generation\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_generation_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FwIsoResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FwIsoResource")
    }
}
