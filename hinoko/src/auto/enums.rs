// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

/// A set of error code for operations in [`FwIsoCtx`][crate::FwIsoCtx].
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinokoFwIsoCtxError")]
pub enum FwIsoCtxError {
    /// The system call fails.
    #[doc(alias = "HINOKO_FW_ISO_CTX_ERROR_FAILED")]
    Failed,
    /// The instance is already associated to any firewire character device.
    #[doc(alias = "HINOKO_FW_ISO_CTX_ERROR_ALLOCATED")]
    Allocated,
    /// The instance is not associated to any firewire character device.
    #[doc(alias = "HINOKO_FW_ISO_CTX_ERROR_NOT_ALLOCATED")]
    NotAllocated,
    /// The intermediate buffer is already mapped to the process.
    #[doc(alias = "HINOKO_FW_ISO_CTX_ERROR_MAPPED")]
    Mapped,
    /// The intermediate buffer is not mapped to the process.
    #[doc(alias = "HINOKO_FW_ISO_CTX_ERROR_NOT_MAPPED")]
    NotMapped,
    /// No chunk registered before starting.
    #[doc(alias = "HINOKO_FW_ISO_CTX_ERROR_CHUNK_UNREGISTERED")]
    ChunkUnregistered,
    /// No isochronous channel is available.
    #[doc(alias = "HINOKO_FW_ISO_CTX_ERROR_NO_ISOC_CHANNEL")]
    NoIsocChannel,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for FwIsoCtxError {
    type GlibType = ffi::HinokoFwIsoCtxError;

    #[inline]
    fn into_glib(self) -> ffi::HinokoFwIsoCtxError {
        match self {
            Self::Failed => ffi::HINOKO_FW_ISO_CTX_ERROR_FAILED,
            Self::Allocated => ffi::HINOKO_FW_ISO_CTX_ERROR_ALLOCATED,
            Self::NotAllocated => ffi::HINOKO_FW_ISO_CTX_ERROR_NOT_ALLOCATED,
            Self::Mapped => ffi::HINOKO_FW_ISO_CTX_ERROR_MAPPED,
            Self::NotMapped => ffi::HINOKO_FW_ISO_CTX_ERROR_NOT_MAPPED,
            Self::ChunkUnregistered => ffi::HINOKO_FW_ISO_CTX_ERROR_CHUNK_UNREGISTERED,
            Self::NoIsocChannel => ffi::HINOKO_FW_ISO_CTX_ERROR_NO_ISOC_CHANNEL,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinokoFwIsoCtxError> for FwIsoCtxError {
    #[inline]
    unsafe fn from_glib(value: ffi::HinokoFwIsoCtxError) -> Self {
        match value {
            ffi::HINOKO_FW_ISO_CTX_ERROR_FAILED => Self::Failed,
            ffi::HINOKO_FW_ISO_CTX_ERROR_ALLOCATED => Self::Allocated,
            ffi::HINOKO_FW_ISO_CTX_ERROR_NOT_ALLOCATED => Self::NotAllocated,
            ffi::HINOKO_FW_ISO_CTX_ERROR_MAPPED => Self::Mapped,
            ffi::HINOKO_FW_ISO_CTX_ERROR_NOT_MAPPED => Self::NotMapped,
            ffi::HINOKO_FW_ISO_CTX_ERROR_CHUNK_UNREGISTERED => Self::ChunkUnregistered,
            ffi::HINOKO_FW_ISO_CTX_ERROR_NO_ISOC_CHANNEL => Self::NoIsocChannel,
            value => Self::__Unknown(value),
        }
    }
}

impl glib::error::ErrorDomain for FwIsoCtxError {
    #[inline]
    fn domain() -> glib::Quark {
        unsafe { from_glib(ffi::hinoko_fw_iso_ctx_error_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    #[allow(clippy::match_single_binding)]
    fn from(code: i32) -> Option<Self> {
        match unsafe { from_glib(code) } {
            Self::__Unknown(_) => Some(Self::Failed),
            value => Some(value),
        }
    }
}

impl StaticType for FwIsoCtxError {
    #[inline]
    #[doc(alias = "hinoko_fw_iso_ctx_error_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::hinoko_fw_iso_ctx_error_get_type()) }
    }
}

impl glib::HasParamSpec for FwIsoCtxError {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for FwIsoCtxError {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for FwIsoCtxError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwIsoCtxError {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FwIsoCtxError> for glib::Value {
    #[inline]
    fn from(v: FwIsoCtxError) -> Self {
        ToValue::to_value(&v)
    }
}

/// A set of mode for isochronous context of Linux FireWire subsystem.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinokoFwIsoCtxMode")]
pub enum FwIsoCtxMode {
    // The mode of IT context of 1394 OHCI.
    #[doc(alias = "HINOKO_FW_ISO_CTX_MODE_IT")]
    It,
    /// The mode of IR context of 1394 OHCI with packer-per-buffer protocol
    #[doc(alias = "HINOKO_FW_ISO_CTX_MODE_IR_SINGLE")]
    IrSingle,
    /// The mode of IR context of 1394 OHCI with buffer-fill protocol.
    #[doc(alias = "HINOKO_FW_ISO_CTX_MODE_IR_MULTIPLE")]
    IrMultiple,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for FwIsoCtxMode {
    type GlibType = ffi::HinokoFwIsoCtxMode;

    #[inline]
    fn into_glib(self) -> ffi::HinokoFwIsoCtxMode {
        match self {
            Self::It => ffi::HINOKO_FW_ISO_CTX_MODE_IT,
            Self::IrSingle => ffi::HINOKO_FW_ISO_CTX_MODE_IR_SINGLE,
            Self::IrMultiple => ffi::HINOKO_FW_ISO_CTX_MODE_IR_MULTIPLE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinokoFwIsoCtxMode> for FwIsoCtxMode {
    #[inline]
    unsafe fn from_glib(value: ffi::HinokoFwIsoCtxMode) -> Self {
        match value {
            ffi::HINOKO_FW_ISO_CTX_MODE_IT => Self::It,
            ffi::HINOKO_FW_ISO_CTX_MODE_IR_SINGLE => Self::IrSingle,
            ffi::HINOKO_FW_ISO_CTX_MODE_IR_MULTIPLE => Self::IrMultiple,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for FwIsoCtxMode {
    #[inline]
    #[doc(alias = "hinoko_fw_iso_ctx_mode_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::hinoko_fw_iso_ctx_mode_get_type()) }
    }
}

impl glib::HasParamSpec for FwIsoCtxMode {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for FwIsoCtxMode {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for FwIsoCtxMode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwIsoCtxMode {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FwIsoCtxMode> for glib::Value {
    #[inline]
    fn from(v: FwIsoCtxMode) -> Self {
        ToValue::to_value(&v)
    }
}

/// A set of error code for operations in [`FwIsoResourceAuto`][crate::FwIsoResourceAuto].
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinokoFwIsoResourceAutoError")]
pub enum FwIsoResourceAutoError {
    /// The system call fails.
    #[doc(alias = "HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_FAILED")]
    Failed,
    /// The instance is already associated to allocated isochronous resources.
    #[doc(alias = "HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_ALLOCATED")]
    Allocated,
    /// The instance is not associated to allocated isochronous resources.
    #[doc(alias = "HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_NOT_ALLOCATED")]
    NotAllocated,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for FwIsoResourceAutoError {
    type GlibType = ffi::HinokoFwIsoResourceAutoError;

    #[inline]
    fn into_glib(self) -> ffi::HinokoFwIsoResourceAutoError {
        match self {
            Self::Failed => ffi::HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_FAILED,
            Self::Allocated => ffi::HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_ALLOCATED,
            Self::NotAllocated => ffi::HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_NOT_ALLOCATED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinokoFwIsoResourceAutoError> for FwIsoResourceAutoError {
    #[inline]
    unsafe fn from_glib(value: ffi::HinokoFwIsoResourceAutoError) -> Self {
        match value {
            ffi::HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_FAILED => Self::Failed,
            ffi::HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_ALLOCATED => Self::Allocated,
            ffi::HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_NOT_ALLOCATED => Self::NotAllocated,
            value => Self::__Unknown(value),
        }
    }
}

impl glib::error::ErrorDomain for FwIsoResourceAutoError {
    #[inline]
    fn domain() -> glib::Quark {
        unsafe { from_glib(ffi::hinoko_fw_iso_resource_auto_error_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    #[allow(clippy::match_single_binding)]
    fn from(code: i32) -> Option<Self> {
        match unsafe { from_glib(code) } {
            Self::__Unknown(_) => Some(Self::Failed),
            value => Some(value),
        }
    }
}

impl StaticType for FwIsoResourceAutoError {
    #[inline]
    #[doc(alias = "hinoko_fw_iso_resource_auto_error_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::hinoko_fw_iso_resource_auto_error_get_type()) }
    }
}

impl glib::HasParamSpec for FwIsoResourceAutoError {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for FwIsoResourceAutoError {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for FwIsoResourceAutoError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwIsoResourceAutoError {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FwIsoResourceAutoError> for glib::Value {
    #[inline]
    fn from(v: FwIsoResourceAutoError) -> Self {
        ToValue::to_value(&v)
    }
}

/// A set of error code for operations in [`FwIsoResource`][crate::FwIsoResource].
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinokoFwIsoResourceError")]
pub enum FwIsoResourceError {
    /// The system call fails.
    #[doc(alias = "HINOKO_FW_ISO_RESOURCE_ERROR_FAILED")]
    Failed,
    /// The instance is already associated to any firewire character device.
    #[doc(alias = "HINOKO_FW_ISO_RESOURCE_ERROR_OPENED")]
    Opened,
    /// The instance is not associated to any firewire character device.
    #[doc(alias = "HINOKO_FW_ISO_RESOURCE_ERROR_NOT_OPENED")]
    NotOpened,
    /// No event to the request arrives within timeout.
    #[doc(alias = "HINOKO_FW_ISO_RESOURCE_ERROR_TIMEOUT")]
    Timeout,
    /// Event for the request arrives but includes error code.
    #[doc(alias = "HINOKO_FW_ISO_RESOURCE_ERROR_EVENT")]
    Event,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for FwIsoResourceError {
    type GlibType = ffi::HinokoFwIsoResourceError;

    #[inline]
    fn into_glib(self) -> ffi::HinokoFwIsoResourceError {
        match self {
            Self::Failed => ffi::HINOKO_FW_ISO_RESOURCE_ERROR_FAILED,
            Self::Opened => ffi::HINOKO_FW_ISO_RESOURCE_ERROR_OPENED,
            Self::NotOpened => ffi::HINOKO_FW_ISO_RESOURCE_ERROR_NOT_OPENED,
            Self::Timeout => ffi::HINOKO_FW_ISO_RESOURCE_ERROR_TIMEOUT,
            Self::Event => ffi::HINOKO_FW_ISO_RESOURCE_ERROR_EVENT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinokoFwIsoResourceError> for FwIsoResourceError {
    #[inline]
    unsafe fn from_glib(value: ffi::HinokoFwIsoResourceError) -> Self {
        match value {
            ffi::HINOKO_FW_ISO_RESOURCE_ERROR_FAILED => Self::Failed,
            ffi::HINOKO_FW_ISO_RESOURCE_ERROR_OPENED => Self::Opened,
            ffi::HINOKO_FW_ISO_RESOURCE_ERROR_NOT_OPENED => Self::NotOpened,
            ffi::HINOKO_FW_ISO_RESOURCE_ERROR_TIMEOUT => Self::Timeout,
            ffi::HINOKO_FW_ISO_RESOURCE_ERROR_EVENT => Self::Event,
            value => Self::__Unknown(value),
        }
    }
}

impl glib::error::ErrorDomain for FwIsoResourceError {
    #[inline]
    fn domain() -> glib::Quark {
        unsafe { from_glib(ffi::hinoko_fw_iso_resource_error_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    #[allow(clippy::match_single_binding)]
    fn from(code: i32) -> Option<Self> {
        match unsafe { from_glib(code) } {
            Self::__Unknown(_) => Some(Self::Failed),
            value => Some(value),
        }
    }
}

impl StaticType for FwIsoResourceError {
    #[inline]
    #[doc(alias = "hinoko_fw_iso_resource_error_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::hinoko_fw_iso_resource_error_get_type()) }
    }
}

impl glib::HasParamSpec for FwIsoResourceError {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for FwIsoResourceError {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for FwIsoResourceError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwIsoResourceError {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FwIsoResourceError> for glib::Value {
    #[inline]
    fn from(v: FwIsoResourceError) -> Self {
        ToValue::to_value(&v)
    }
}

/// A set of speed for isochronous context on IEEE 1394 bus.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinokoFwScode")]
pub enum FwScode {
    /// 100 Mbps.
    #[doc(alias = "HINOKO_FW_SCODE_S100")]
    S100,
    /// 200 Mbps.
    #[doc(alias = "HINOKO_FW_SCODE_S200")]
    S200,
    /// 400 Mbps.
    #[doc(alias = "HINOKO_FW_SCODE_S400")]
    S400,
    /// 800 Mbps.
    #[doc(alias = "HINOKO_FW_SCODE_S800")]
    S800,
    /// 1600 Mbps.
    #[doc(alias = "HINOKO_FW_SCODE_S1600")]
    S1600,
    /// 3200 Mbps.
    #[doc(alias = "HINOKO_FW_SCODE_S3200")]
    S3200,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for FwScode {
    type GlibType = ffi::HinokoFwScode;

    #[inline]
    fn into_glib(self) -> ffi::HinokoFwScode {
        match self {
            Self::S100 => ffi::HINOKO_FW_SCODE_S100,
            Self::S200 => ffi::HINOKO_FW_SCODE_S200,
            Self::S400 => ffi::HINOKO_FW_SCODE_S400,
            Self::S800 => ffi::HINOKO_FW_SCODE_S800,
            Self::S1600 => ffi::HINOKO_FW_SCODE_S1600,
            Self::S3200 => ffi::HINOKO_FW_SCODE_S3200,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinokoFwScode> for FwScode {
    #[inline]
    unsafe fn from_glib(value: ffi::HinokoFwScode) -> Self {
        match value {
            ffi::HINOKO_FW_SCODE_S100 => Self::S100,
            ffi::HINOKO_FW_SCODE_S200 => Self::S200,
            ffi::HINOKO_FW_SCODE_S400 => Self::S400,
            ffi::HINOKO_FW_SCODE_S800 => Self::S800,
            ffi::HINOKO_FW_SCODE_S1600 => Self::S1600,
            ffi::HINOKO_FW_SCODE_S3200 => Self::S3200,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for FwScode {
    #[inline]
    #[doc(alias = "hinoko_fw_scode_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::hinoko_fw_scode_get_type()) }
    }
}

impl glib::HasParamSpec for FwScode {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for FwScode {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for FwScode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwScode {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FwScode> for glib::Value {
    #[inline]
    fn from(v: FwScode) -> Self {
        ToValue::to_value(&v)
    }
}
