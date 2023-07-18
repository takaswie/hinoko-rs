// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type HinokoFwIsoCtxError = c_int;
pub const HINOKO_FW_ISO_CTX_ERROR_FAILED: HinokoFwIsoCtxError = 0;
pub const HINOKO_FW_ISO_CTX_ERROR_ALLOCATED: HinokoFwIsoCtxError = 1;
pub const HINOKO_FW_ISO_CTX_ERROR_NOT_ALLOCATED: HinokoFwIsoCtxError = 2;
pub const HINOKO_FW_ISO_CTX_ERROR_MAPPED: HinokoFwIsoCtxError = 3;
pub const HINOKO_FW_ISO_CTX_ERROR_NOT_MAPPED: HinokoFwIsoCtxError = 4;
pub const HINOKO_FW_ISO_CTX_ERROR_CHUNK_UNREGISTERED: HinokoFwIsoCtxError = 5;
pub const HINOKO_FW_ISO_CTX_ERROR_NO_ISOC_CHANNEL: HinokoFwIsoCtxError = 6;

pub type HinokoFwIsoCtxMode = c_int;
pub const HINOKO_FW_ISO_CTX_MODE_IT: HinokoFwIsoCtxMode = 0;
pub const HINOKO_FW_ISO_CTX_MODE_IR_SINGLE: HinokoFwIsoCtxMode = 1;
pub const HINOKO_FW_ISO_CTX_MODE_IR_MULTIPLE: HinokoFwIsoCtxMode = 2;

pub type HinokoFwIsoResourceAutoError = c_int;
pub const HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_FAILED: HinokoFwIsoResourceAutoError = 0;
pub const HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_ALLOCATED: HinokoFwIsoResourceAutoError = 1;
pub const HINOKO_FW_ISO_RESOURCE_AUTO_ERROR_NOT_ALLOCATED: HinokoFwIsoResourceAutoError = 2;

pub type HinokoFwIsoResourceError = c_int;
pub const HINOKO_FW_ISO_RESOURCE_ERROR_FAILED: HinokoFwIsoResourceError = 0;
pub const HINOKO_FW_ISO_RESOURCE_ERROR_OPENED: HinokoFwIsoResourceError = 1;
pub const HINOKO_FW_ISO_RESOURCE_ERROR_NOT_OPENED: HinokoFwIsoResourceError = 2;
pub const HINOKO_FW_ISO_RESOURCE_ERROR_TIMEOUT: HinokoFwIsoResourceError = 3;
pub const HINOKO_FW_ISO_RESOURCE_ERROR_EVENT: HinokoFwIsoResourceError = 4;

pub type HinokoFwScode = c_int;
pub const HINOKO_FW_SCODE_S100: HinokoFwScode = 0;
pub const HINOKO_FW_SCODE_S200: HinokoFwScode = 1;
pub const HINOKO_FW_SCODE_S400: HinokoFwScode = 2;
pub const HINOKO_FW_SCODE_S800: HinokoFwScode = 3;
pub const HINOKO_FW_SCODE_S1600: HinokoFwScode = 4;
pub const HINOKO_FW_SCODE_S3200: HinokoFwScode = 5;

// Flags
pub type HinokoFwIsoCtxMatchFlag = c_uint;
pub const HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG0: HinokoFwIsoCtxMatchFlag = 1;
pub const HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG1: HinokoFwIsoCtxMatchFlag = 2;
pub const HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG2: HinokoFwIsoCtxMatchFlag = 4;
pub const HINOKO_FW_ISO_CTX_MATCH_FLAG_TAG3: HinokoFwIsoCtxMatchFlag = 8;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoCtxInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub stop: Option<unsafe extern "C" fn(*mut HinokoFwIsoCtx)>,
    pub unmap_buffer: Option<unsafe extern "C" fn(*mut HinokoFwIsoCtx)>,
    pub release: Option<unsafe extern "C" fn(*mut HinokoFwIsoCtx)>,
    pub read_cycle_time: Option<
        unsafe extern "C" fn(
            *mut HinokoFwIsoCtx,
            c_int,
            *const *mut hinawa::HinawaCycleTime,
            *mut *mut glib::GError,
        ) -> gboolean,
    >,
    pub flush_completions:
        Option<unsafe extern "C" fn(*mut HinokoFwIsoCtx, *mut *mut glib::GError) -> gboolean>,
    pub create_source: Option<
        unsafe extern "C" fn(
            *mut HinokoFwIsoCtx,
            *mut *mut glib::GSource,
            *mut *mut glib::GError,
        ) -> gboolean,
    >,
    pub stopped: Option<unsafe extern "C" fn(*mut HinokoFwIsoCtx, *const glib::GError)>,
}

