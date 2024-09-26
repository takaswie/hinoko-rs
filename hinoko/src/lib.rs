// SPDX-License-Identifier: MIT

#![doc = include_str!("../README.md")]

mod auto;
mod fw_iso_ctx;
mod fw_iso_ir_multiple;
mod fw_iso_ir_single;
mod fw_iso_it;

pub use crate::auto::*;

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{
        auto::traits::*, fw_iso_ctx::*, fw_iso_ir_multiple::*, fw_iso_ir_single::*, fw_iso_it::*,
    };
}

/// For subclass implementations derived from provided class.
pub mod subclass;

pub use ffi;

pub(crate) use glib;

use glib::{object::*, signal::*, translate::*, types::StaticType, Error, Value};

use libc::c_uint;

use hinawa::CycleTime;
