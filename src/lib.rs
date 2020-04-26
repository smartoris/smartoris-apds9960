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
#![cfg_attr(not(feature = "std"), no_std)]

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;
