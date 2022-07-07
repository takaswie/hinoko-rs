// SPDX-License-Identifier: MIT

// For subclass of [`FwIsoCtx`][crate::FwIsoCtx] .
mod fw_iso_ctx;

// For subclass of [`FwIsoResource`][crate::FwIsoResource] .
mod fw_iso_resource;

// For subclass of [`FwIsoRxMultiple`][crate::FwIsoRxMultiple] .
mod fw_iso_rx_multiple;

// For subclass of [`FwIsoRxSingle`][crate::FwIsoRxSingle] .
mod fw_iso_rx_single;

// For subclass of [`FwIsoTx`][crate::FwIsoTx] .
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
