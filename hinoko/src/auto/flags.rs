// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    #[doc(alias = "HinokoFwIsoCtxMatchFlag")]
    pub struct FwIsoCtxMatchFlag: u32 {
        #[doc(alias = "HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG0")]
        const TAG0 = ffi::HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG0 as u32;
        #[doc(alias = "HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG1")]
        const TAG1 = ffi::HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG1 as u32;
        #[doc(alias = "HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG2")]
        const TAG2 = ffi::HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG2 as u32;
        #[doc(alias = "HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG3")]
        const TAG3 = ffi::HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG3 as u32;
    }
}

impl fmt::Display for FwIsoCtxMatchFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FwIsoCtxMatchFlag {
    type GlibType = ffi::HinokoFwIsoCtxMatchFlag;

    fn into_glib(self) -> ffi::HinokoFwIsoCtxMatchFlag {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinokoFwIsoCtxMatchFlag> for FwIsoCtxMatchFlag {
    unsafe fn from_glib(value: ffi::HinokoFwIsoCtxMatchFlag) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for FwIsoCtxMatchFlag {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinoko_fw_iso_ctx_match_flag_get_type()) }
    }
}

impl glib::value::ValueType for FwIsoCtxMatchFlag {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FwIsoCtxMatchFlag {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for FwIsoCtxMatchFlag {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
