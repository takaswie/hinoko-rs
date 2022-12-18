// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`FwIsoIrMultiple`][crate::FwIsoIrMultiple].
pub trait FwIsoIrMultipleImpl: FwIsoCtxImpl {
    fn interrupted(&self, ctx: &Self::Type, count: u32) {
        self.parent_interrupted(ctx, count)
    }
}

/// Trait which is automatically implemented to implementator of
/// [`FwIsoIrMultipleImpl`][self::FwIsoIrMultipleImpl].
pub trait FwIsoIrMultipleImplExt: ObjectSubclass {
    fn parent_interrupted(&self, ctx: &Self::Type, count: u32);
}

impl<T: FwIsoIrMultipleImpl> FwIsoIrMultipleImplExt for T {
    fn parent_interrupted(&self, ctx: &Self::Type, count: u32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoIrMultipleClass;
            let f = (*parent_class)
                .interrupted
                .expect("No parent \"interrupted\" implementation");

            f(
                ctx.unsafe_cast_ref::<FwIsoIrMultiple>().to_glib_none().0,
                count.into(),
            )
        }
    }
}

unsafe impl<T: FwIsoIrMultipleImpl> IsSubclassable<T> for FwIsoIrMultiple {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.interrupted = Some(fw_iso_ir_multiple_interrupted::<T>);
    }
}

unsafe extern "C" fn fw_iso_ir_multiple_interrupted<T: FwIsoIrMultipleImpl>(
    ctx: *mut ffi::HinokoFwIsoIrMultiple,
    count: c_uint,
) {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoIrMultiple> = from_glib_borrow(ctx);

    imp.interrupted(wrap.unsafe_cast_ref(), count.into())
}