impl ::std::fmt::Debug for HinokoFwIsoCtxInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoCtxInterface @ {:p}", self))
            .field("parent_iface", &self.parent_iface)
            .field("stop", &self.stop)
            .field("unmap_buffer", &self.unmap_buffer)
            .field("release", &self.release)
            .field("read_cycle_time", &self.read_cycle_time)
            .field("flush_completions", &self.flush_completions)
            .field("create_source", &self.create_source)
            .field("stopped", &self.stopped)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoIrMultipleClass {
    pub parent_class: gobject::GObjectClass,
    pub interrupted: Option<unsafe extern "C" fn(*mut HinokoFwIsoIrMultiple, c_uint)>,
}

impl ::std::fmt::Debug for HinokoFwIsoIrMultipleClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoIrMultipleClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("interrupted", &self.interrupted)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoIrSingleClass {
    pub parent_class: gobject::GObjectClass,
    pub interrupted: Option<
        unsafe extern "C" fn(*mut HinokoFwIsoIrSingle, c_uint, c_uint, *const u8, c_uint, c_uint),
    >,
}

impl ::std::fmt::Debug for HinokoFwIsoIrSingleClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoIrSingleClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("interrupted", &self.interrupted)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoItClass {
    pub parent_class: gobject::GObjectClass,
    pub interrupted:
        Option<unsafe extern "C" fn(*mut HinokoFwIsoIt, c_uint, c_uint, *const u8, c_uint, c_uint)>,
}

impl ::std::fmt::Debug for HinokoFwIsoItClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoItClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("interrupted", &self.interrupted)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoResourceAutoClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for HinokoFwIsoResourceAutoClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoResourceAutoClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoResourceInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub open: Option<
        unsafe extern "C" fn(
            *mut HinokoFwIsoResource,
            *const c_char,
            c_int,
            *mut *mut glib::GError,
        ) -> gboolean,
    >,
    pub allocate_async: Option<
        unsafe extern "C" fn(
            *mut HinokoFwIsoResource,
            *const u8,
            size_t,
            c_uint,
            *mut *mut glib::GError,
        ) -> gboolean,
    >,
    pub create_source: Option<
        unsafe extern "C" fn(
            *mut HinokoFwIsoResource,
            *mut *mut glib::GSource,
            *mut *mut glib::GError,
        ) -> gboolean,
    >,
    pub allocated:
        Option<unsafe extern "C" fn(*mut HinokoFwIsoResource, c_uint, c_uint, *const glib::GError)>,
    pub deallocated:
        Option<unsafe extern "C" fn(*mut HinokoFwIsoResource, c_uint, c_uint, *const glib::GError)>,
}

