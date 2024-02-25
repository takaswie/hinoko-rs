// SPDX-License-Identifier: MIT

mod fw_iso_ctx;

mod fw_iso_resource;

mod fw_iso_ir_multiple;

mod fw_iso_ir_single;

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
