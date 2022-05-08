// SPDX-License-Identifier: MIT

use super::*;

pub trait FwIsoResourceOnceImpl: FwIsoResourceImpl {}

unsafe impl<T: FwIsoResourceOnceImpl> IsSubclassable<T> for FwIsoResourceOnce {}
