// SPDX-License-Identifier: MIT

use super::*;

pub trait FwIsoResourceAutoImpl: FwIsoResourceImpl {}

unsafe impl<T: FwIsoResourceAutoImpl> IsSubclassable<T> for FwIsoResourceAuto {}
