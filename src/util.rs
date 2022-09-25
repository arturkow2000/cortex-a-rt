#[cfg(feature = "defmt")]
pub use defmt::panic;

#[cfg(not(feature = "defmt"))]
pub use core::panic;
