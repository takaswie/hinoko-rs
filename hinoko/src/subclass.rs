// SPDX-License-Identifier: MIT

// For subclass of [`FwIsoCit`][crate::FwIsoCit] .
mod fw_iso_ctx;

// For subclass of [`FwIsoResource`][crate::FwIsoResource] .
mod fw_iso_resource;

// For subclass of [`FwIsoIrMultiple`][crate::FwIsoIrMultiple] .
mod fw_iso_ir_multiple;

// For subclass of [`FwIsoIrSingle`][crate::FwIsoIrSingle] .
mod fw_iso_ir_single;

// For subclass of [`FwIsoIt`][crate::FwIsoIt] .
mod fw_iso_it;

/// For convenience to provide traits and their blanket implementations to write subclass.
pub mod prelude {
    pub use super::{
        fw_iso_ctx::*, fw_iso_ir_multiple::*, fw_iso_ir_single::*, fw_iso_it::*, fw_iso_resource::*,
    };
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Class, Error, Interface, Source},
    libc::*,
    prelude::*,
};
