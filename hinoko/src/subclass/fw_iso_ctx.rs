// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`FwIsoCtx`][crate::FwIsoCtx].
pub trait FwIsoCtxImpl: ObjectImpl {
    fn create_source(&self, ctx: &Self::Type) -> Result<Source, Error>;
    fn flush_completions(&self, ctx: &Self::Type) -> Result<(), Error>;
    fn read_cycle_time(
        &self,
        ctx: &Self::Type,
        clock_id: i32,
        cycle_time: &mut CycleTime,
    ) -> Result<(), Error>;
    fn release(&self, ctx: &Self::Type);
    fn stop(&self, ctx: &Self::Type);
    fn unmap_buffer(&self, ctx: &Self::Type);
    fn stopped(&self, ctx: &Self::Type, error: Option<&Error>);
}

/// Trait which is automatically implemented to implementator of
/// [`FwIsoCtxImpl`][self::FwIsoCtxImpl].
pub trait FwIsoCtxImplExt: ObjectSubclass {
    fn parent_create_source(&self, ctx: &Self::Type) -> Result<Source, Error>;
    fn parent_flush_completions(&self, ctx: &Self::Type) -> Result<(), Error>;
    fn parent_read_cycle_time(
        &self,
        ctx: &Self::Type,
        clock_id: i32,
        cycle_time: &mut CycleTime,
    ) -> Result<(), Error>;
    fn parent_release(&self, ctx: &Self::Type);
    fn parent_stop(&self, ctx: &Self::Type);
    fn parent_unmap_buffer(&self, ctx: &Self::Type);
    fn parent_stopped(&self, ctx: &Self::Type, error: Option<&Error>);
}

impl<T: FwIsoCtxImpl> FwIsoCtxImplExt for T {
    fn parent_create_source(&self, ctx: &Self::Type) -> Result<Source, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoCtxInterface;
            let f = (*parent_class)
                .create_source
                .expect("No parent \"create_source\" implementation");

            let mut source = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let is_ok = f(
                ctx.unsafe_cast_ref::<FwIsoCtx>().to_glib_none().0,
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

    fn parent_flush_completions(&self, ctx: &Self::Type) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoCtxInterface;
            let f = (*parent_class)
                .flush_completions
                .expect("No parent \"flush_completions\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = f(
                ctx.unsafe_cast_ref::<FwIsoCtx>().to_glib_none().0,
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

    fn parent_read_cycle_time(
        &self,
        ctx: &Self::Type,
        clock_id: i32,
        cycle_time: &mut CycleTime,
    ) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoCtxInterface;
            let f = (*parent_class)
                .read_cycle_time
                .expect("No parent \"flush_completions\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = f(
                ctx.unsafe_cast_ref::<FwIsoCtx>().to_glib_none().0,
                clock_id.into(),
                &mut cycle_time.to_glib_none_mut().0,
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

    fn parent_release(&self, ctx: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoCtxInterface;
            let f = (*parent_class)
                .release
                .expect("No parent \"release\" implementation");

            f(ctx.unsafe_cast_ref::<FwIsoCtx>().to_glib_none().0)
        }
    }

    fn parent_stop(&self, ctx: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoCtxInterface;
            let f = (*parent_class)
                .stop
                .expect("No parent \"stop\" implementation");

            f(ctx.unsafe_cast_ref::<FwIsoCtx>().to_glib_none().0)
        }
    }

    fn parent_unmap_buffer(&self, ctx: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoCtxInterface;
            let f = (*parent_class)
                .unmap_buffer
                .expect("No parent \"unmap_buffer\" implementation");

            f(ctx.unsafe_cast_ref::<FwIsoCtx>().to_glib_none().0)
        }
    }

    fn parent_stopped(&self, ctx: &Self::Type, error: Option<&Error>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoCtxInterface;
            let f = (*parent_class)
                .stopped
                .expect("No parent \"stopped\" implementation");

            f(
                ctx.unsafe_cast_ref::<FwIsoCtx>().to_glib_none().0,
                error.to_glib_none().0,
            )
        }
    }
}

unsafe impl<T: FwIsoCtxImpl> IsImplementable<T> for FwIsoCtx {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.create_source = Some(fw_iso_ctx_create_source::<T>);
        iface.flush_completions = Some(fw_iso_ctx_flush_completions::<T>);
        iface.read_cycle_time = Some(fw_iso_ctx_read_cycle_time::<T>);
        iface.release = Some(fw_iso_ctx_release::<T>);
        iface.stop = Some(fw_iso_ctx_stop::<T>);
        iface.unmap_buffer = Some(fw_iso_ctx_unmap_buffer::<T>);
        iface.stopped = Some(fw_iso_ctx_stopped::<T>);
    }
}

