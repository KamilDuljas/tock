// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Trait that encompasses chip specifications
//!
//! The main use of this trait is to be passed as a bound for the type parameter for chip
//! peripherals in crates such as `stm32l476rg`.

use kernel::platform::chip::Chip;

use crate::chip_specific::clock_constants::{ClockConstants, PllConstants, SystemClockConstants};
//use crate::chip_specific::flash::FlashChipSpecific;

pub enum Stm32l476Specs {}

pub trait ChipSpecs: ClockConstants {}
// pub trait ChipSpecs: ClockConstants + FlashChipSpecific {}

impl<T: ClockConstants> ChipSpecs for T {}
//impl<T: ClockConstants + FlashChipSpecific> ChipSpecs for T {}
