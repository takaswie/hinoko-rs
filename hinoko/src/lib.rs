// SPDX-License-Identifier: MIT

#![doc = include_str!("../README.md")]

mod auto;
mod cycle_timer;
mod fw_iso_ctx;
mod fw_iso_rx_multiple;
mod fw_iso_rx_single;
mod fw_iso_tx;

// For convenience to provide structures and functions.
pub use crate::{auto::*, cycle_timer::*};

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{
        auto::traits::*, fw_iso_ctx::*, fw_iso_rx_multiple::*, fw_iso_rx_single::*, fw_iso_tx::*,
    };
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// To access to hinoko-sys crate for FFI.
pub use ffi;

// For links in documentation.
use glib;

use glib::{signal::*, translate::*, Cast, Error, IsA, SignalHandlerId, StaticType, Value};

use libc::c_uint;
