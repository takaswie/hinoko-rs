// SPDX-License-Identifier: MIT

use super::*;

pub trait FwIsoResourceImpl: ObjectImpl {
    fn open(&self, resource: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error>;
    fn create_source(&self, resource: &Self::Type) -> Result<Source, Error>;
    fn allocate_async(
        &self,
        resource: &Self::Type,
        channel_candidates: &[u8],
        bandwidth: u32,
    ) -> Result<(), Error>;
    fn allocated(&self, resource: &Self::Type, channel: u32, bandwidth: u32, error: Option<&Error>);
    fn deallocated(
        &self,
        resource: &Self::Type,
        channel: u32,
        bandwidth: u32,
        error: Option<&Error>,
    );
}

pub trait FwIsoResourceImplExt: ObjectSubclass {
    fn parent_open(&self, resource: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error>;
    fn parent_create_source(&self, resource: &Self::Type) -> Result<Source, Error>;
    fn parent_allocate_async(
        &self,
        resource: &Self::Type,
        channel_candidates: &[u8],
        bandwidth: u32,
    ) -> Result<(), Error>;
    fn parent_allocated(
        &self,
        resource: &Self::Type,
        channel: u32,
        bandwidth: u32,
        error: Option<&Error>,
    );
    fn parent_deallocated(
        &self,
        resource: &Self::Type,
        channel: u32,
        bandwidth: u32,
        error: Option<&Error>,
    );
}

impl<T: FwIsoResourceImpl> FwIsoResourceImplExt for T {
    fn parent_open(&self, resource: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::HinokoFwIsoResourceInterface;
            let f = (*parent_class)
                .open
                .expect("No parent \"open\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = f(
                resource.unsafe_cast_ref::<FwIsoResource>().to_glib_none().0,
                path.to_glib_none().0,
                open_flag.into(),
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

    fn parent_allocate_async(
        &self,
        resource: &Self::Type,
        channel_candidates: &[u8],
        bandwidth: u32,
    ) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::HinokoFwIsoResourceInterface;
            let f = (*parent_class)
                .allocate_async
                .expect("No parent \"allocate_async\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = f(
                resource.unsafe_cast_ref::<FwIsoResource>().to_glib_none().0,
                channel_candidates.to_glib_none().0,
                channel_candidates.len(),
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
    fn parent_create_source(&self, resource: &Self::Type) -> Result<Source, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::HinokoFwIsoResourceInterface;
            let f = (*parent_class)
                .create_source
                .expect("No parent \"create_source\" implementation");

            let mut source = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let is_ok = f(
                resource.unsafe_cast_ref::<FwIsoResource>().to_glib_none().0,
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

    fn parent_allocated(
        &self,
        resource: &Self::Type,
        channel: u32,
        bandwidth: u32,
        error: Option<&Error>,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::HinokoFwIsoResourceInterface;
            let f = (*parent_class)
                .allocated
                .expect("No parent class implementation for \"allocated\"");
            f(
                resource.unsafe_cast_ref::<FwIsoResource>().to_glib_none().0,
                channel.into(),
                bandwidth.into(),
                error.to_glib_none().0,
            )
        }
    }

    fn parent_deallocated(
        &self,
        resource: &Self::Type,
        channel: u32,
        bandwidth: u32,
        error: Option<&Error>,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::HinokoFwIsoResourceInterface;
            let f = (*parent_class)
                .deallocated
                .expect("No parent class implementation for \"deallocated\"");
            f(
                resource.unsafe_cast_ref::<FwIsoResource>().to_glib_none().0,
                channel.into(),
                bandwidth.into(),
                error.to_glib_none().0,
            )
        }
    }
}

unsafe impl<T: FwIsoResourceImpl> IsImplementable<T> for FwIsoResource {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.open = Some(fw_iso_resource_open::<T>);
        iface.create_source = Some(fw_iso_resource_create_source::<T>);
        iface.allocate_async = Some(fw_iso_resource_allocate_async::<T>);
        iface.allocated = Some(fw_iso_resource_allocated::<T>);
        iface.deallocated = Some(fw_iso_resource_deallocated::<T>);
    }
}

unsafe extern "C" fn fw_iso_resource_open<T: FwIsoResourceImpl>(
    resource: *mut ffi::HinokoFwIsoResource,
    path: *const c_char,
    open_flag: c_int,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(resource as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoResource> = from_glib_borrow(resource);

    match imp.open(
        wrap.unsafe_cast_ref(),
        String::from_glib_none(path).as_str(),
        open_flag.into(),
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

unsafe extern "C" fn fw_iso_resource_create_source<T: FwIsoResourceImpl>(
    resource: *mut ffi::HinokoFwIsoResource,
    source: *mut *mut glib::ffi::GSource,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(resource as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoResource> = from_glib_borrow(resource);

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

unsafe extern "C" fn fw_iso_resource_allocate_async<T: FwIsoResourceImpl>(
    resource: *mut ffi::HinokoFwIsoResource,
    channel_candidates: *const u8,
    channel_candidates_count: size_t,
    bandwidth: c_uint,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(resource as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoResource> = from_glib_borrow(resource);

    match imp.allocate_async(
        wrap.unsafe_cast_ref(),
        std::slice::from_raw_parts(channel_candidates, channel_candidates_count),
        bandwidth.into(),
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

unsafe extern "C" fn fw_iso_resource_allocated<T: FwIsoResourceImpl>(
    resource: *mut ffi::HinokoFwIsoResource,
    channel: c_uint,
    bandwidth: c_uint,
    error: *const glib::ffi::GError,
) {
    let instance = &*(resource as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoResource> = from_glib_borrow(resource);

    imp.allocated(
        wrap.unsafe_cast_ref(),
        channel.into(),
        bandwidth.into(),
        Option::<Error>::from_glib_none(error).as_ref(),
    )
}

unsafe extern "C" fn fw_iso_resource_deallocated<T: FwIsoResourceImpl>(
    resource: *mut ffi::HinokoFwIsoResource,
    channel: c_uint,
    bandwidth: c_uint,
    error: *const glib::ffi::GError,
) {
    let instance = &*(resource as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoResource> = from_glib_borrow(resource);

    imp.deallocated(
        wrap.unsafe_cast_ref(),
        channel.into(),
        bandwidth.into(),
        Option::<Error>::from_glib_none(error).as_ref(),
    )
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::prelude::*, *};
    use glib::{subclass::prelude::*, Error, ParamSpec, ParamSpecOverride, Source, ToValue, Value};

    pub mod imp {
        use super::*;

        #[derive(Default)]
        pub struct FwIsoResourceTest;

        #[glib::object_subclass]
        impl ObjectSubclass for FwIsoResourceTest {
            const NAME: &'static str = "FwIsoResourceTest";
            type Type = super::FwIsoResourceTest;
            type Interfaces = (FwIsoResource,);

            fn new() -> Self {
                Default::default()
            }
        }

        impl ObjectImpl for FwIsoResourceTest {
            fn properties() -> &'static [ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                    vec![ParamSpecOverride::for_interface::<FwIsoResource>(
                        "generation",
                    )]
                });

                PROPERTIES.as_ref()
            }

            fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
                match pspec.name() {
                    "generation" => 1111u32.to_value(),
                    _ => unimplemented!(),
                }
            }
        }

        impl FwIsoResourceImpl for FwIsoResourceTest {
            fn open(
                &self,
                _resource: &Self::Type,
                _path: &str,
                _open_flag: i32,
            ) -> Result<(), Error> {
                Ok(())
            }

            fn create_source(&self, _ctx: &Self::Type) -> Result<Source, Error> {
                Err(Error::new(FwIsoResourceError::Failed, "expected failure"))
            }

            fn allocate_async(
                &self,
                _resource: &Self::Type,
                _channel_candidates: &[u8],
                _bandwidth: u32,
            ) -> Result<(), Error> {
                Ok(())
            }

            fn allocated(
                &self,
                _resource: &Self::Type,
                channel: u32,
                bandwidth: u32,
                error: Option<&Error>,
            ) {
                assert_eq!(channel, 10);
                assert_eq!(bandwidth, 20);
                assert_eq!(error, None);
            }

            fn deallocated(
                &self,
                _resource: &Self::Type,
                channel: u32,
                bandwidth: u32,
                error: Option<&Error>,
            ) {
                assert_eq!(channel, 30);
                assert_eq!(bandwidth, 40);
                assert_eq!(error, None);
            }
        }
    }

    glib::wrapper! {
        pub struct FwIsoResourceTest(ObjectSubclass<imp::FwIsoResourceTest>)
            @implements FwIsoResource;
    }

    #[allow(clippy::new_without_default)]
    impl FwIsoResourceTest {
        pub fn new() -> Self {
            glib::Object::new(&[])
                .expect("Failed creation/initialization of FwIsoResourceTest object")
        }
    }

    #[test]
    fn fw_iso_ctx_iface() {
        let ctx = FwIsoResourceTest::new();

        assert_eq!(ctx.generation(), 1111);

        assert_eq!(ctx.open("blank", 0), Ok(()));
        assert!(ctx.create_source().is_err());
        ctx.emit_allocated(10, 20, None);
        ctx.emit_deallocated(30, 40, None);
    }
}