impl ::std::fmt::Debug for HinokoFwIsoResourceInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoResourceInterface @ {:p}", self))
            .field("parent_iface", &self.parent_iface)
            .field("open", &self.open)
            .field("allocate_async", &self.allocate_async)
            .field("create_source", &self.create_source)
            .field("allocated", &self.allocated)
            .field("deallocated", &self.deallocated)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoResourceOnceClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for HinokoFwIsoResourceOnceClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoResourceOnceClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoIrMultiple {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for HinokoFwIsoIrMultiple {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoIrMultiple @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoIrSingle {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for HinokoFwIsoIrSingle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoIrSingle @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoIt {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for HinokoFwIsoIt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoIt @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoResourceAuto {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for HinokoFwIsoResourceAuto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoResourceAuto @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinokoFwIsoResourceOnce {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for HinokoFwIsoResourceOnce {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinokoFwIsoResourceOnce @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct HinokoFwIsoCtx {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for HinokoFwIsoCtx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "HinokoFwIsoCtx @ {:p}", self)
    }
}

#[repr(C)]
pub struct HinokoFwIsoResource {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for HinokoFwIsoResource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "HinokoFwIsoResource @ {:p}", self)
    }
}

#[link(name = "hinoko")]
extern "C" {

    //=========================================================================
    // HinokoFwIsoCtxError
    //=========================================================================
    pub fn hinoko_fw_iso_ctx_error_get_type() -> GType;
    pub fn hinoko_fw_iso_ctx_error_quark() -> glib::GQuark;

    //=========================================================================
    // HinokoFwIsoCtxMode
    //=========================================================================
    pub fn hinoko_fw_iso_ctx_mode_get_type() -> GType;

    //=========================================================================
    // HinokoFwIsoResourceAutoError
    //=========================================================================
    pub fn hinoko_fw_iso_resource_auto_error_get_type() -> GType;
    pub fn hinoko_fw_iso_resource_auto_error_quark() -> glib::GQuark;

    //=========================================================================
    // HinokoFwIsoResourceError
    //=========================================================================
    pub fn hinoko_fw_iso_resource_error_get_type() -> GType;
    pub fn hinoko_fw_iso_resource_error_quark() -> glib::GQuark;

    //=========================================================================
    // HinokoFwScode
    //=========================================================================
    pub fn hinoko_fw_scode_get_type() -> GType;

    //=========================================================================
    // HinokoFwIsoCtxMatchFlag
    //=========================================================================
    pub fn hinoko_fw_iso_ctx_match_flag_get_type() -> GType;

    //=========================================================================
    // HinokoFwIsoIrMultiple
    //=========================================================================
    pub fn hinoko_fw_iso_ir_multiple_get_type() -> GType;
    pub fn hinoko_fw_iso_ir_multiple_new() -> *mut HinokoFwIsoIrMultiple;
    pub fn hinoko_fw_iso_ir_multiple_allocate(
        self_: *mut HinokoFwIsoIrMultiple,
        path: *const c_char,
        channels: *const u8,
        channels_length: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_ir_multiple_get_payload(
        self_: *mut HinokoFwIsoIrMultiple,
        index: c_uint,
        payload: *mut *const u8,
        length: *mut c_uint,
    );
    pub fn hinoko_fw_iso_ir_multiple_map_buffer(
        self_: *mut HinokoFwIsoIrMultiple,
        bytes_per_chunk: c_uint,
        chunks_per_buffer: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_ir_multiple_start(
        self_: *mut HinokoFwIsoIrMultiple,
        cycle_match: *const [u16; 2],
        sync_code: u32,
        tags: HinokoFwIsoCtxMatchFlag,
        chunks_per_irq: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // HinokoFwIsoIrSingle
    //=========================================================================
    pub fn hinoko_fw_iso_ir_single_get_type() -> GType;
    pub fn hinoko_fw_iso_ir_single_new() -> *mut HinokoFwIsoIrSingle;
    pub fn hinoko_fw_iso_ir_single_allocate(
        self_: *mut HinokoFwIsoIrSingle,
        path: *const c_char,
        channel: c_uint,
        header_size: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_ir_single_get_payload(
        self_: *mut HinokoFwIsoIrSingle,
        index: c_uint,
        payload: *mut *const u8,
        length: *mut c_uint,
    );
    pub fn hinoko_fw_iso_ir_single_map_buffer(
        self_: *mut HinokoFwIsoIrSingle,
        maximum_bytes_per_payload: c_uint,
        payloads_per_buffer: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_ir_single_register_packet(
        self_: *mut HinokoFwIsoIrSingle,
        schedule_interrupt: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_ir_single_start(
        self_: *mut HinokoFwIsoIrSingle,
        cycle_match: *const [u16; 2],
        sync_code: u32,
        tags: HinokoFwIsoCtxMatchFlag,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // HinokoFwIsoIt
    //=========================================================================
    pub fn hinoko_fw_iso_it_get_type() -> GType;
    pub fn hinoko_fw_iso_it_new() -> *mut HinokoFwIsoIt;
    pub fn hinoko_fw_iso_it_allocate(
        self_: *mut HinokoFwIsoIt,
        path: *const c_char,
        scode: HinokoFwScode,
        channel: c_uint,
        header_size: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_it_map_buffer(
        self_: *mut HinokoFwIsoIt,
        maximum_bytes_per_payload: c_uint,
        payloads_per_buffer: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_it_register_packet(
        self_: *mut HinokoFwIsoIt,
        tags: HinokoFwIsoCtxMatchFlag,
        sync_code: c_uint,
        header: *const u8,
        header_length: c_uint,
        payload: *const u8,
        payload_length: c_uint,
        schedule_interrupt: gboolean,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_it_start(
        self_: *mut HinokoFwIsoIt,
        cycle_match: *const [u16; 2],
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // HinokoFwIsoResourceAuto
    //=========================================================================
    pub fn hinoko_fw_iso_resource_auto_get_type() -> GType;
    pub fn hinoko_fw_iso_resource_auto_new() -> *mut HinokoFwIsoResourceAuto;
    pub fn hinoko_fw_iso_resource_auto_deallocate_async(
        self_: *mut HinokoFwIsoResourceAuto,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_resource_auto_deallocate_sync(
        self_: *mut HinokoFwIsoResourceAuto,
        timeout_ms: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // HinokoFwIsoResourceOnce
    //=========================================================================
    pub fn hinoko_fw_iso_resource_once_get_type() -> GType;
    pub fn hinoko_fw_iso_resource_once_new() -> *mut HinokoFwIsoResourceOnce;
    pub fn hinoko_fw_iso_resource_once_deallocate_async(
        self_: *mut HinokoFwIsoResourceOnce,
        channel: c_uint,
        bandwidth: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_resource_once_deallocate_sync(
        self_: *mut HinokoFwIsoResourceOnce,
        channel: c_uint,
        bandwidth: c_uint,
        timeout_ms: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // HinokoFwIsoCtx
    //=========================================================================
    pub fn hinoko_fw_iso_ctx_get_type() -> GType;
    pub fn hinoko_fw_iso_ctx_create_source(
        self_: *mut HinokoFwIsoCtx,
        source: *mut *mut glib::GSource,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_ctx_flush_completions(
        self_: *mut HinokoFwIsoCtx,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_ctx_read_cycle_time(
        self_: *mut HinokoFwIsoCtx,
        clock_id: c_int,
        cycle_time: *const *mut hinawa::HinawaCycleTime,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_ctx_release(self_: *mut HinokoFwIsoCtx);
    pub fn hinoko_fw_iso_ctx_stop(self_: *mut HinokoFwIsoCtx);
    pub fn hinoko_fw_iso_ctx_unmap_buffer(self_: *mut HinokoFwIsoCtx);

    //=========================================================================
    // HinokoFwIsoResource
    //=========================================================================
    pub fn hinoko_fw_iso_resource_get_type() -> GType;
    pub fn hinoko_fw_iso_resource_calculate_bandwidth(
        bytes_per_payload: c_uint,
        scode: HinokoFwScode,
    ) -> c_uint;
    pub fn hinoko_fw_iso_resource_allocate_async(
        self_: *mut HinokoFwIsoResource,
        channel_candidates: *const u8,
        channel_candidates_count: size_t,
        bandwidth: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_resource_allocate_sync(
        self_: *mut HinokoFwIsoResource,
        channel_candidates: *const u8,
        channel_candidates_count: size_t,
        bandwidth: c_uint,
        timeout_ms: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_resource_create_source(
        self_: *mut HinokoFwIsoResource,
        source: *mut *mut glib::GSource,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinoko_fw_iso_resource_open(
        self_: *mut HinokoFwIsoResource,
        path: *const c_char,
        open_flag: c_int,
        error: *mut *mut glib::GError,
    ) -> gboolean;

}
