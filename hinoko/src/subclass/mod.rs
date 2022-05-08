// SPDX-License-Identifier: MIT

pub mod fw_iso_ctx;
pub mod fw_iso_rx_multiple;
pub mod fw_iso_rx_single;
pub mod fw_iso_tx;

pub mod fw_iso_resource;
pub mod fw_iso_resource_auto;
pub mod fw_iso_resource_once;

pub mod prelude {
    pub use {
        super::fw_iso_ctx::{FwIsoCtxImpl, FwIsoCtxImplExt},
        super::fw_iso_resource::{FwIsoResourceImpl, FwIsoResourceImplExt},
        super::fw_iso_resource_auto::FwIsoResourceAutoImpl,
        super::fw_iso_resource_once::FwIsoResourceOnceImpl,
        super::fw_iso_rx_multiple::{FwIsoRxMultipleImpl, FwIsoRxMultipleImplExt},
        super::fw_iso_rx_single::{FwIsoRxSingleImpl, FwIsoRxSingleImplExt},
        super::fw_iso_tx::{FwIsoTxImpl, FwIsoTxImplExt},
    };
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Class, Error, Interface, Source},
    libc::*,
    prelude::*,
};
