#![no_std]

use crate::i2c::Error;
use crate::spi::Error as OtherError;

mod reg;
pub use crate::reg::*;

use core::fmt::Debug;

#[cfg(feature = "out_f32")]
pub use accelerometer::{vector::F32x3, Accelerometer};

pub mod i2c;
pub mod spi;
