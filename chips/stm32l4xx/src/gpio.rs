use kernel::utilities::{
    registers::{register_bitfields, ReadWrite, WriteOnly},
    StaticRef,
};

/// General-purpose I/Os
#[repr(C)]
struct GpioRegisters {
    /// GPIO port mode register
    moder: ReadWrite<u32, MODER::Register>,
    /// GPIO port bit set/reset register
    bsrr: WriteOnly<u32, BSRR::Register>,
}

register_bitfields![u32,
    MODER [
        /// Port x configuration bits (y = 0..15)
        MODER5 OFFSET(10) NUMBITS(2) []
    ],
    BSRR [
        /// Port x reset bit y (y = 0..15)
        BR5 OFFSET(11) NUMBITS(1) [],
        BS5 OFFSET(5) NUMBITS(1) [],
    ]
];

const GPIOA_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x48000000 as *const GpioRegisters) };
