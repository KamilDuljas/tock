// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! Board file for Nucleo-F446RE development board
//!
//! - <https://www.st.com/en/evaluation-tools/nucleo-f446re.html>

#![no_std]
#![no_main]
#![deny(missing_docs)]

use core::ptr::addr_of_mut;
use cortexm4::support::nop;

use capsules_core::virtualizers::virtual_alarm::VirtualMuxAlarm;
use components::gpio::GpioComponent;
use core::panic::PanicInfo;
use kernel::capabilities;
use kernel::component::Component;
use kernel::hil::gpio::Configure;
use kernel::hil::led::LedHigh;
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::process::ProcessArray;
use kernel::scheduler::round_robin::RoundRobinSched;
use kernel::{create_capability, debug, static_init};

struct Nucleol476RG {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// This is in a separate, inline(never) function so that its stack frame is
/// removed when this function returns. Otherwise, the stack space used for
/// these static_inits is wasted.
// #[inline(never)]
// unsafe fn start() -> (
//     &'static kernel::Kernel,
//     Nucleol476RG,
//     &'static stm32l4xx::chip::Stm32l4xx<'static, Stm32l476rgDefaultPeripherals<'static>>,
// ) {
//     stm32f446re::init();
//     const RCC_BASE: u32 = 0x4002_1000;
// }

/// Main function called after RAM initialized.
#[no_mangle]
pub unsafe fn main() {
    stm32l4xx::init();
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);
    const RCC_BASE: u32 = 0x4002_1000;
    const GPIOA_BASE: u32 = 0x4800_0000;
    const RCC_AHB2ENR: *mut u32 = (RCC_BASE + 0x4C) as *mut u32;
    const GPIOA_MODER: *mut u32 = (GPIOA_BASE + 0x00) as *mut u32;
    const GPIOA_ODR: *mut u32 = (GPIOA_BASE + 0x14) as *mut u32;
    // 1. Włącz zegar GPIOA
    *RCC_AHB2ENR |= 1 << 0;

    // 2. Ustaw PA5 jako output (MODER5 = 01)
    *GPIOA_MODER &= !(0b11 << (5 * 2));
    *GPIOA_MODER |= 0b01 << (5 * 2);


    // 3. Migaj diodą
    loop {
        *GPIOA_ODR ^= 1 << 5; // toggle PA5
        for _ in 0..300_000 {
            unsafe { core::arch::asm!("nop") }
        }
    }

    //let (board_kernel, platform, chip) = start();
    //board_kernel.kernel_loop(&platform, chip, Some(&platform.ipc), &main_loop_capability);
}
