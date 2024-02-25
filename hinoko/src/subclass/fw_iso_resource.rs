// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`FwIsoResource`][crate::FwIsoResource].
pub trait FwIsoResourceImpl: ObjectImpl {
    fn open(&self, resource: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error>;
    fn create_source(&self, resource: &Self::Type) -> Result<Source, Error>;
    fn allocate(
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

/// Trait which is automatically implemented to implementator of [`FwIsoResourceImpl`][self::FwIsoResourceImpl].
pub trait FwIsoResourceImplExt: ObjectSubclass {
    fn parent_open(&self, resource: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error>;
    fn parent_create_source(&self, resource: &Self::Type) -> Result<Source, Error>;
    fn parent_allocate(
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
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_allocate(
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
                .allocate
                .expect("No parent \"allocate\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = f(
                resource.unsafe_cast_ref::<FwIsoResource>().to_glib_none().0,
                channel_candidates.to_glib_none().0,
                channel_candidates.len(),
                bandwidth,
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
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
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
        iface.allocate = Some(fw_iso_resource_allocate::<T>);
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
                *error = err.into_glib_ptr();
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
                *error = err.into_glib_ptr();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn fw_iso_resource_allocate<T: FwIsoResourceImpl>(
    resource: *mut ffi::HinokoFwIsoResource,
    channel_candidates: *const u8,
    channel_candidates_count: size_t,
    bandwidth: c_uint,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(resource as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoResource> = from_glib_borrow(resource);

    match imp.allocate(
        wrap.unsafe_cast_ref(),
        std::slice::from_raw_parts(channel_candidates, channel_candidates_count),
        bandwidth.into(),
    ) {
        Ok(_) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
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
    use glib::{subclass::prelude::*, Error, Properties, Source};

    const GENERATION: u32 = 1111;

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Properties)]
        #[properties(wrapper_type = super::FwIsoResourceTest)]
        pub struct FwIsoResourceTest {
            #[property(override_interface = FwIsoResource, get)]
            generation: RefCell<u32>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for FwIsoResourceTest {
            const NAME: &'static str = "FwIsoResourceTest";
            type Type = super::FwIsoResourceTest;
            type Interfaces = (FwIsoResource,);

            fn new() -> Self {
                Self {
                    generation: RefCell::new(GENERATION),
                }
            }
        }

        #[glib::derived_properties]
        impl ObjectImpl for FwIsoResourceTest {}

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

            fn allocate(
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

    #[test]
    fn fw_iso_ctx_iface() {
        let resource: FwIsoResourceTest = glib::object::Object::new();

        assert_eq!(resource.generation(), GENERATION);

        assert_eq!(resource.open("blank", 0), Ok(()));
        assert!(resource.create_source().is_err());
        resource.emit_allocated(10, 20, None);
        resource.emit_deallocated(30, 40, None);
    }
}
