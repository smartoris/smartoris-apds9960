//! APDS-9960 digital proximity, ambient light, RGB and gesture sensor driver
//! for [Drone OS].
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
//! The driver can be used with any I²C implementation. Here is an example of
//! integration with [`smartoris-i2c`](https://crates.io/crates/smartoris-i2c)
//! crate.
//!
//! ```no_run
//! # #![feature(const_fn_fn_ptr_basics)]
//! # #![feature(never_type)]
//! # #![feature(unwrap_infallible)]
//! # use drone_stm32_map::periph::{
//! #     dma::ch::{Dma1Ch5, Dma1Ch6},
//! #     i2c::I2C1,
//! # };
//! # mod thr {
//! #     use drone_stm32_map::thr::*;
//! #     drone_cortexm::thr::nvic! {
//! #         thread => pub Thr {};
//! #         local => pub ThrLocal {};
//! #         vtable => pub Vtable;
//! #         index => pub Thrs;
//! #         init => pub ThrsInit;
//! #         threads => {
//! #             interrupts => {
//! #                 16: pub dma1_ch5;
//! #                 17: pub dma1_ch6;
//! #                 31: pub i2c1_ev;
//! #                 32: pub i2c1_er;
//! #             };
//! #         };
//! #     }
//! # }
//! # async fn handler() {
//! # let mut i2c1: smartoris_i2c::I2CDrv<
//! #     I2C1,
//! #     thr::I2C1Ev,
//! #     thr::I2C1Er,
//! #     Dma1Ch6,
//! #     thr::Dma1Ch6,
//! #     Dma1Ch5,
//! #     thr::Dma1Ch5,
//! # > = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
//! /// Adapters for external crates.
//! mod adapters {
//!     /// A marker type for port implementations in this module.
//!     pub struct Adapters;
//!
//!     /// APDS-9960 adapters.
//!     mod apds9960 {
//!         use super::Adapters;
//!         use async_trait::async_trait;
//!         use drone_cortexm::thr::prelude::*;
//!         use drone_stm32_map::periph::{dma::ch::DmaChMap, i2c::I2CMap};
//!         use smartoris_apds9960::Apds9960I2CPort;
//!         use smartoris_i2c::I2CDrv;
//!
//!         #[async_trait]
//!         impl<
//!             I2C: I2CMap,
//!             I2CEv: IntToken,
//!             I2CEr: IntToken,
//!             DmaTx: DmaChMap,
//!             DmaTxInt: IntToken,
//!             DmaRx: DmaChMap,
//!             DmaRxInt: IntToken,
//!         > Apds9960I2CPort<Adapters>
//!             for I2CDrv<I2C, I2CEv, I2CEr, DmaTx, DmaTxInt, DmaRx, DmaRxInt>
//!         {
//!             type Error = !;
//!
//!             async fn write(
//!                 &mut self,
//!                 addr: u8,
//!                 buf: Box<[u8]>,
//!                 count: usize,
//!             ) -> Result<Box<[u8]>, (Box<[u8]>, !)> {
//!                 Ok(self.master(buf).write(addr, ..count).await.stop())
//!             }
//!
//!             async fn read(
//!                 &mut self,
//!                 addr: u8,
//!                 buf: Box<[u8]>,
//!                 count: usize,
//!             ) -> Result<Box<[u8]>, (Box<[u8]>, !)> {
//!                 Ok(self.master(buf).write(addr, ..1).await.read(addr, ..count).await.stop())
//!             }
//!         }
//!     }
//! }
//!
//! use smartoris_apds9960::Apds9960Drv;
//!
//! let mut apds9960 = Apds9960Drv::init();
//! apds9960.store_enable(&mut i2c1, |r| r.set_pon().set_pen()).await.into_ok();
//! loop {
//!     if apds9960.load_status(&mut i2c1).await.into_ok().pvalid() {
//!         let pdata = apds9960.load_pdata(&mut i2c1).await.into_ok();
//!         println!("proximity: {}", pdata);
//!     }
//! }
//! # }
//! # fn main() {}
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
