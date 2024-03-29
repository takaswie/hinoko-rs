// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`FwIsoIrSingle`][crate::FwIsoIrSingle].
pub trait FwIsoIrSingleImpl: FwIsoCtxImpl {
    /// Class closure for the [`interrupted`][struct@crate::FwIsoIrSingle#interrupted] signal.
    /// ## `sec`
    /// The sec part of isochronous cycle when interrupt occurs, up to 7.
    /// ## `cycle`
    /// The cycle part of of isochronous cycle when interrupt occurs, up to 7999.
    /// ## `header`
    /// The headers of IR context
    ///     for packets handled in the event of interrupt. The content is different
    ///     depending on header_size parameter of [`FwIsoIrSingleExt::allocate()`][crate::prelude::FwIsoIrSingleExt::allocate()].
    /// ## `count`
    /// the number of packets to handle.
    fn interrupted(&self, ctx: &Self::Type, sec: u32, cycle: u32, header: &[u8], count: u32) {
        self.parent_interrupted(ctx, sec, cycle, header, count)
    }
}

/// Trait which is automatically implemented to implementator of [`FwIsoIrSingleImpl`][self::FwIsoIrSingleImpl].
pub trait FwIsoIrSingleImplExt: ObjectSubclass {
    fn parent_interrupted(&self, ctx: &Self::Type, sec: u32, cycle: u32, header: &[u8], count: u32);
}

impl<T: FwIsoIrSingleImpl> FwIsoIrSingleImplExt for T {
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoIrSingleClass;
            let f = (*parent_class)
                .interrupted
                .expect("No parent \"interrupted\" implementation");

            f(
                ctx.unsafe_cast_ref::<FwIsoIrSingle>().to_glib_none().0,
                sec.into(),
                cycle.into(),
                header.as_ptr(),
                header.len() as u32,
                count.into(),
            )
        }
    }
}

unsafe impl<T: FwIsoIrSingleImpl> IsSubclassable<T> for FwIsoIrSingle {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.interrupted = Some(fw_iso_ir_single_interrupted::<T>);
    }
}

unsafe extern "C" fn fw_iso_ir_single_interrupted<T: FwIsoIrSingleImpl>(
    ctx: *mut ffi::HinokoFwIsoIrSingle,
    sec: c_uint,
    cycle: c_uint,
    header: *const u8,
    header_length: c_uint,
    count: c_uint,
) {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoIrSingle> = from_glib_borrow(ctx);

    imp.interrupted(
        wrap.unsafe_cast_ref(),
        sec.into(),
        cycle.into(),
        std::slice::from_raw_parts(header, header_length as usize),
        count.into(),
    )
}
