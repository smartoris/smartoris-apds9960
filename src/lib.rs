//! APDS-9960 digital proximity, ambient light, RGB and gesture sensor [Drone
//! OS] driver.
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! smartoris-apds9960 = { version = "0.1.0" }
//! ```
//!
//! Add or extend `std` feature as follows:
//!
//! ```toml
//! [features]
//! std = ["smartoris-apds9960/std"]
//! ```
//!
//! # References
//!
//! * [Datasheet](https://docs.broadcom.com/doc/AV02-4191EN)
//!
//! [Drone OS]: https://www.drone-os.com/

#![feature(prelude_import)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod reg;

mod drv;
mod ports;

pub use self::{drv::Apds9960Drv, ports::i2c::Apds9960I2CPort};

/// Default APDS-9960 I²C slave address.
pub const DEFAULT_ADDR: u8 = 0x39;

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