unsafe extern "C" fn fw_iso_ctx_create_source<T: FwIsoCtxImpl>(
    ctx: *mut ffi::HinokoFwIsoCtx,
    source: *mut *mut glib::ffi::GSource,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoCtx> = from_glib_borrow(ctx);

    match imp.create_source(wrap.unsafe_cast_ref()) {
        Ok(src) => {
            *source = src.to_glib_none().0;
            glib::ffi::GTRUE
        }
        Err(err) => {
            if !error.is_null() {
                *error = err.into_raw();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn fw_iso_ctx_flush_completions<T: FwIsoCtxImpl>(
    ctx: *mut ffi::HinokoFwIsoCtx,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoCtx> = from_glib_borrow(ctx);

    match imp.flush_completions(wrap.unsafe_cast_ref()) {
        Ok(_) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_raw();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn fw_iso_ctx_read_cycle_time<T: FwIsoCtxImpl>(
    ctx: *mut ffi::HinokoFwIsoCtx,
    clock_id: c_int,
    cycle_time: *const *mut hinawa::ffi::HinawaCycleTime,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoCtx> = from_glib_borrow(ctx);

    match imp.read_cycle_time(
        wrap.unsafe_cast_ref(),
        clock_id.into(),
        &mut from_glib_none(*cycle_time),
    ) {
        Ok(_) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_raw();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn fw_iso_ctx_release<T: FwIsoCtxImpl>(ctx: *mut ffi::HinokoFwIsoCtx) {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoCtx> = from_glib_borrow(ctx);

    imp.release(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn fw_iso_ctx_stop<T: FwIsoCtxImpl>(ctx: *mut ffi::HinokoFwIsoCtx) {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoCtx> = from_glib_borrow(ctx);

    imp.stop(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn fw_iso_ctx_unmap_buffer<T: FwIsoCtxImpl>(ctx: *mut ffi::HinokoFwIsoCtx) {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoCtx> = from_glib_borrow(ctx);

    imp.unmap_buffer(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn fw_iso_ctx_stopped<T: FwIsoCtxImpl>(
    ctx: *mut ffi::HinokoFwIsoCtx,
    error: *const glib::ffi::GError,
) {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoCtx> = from_glib_borrow(ctx);

    imp.stopped(
        wrap.unsafe_cast_ref(),
        Option::<Error>::from_glib_none(error).as_ref(),
    )
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::prelude::*, *};
    use glib::{subclass::prelude::*, Error, ParamSpec, ParamSpecOverride, Source, ToValue, Value};

    pub mod imp {
        use super::*;

        pub struct FwIsoCtxTest;

        impl Default for FwIsoCtxTest {
            fn default() -> Self {
                Self {}
            }
        }

        #[glib::object_subclass]
        impl ObjectSubclass for FwIsoCtxTest {
            const NAME: &'static str = "FwIsoCtxTest";
            type Type = super::FwIsoCtxTest;
            type Interfaces = (FwIsoCtx,);

            fn new() -> Self {
                Default::default()
            }
        }

        impl ObjectImpl for FwIsoCtxTest {
            fn properties() -> &'static [ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                    vec![
                        ParamSpecOverride::for_interface::<FwIsoCtx>("bytes-per-chunk"),
                        ParamSpecOverride::for_interface::<FwIsoCtx>("chunks-per-buffer"),
                    ]
                });

                PROPERTIES.as_ref()
            }

            fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
                match pspec.name() {
                    "bytes-per-chunk" => 1u32.to_value(),
                    "chunks-per-buffer" => 2u32.to_value(),
                    _ => unimplemented!(),
                }
            }
        }

        impl FwIsoCtxImpl for FwIsoCtxTest {
            fn create_source(&self, _ctx: &Self::Type) -> Result<Source, Error> {
                Err(Error::new(FwIsoCtxError::Failed, "expected failure"))
            }

            fn flush_completions(&self, _ctx: &Self::Type) -> Result<(), Error> {
                Ok(())
            }

            fn read_cycle_time(
                &self,
                _ctx: &Self::Type,
                _clock_id: i32,
                _cycle_time: &mut CycleTime,
            ) -> Result<(), Error> {
                Ok(())
            }

            fn release(&self, _ctx: &Self::Type) {}

            fn stop(&self, _ctx: &Self::Type) {}

            fn unmap_buffer(&self, _ctx: &Self::Type) {}

            fn stopped(&self, _ctx: &Self::Type, error: Option<&Error>) {
                assert_eq!(error, None);
            }
        }
    }

    glib::wrapper! {
        pub struct FwIsoCtxTest(ObjectSubclass<imp::FwIsoCtxTest>)
            @implements FwIsoCtx;
    }

    #[allow(clippy::new_without_default)]
    impl FwIsoCtxTest {
        pub fn new() -> Self {
            glib::Object::new(&[]).expect("Failed creation/initialization of FwIsoCtxTest object")
        }
    }

    #[test]
    fn fw_iso_ctx_iface() {
        let ctx = FwIsoCtxTest::new();

        assert!(ctx.create_source().is_err());

        assert_eq!(ctx.flush_completions(), Ok(()));
        assert_eq!(ctx.release(), (()));
        assert_eq!(ctx.stop(), (()));
        assert_eq!(ctx.unmap_buffer(), (()));

        let mut cycle_time = CycleTime::new();
        assert_eq!(ctx.read_cycle_time(0, &mut cycle_time), Ok(()));

        assert_eq!(ctx.bytes_per_chunk(), 1);
        assert_eq!(ctx.chunks_per_buffer(), 2);

        ctx.emit_stopped(None);
    }
}
