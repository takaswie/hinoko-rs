// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`FwIsoIt`][crate::FwIsoIt].
pub trait FwIsoItImpl: FwIsoCtxImpl {
    /// Class closure for the [`interrupted`][struct@crate::FwIsoIt#interrupted] signal.
    /// ## `sec`
    /// The sec part of isochronous cycle when interrupt occurs, up to 7.
    /// ## `cycle`
    /// The cycle part of of isochronous cycle when interrupt occurs, up to 7999.
    /// ## `tstamp`
    /// A series of timestamps for
    ///     packets already handled.
    /// ## `count`
    /// the number of handled packets.
    fn interrupted(&self, ctx: &Self::Type, sec: u32, cycle: u32, header: &[u8], count: u32) {
        self.parent_interrupted(ctx, sec, cycle, header, count)
    }
}

/// Trait which is automatically implemented to implementator of [`FwIsoItImpl`][self::FwIsoItImpl].
pub trait FwIsoItImplExt: ObjectSubclass {
    fn parent_interrupted(&self, ctx: &Self::Type, sec: u32, cycle: u32, header: &[u8], count: u32);
}

impl<T: FwIsoItImpl> FwIsoItImplExt for T {
    fn parent_interrupted(
        &self,
        ctx: &Self::Type,
        sec: u32,
        cycle: u32,
        header: &[u8],
        count: u32,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoItClass;
            let f = (*parent_class)
                .interrupted
                .expect("No parent \"interrupted\" implementation");

            f(
                ctx.unsafe_cast_ref::<FwIsoIt>().to_glib_none().0,
                sec.into(),
                cycle.into(),
                header.as_ptr(),
                header.len() as u32,
                count.into(),
            )
        }
    }
}

unsafe impl<T: FwIsoItImpl> IsSubclassable<T> for FwIsoIt {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.interrupted = Some(fw_iso_it_interrupted::<T>);
    }
}

unsafe extern "C" fn fw_iso_it_interrupted<T: FwIsoItImpl>(
    ctx: *mut ffi::HinokoFwIsoIt,
    sec: c_uint,
    cycle: c_uint,
    header: *const u8,
    header_length: c_uint,
    count: c_uint,
) {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoIt> = from_glib_borrow(ctx);

    imp.interrupted(
        wrap.unsafe_cast_ref(),
        sec.into(),
        cycle.into(),
        std::slice::from_raw_parts(header, header_length as usize),
        count.into(),
    )
}
