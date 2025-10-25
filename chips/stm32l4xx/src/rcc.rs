// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};
use kernel::utilities::registers::{register_bitfields, ReadWrite};
use kernel::utilities::StaticRef;

pub struct Rcc {
    registers: StaticRef<RccRegisters>,
}

/// Reset and clock control
#[repr(C)]
struct RccRegisters {
    _reserved: [u8; 0x4C],
    /// APB1 peripheral clock enable register
    ahb2enr: ReadWrite<u32, AHB2ENR::Register>,
}

impl Rcc {
    pub fn new() -> Self {
        let rcc = Self {
            registers: RCC_BASE,
        };
        rcc
    }

    // GPIOA clock

    pub(crate) fn is_enabled_gpioa_clock(&self) -> bool {
        self.registers.ahb2enr.is_set(AHB2ENR::GPIOAEN)
    }

    pub(crate) fn enable_gpioa_clock(&self) {
        self.registers.ahb2enr.modify(AHB2ENR::GPIOAEN::SET)
    }

    pub(crate) fn disable_gpioa_clock(&self) {
        self.registers.ahb2enr.modify(AHB2ENR::GPIOAEN::CLEAR)
    }
}

register_bitfields![u32,
   AHB2ENR [
    GPIOAEN OFFSET(0) NUMBITS(1) []
   ]
];

const RCC_BASE: StaticRef<RccRegisters> =
    unsafe { StaticRef::new(0x40021000 as *const RccRegisters) };
