// SPDX-License-Identifier: MIT

use super::*;

pub trait FwIsoRxSingleImpl: FwIsoCtxImpl {
    fn interrupted(&self, ctx: &Self::Type, sec: u32, cycle: u32, header: &[u8], count: u32) {
        self.parent_interrupted(ctx, sec, cycle, header, count)
    }
}

pub trait FwIsoRxSingleImplExt: ObjectSubclass {
    fn parent_interrupted(&self, ctx: &Self::Type, sec: u32, cycle: u32, header: &[u8], count: u32);
}

impl<T: FwIsoRxSingleImpl> FwIsoRxSingleImplExt for T {
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinokoFwIsoRxSingleClass;
            let f = (*parent_class)
                .interrupted
                .expect("No parent \"interrupted\" implementation");

            f(
                ctx.unsafe_cast_ref::<FwIsoRxSingle>().to_glib_none().0,
                sec.into(),
                cycle.into(),
                header.as_ptr(),
                header.len() as u32,
                count.into(),
            )
        }
    }
}

unsafe impl<T: FwIsoRxSingleImpl> IsSubclassable<T> for FwIsoRxSingle {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.interrupted = Some(fw_iso_rx_single_interrupted::<T>);
    }
}

unsafe extern "C" fn fw_iso_rx_single_interrupted<T: FwIsoRxSingleImpl>(
    ctx: *mut ffi::HinokoFwIsoRxSingle,
    sec: c_uint,
    cycle: c_uint,
    header: *const u8,
    header_length: c_uint,
    count: c_uint,
) {
    let instance = &*(ctx as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwIsoRxSingle> = from_glib_borrow(ctx);

    imp.interrupted(
        wrap.unsafe_cast_ref(),
        sec.into(),
        cycle.into(),
        std::slice::from_raw_parts(header, header_length as usize),
        count.into(),
    )
}