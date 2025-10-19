use crate::rcc::Rcc;
/// Main struct for configuring on-board clocks.
pub struct Clocks<'a> {
    pub rcc: &'a Rcc,
}
