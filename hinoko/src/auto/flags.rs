// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_sys;
use hinoko_sys;

bitflags! {
    pub struct FwIsoCtxMatchFlag: u32 {
        const TAG0 = 1;
        const TAG1 = 2;
        const TAG2 = 4;
        const TAG3 = 8;
    }
}

#[doc(hidden)]
impl ToGlib for FwIsoCtxMatchFlag {
    type GlibType = hinoko_sys::HinokoFwIsoCtxMatchFlag;

    fn to_glib(&self) -> hinoko_sys::HinokoFwIsoCtxMatchFlag {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<hinoko_sys::HinokoFwIsoCtxMatchFlag> for FwIsoCtxMatchFlag {
    fn from_glib(value: hinoko_sys::HinokoFwIsoCtxMatchFlag) -> FwIsoCtxMatchFlag {
        FwIsoCtxMatchFlag::from_bits_truncate(value)
    }
}

impl StaticType for FwIsoCtxMatchFlag {
    fn static_type() -> Type {
        unsafe { from_glib(hinoko_sys::hinoko_fw_iso_ctx_match_flag_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FwIsoCtxMatchFlag {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FwIsoCtxMatchFlag {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for FwIsoCtxMatchFlag {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

