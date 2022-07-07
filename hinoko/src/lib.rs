// SPDX-License-Identifier: MIT

//! Rust libhinoko bindings
//!
//! Rust bindings and wrappers for [hinoko library](https://github.com/takaswie/libhinoko) to
//! operate Linux FireWire character device for isochronous packets and resources in IEEE 1394
//! bus.
//!
//! The hinoko library v0.7.1 is the minimum supported version for underlying library.
//!
//! The crate depends on [glib crate v0.15](https://crates.io/crates/glib) provided by
//! [gtk-rs project](https://gtk-rs.org/) for type/object system, event loop, and dispacher.
//!
//! # License
//!
//! Released under MIT license.
//!
//! # Sample programs
//!
//! Some programs are available under `examples` directory.
//!
//! `fw-iso-resource`
//! : demonstration to allocate and deallocate isochronous resources
//!
//! `fw-iso-rx-single`
//! : demonstration to receive isochronous packets at single channel
//!
//! `fw-iso-rx-multiple`
//! : demonstration to receive isochronous packets at multiple channels
//!
//! `fw-iso-tx`
//! : demonstration to transmit isochronous packets at single channel

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
