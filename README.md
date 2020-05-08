[![crates.io](https://img.shields.io/crates/v/smartoris-apds9960.svg)](https://crates.io/crates/smartoris-apds9960)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# smartoris-apds9960

APDS-9960 digital proximity, ambient light, RGB and gesture sensor [Drone
OS] driver.

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
smartoris-apds9960 = { version = "0.1.0" }
```

Add or extend `std` feature as follows:

```toml
[features]
std = ["smartoris-apds9960/std"]
```

The driver can be used with any I²C implementation. Here is an example of
integration with [`smartoris-i2c`](https://crates.io/crates/smartoris-i2c)
crate.

Example of usage:

```rust
/// Adapters for external crates.
mod adapters {
    /// A marker type for port implementations in this module.
    pub struct Adapters;

    /// APDS-9960 adapters.
    mod apds9960 {
        use super::Adapters;
        use async_trait::async_trait;
        use drone_cortexm::thr::prelude::*;
        use drone_stm32_map::periph::{dma::ch::DmaChMap, i2c::I2CMap};
        use smartoris_apds9960::Apds9960I2CPort;
        use smartoris_i2c::I2CDrv;

        #[async_trait]
        impl<
            I2C: I2CMap,
            I2CEv: IntToken,
            I2CEr: IntToken,
            DmaTx: DmaChMap,
            DmaTxInt: IntToken,
            DmaRx: DmaChMap,
            DmaRxInt: IntToken,
        > Apds9960I2CPort<Adapters>
            for I2CDrv<I2C, I2CEv, I2CEr, DmaTx, DmaTxInt, DmaRx, DmaRxInt>
        {
            type Error = !;

            async fn write(
                &mut self,
                addr: u8,
                buf: Box<[u8]>,
                count: usize,
            ) -> Result<Box<[u8]>, (Box<[u8]>, !)> {
                Ok(self.master(buf).write(addr, ..count).await.stop())
            }

            async fn read(
                &mut self,
                addr: u8,
                buf: Box<[u8]>,
                count: usize,
            ) -> Result<Box<[u8]>, (Box<[u8]>, !)> {
                Ok(self.master(buf).write(addr, ..1).await.read(addr, ..count).await.stop())
            }
        }
    }
}

use smartoris_apds9960::Apds9960Drv;

let mut apds9960 = Apds9960Drv::init();
apds9960.store_enable(&mut i2c1, |r| r.set_pon().set_pen()).await.into_ok();
loop {
    if apds9960.load_status(&mut i2c1).await.into_ok().pvalid() {
        let pdata = apds9960.load_pdata(&mut i2c1).await.into_ok();
        println!("proximity: {}", pdata);
    }
}
```

## References

* [Datasheet](https://docs.broadcom.com/doc/AV02-4191EN)

[Drone OS]: https://www.drone-os.com/

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
