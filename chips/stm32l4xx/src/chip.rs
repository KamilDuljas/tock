use kernel::platform::chip::InterruptService;

use crate::chip_specific::chip_specs::ChipSpecs as ChipSpecsTrait;

pub struct Stm32l4xx<'a, I: InterruptService + 'a> {
    mpu: cortexm4f::mpu::MPU,
    userspace_kernel_boundary: cortexm4f::syscall::SysCall,
    interrupt_service: &'a I,
}
