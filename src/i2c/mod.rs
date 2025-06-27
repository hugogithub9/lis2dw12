use lis2dw12::FullScaleSelection;

#[cfg(feature = "non_blocking")]
mod non_blocking;

#[cfg(feature = "non_blocking")]
pub use non_blocking::*;

#[cfg(not(feature = "non_blocking"))]
mod blocking;

#[cfg(not(feature = "non_blocking"))]
pub use blocking::*;

#[derive(Debug)]
pub enum Error<I2cError, PinError> {
    /// I2C communication error
    I2c(I2cError),
    /// CS output pin error
    Pin(PinError),
    InvalidWhoAmI(u8),
}

impl<I2cError, PinError> From<I2cError> for Error<I2cError, PinError> {
    fn from(err: I2cError) -> Self {
        Self::I2c(err)
    }
}

pub struct Lis2dw12<I2c, CS> {
    i2c: I2c,
    cs: CS,
    #[cfg(feature = "out_f32")]
    scale: FullScaleSelection,
    #[cfg(feature = "out_f32")]
    #[cfg(not(feature = "non_blocking"))]
    operating_mode: OperatingMode,
}

impl<I2c, CS> Lis2dw12<I2c, CS> {
    pub fn new(i2c: I2c, cs: CS) -> Self {
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
    pub fn destroy(self) -> (I2c, CS) {
        (self.i2c, self.cs)
    }
}
