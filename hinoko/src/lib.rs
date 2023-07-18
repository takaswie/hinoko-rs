// SPDX-License-Identifier: MIT

#![doc = include_str!("../README.md")]

mod auto;
mod fw_iso_ctx;
mod fw_iso_ir_multiple;
mod fw_iso_ir_single;
mod fw_iso_it;

// For convenience to provide structures and functions.
pub use crate::auto::*;

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{
        auto::traits::*, fw_iso_ctx::*, fw_iso_ir_multiple::*, fw_iso_ir_single::*, fw_iso_it::*,
    };
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// To access to hinoko-sys crate for FFI.
pub use ffi;

// For links in documentation.
pub(crate) use glib;

use glib::{signal::*, translate::*, Cast, Error, IsA, SignalHandlerId, StaticType, Value};

use libc::c_uint;

use hinawa::CycleTime;
