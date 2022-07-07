// SPDX-License-Identifier: MIT

mod fw_iso_ctx;
mod fw_iso_resource;
mod fw_iso_rx_multiple;
mod fw_iso_rx_single;
mod fw_iso_tx;

/// For convenience to provide traits and their blanket implementations to write subclass.
pub mod prelude {
    pub use super::{
        fw_iso_ctx::*, fw_iso_resource::*, fw_iso_rx_multiple::*, fw_iso_rx_single::*, fw_iso_tx::*,
    };
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Class, Error, Interface, Source},
    libc::*,
    prelude::*,
};
