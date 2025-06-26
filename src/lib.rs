#![no_std]

mod reg;
pub use crate::reg::*;

#[cfg(feature = "spi/non_blocking")]
mod non_blocking;

#[cfg(feature = "non_blocking")]
pub use non_blocking::*;

#[cfg(not(feature = "non_blocking"))]
mod blocking;

#[cfg(not(feature = "non_blocking"))]
pub use blocking::*;

use core::fmt::Debug;

#[cfg(feature = "out_f32")]
pub use accelerometer::{vector::F32x3, Accelerometer};

pub mod spi {
    #[derive(Debug)]
    pub enum Error<SpiError, PinError> {
        /// SPI communication error
        Spi(SpiError),
        /// CS output pin error
        Pin(PinError),
        InvalidWhoAmI(u8),
    }

    impl<SpiError, PinError> From<SpiError> for Error<SpiError, PinError> {
        fn from(err: SpiError) -> Self {
            Self::Spi(err)
        }
    }

    pub struct Lis2dw12<SPI, CS> {
        spi: SPI,
        cs: CS,
        #[cfg(feature = "out_f32")]
        scale: FullScaleSelection,
        #[cfg(feature = "out_f32")]
        #[cfg(not(feature = "non_blocking"))]
        operating_mode: OperatingMode,
    }

    impl<SPI, CS> Lis2dw12<SPI, CS> {
        pub fn new(spi: SPI, cs: CS) -> Self {
            Self {
                spi,
                cs,
                #[cfg(feature = "out_f32")]
                scale: FullScaleSelection::PlusMinus2G,
                #[cfg(feature = "out_f32")]
                #[cfg(not(feature = "non_blocking"))]
                operating_mode: OperatingMode::LowPower,
            }
        }

        // destroy the instance and return the spi bus and its cs pin
        pub fn destroy(self) -> (SPI, CS) {
            (self.spi, self.cs)
        }
    }
}

pub mod i2c {
    use embedded_hal::i2c;

    #[derive(Debug)]
    pub enum Error<I2CError, PinError> {
        /// I2C communication error
        I2C(I2CError),
        /// CS output pin error
        Pin(PinError),
        InvalidWhoAmI(u8),
    }

    impl<I2CError, PinError> From<I2CError> for Error<I2CError, PinError> {
        fn from(err: I2CError) -> Self {
            Self::I2C(err)
        }
    }

    pub struct Lis2dw12<I2C, CS> {
        i2c: I2C,
        cs: CS,
        #[cfg(feature = "out_f32")]
        scale: FullScaleSelection,
        #[cfg(feature = "out_f32")]
        #[cfg(not(feature = "non_blocking"))]
        operating_mode: OperatingMode,
    }

    impl<I2C, CS> Lis2dw12<I2C, CS> {
        pub fn new(i2c: I2C, cs: CS) -> Self {
            Self {
                i2c,
                cs,
                #[cfg(feature = "out_f32")]
                scale: FullScaleSelection::PlusMinus2G,
                #[cfg(feature = "out_f32")]
                #[cfg(not(feature = "non_blocking"))]
                operating_mode: OperatingMode::LowPower,
            }
        }

        // destroy the instance and return the i2c bus and its cs pin
        pub fn destroy(self) -> (I2C, CS) {
            (self.i2c, self.cs)
        }
    }
}
