use crate::{Apds9960I2CPort, DEFAULT_ADDR};
use core::{marker::PhantomData, mem::take};

/// Internal buffer size.
const BUF_SIZE: usize = 3;

/// APDS-9960 driver.
pub struct Apds9960Drv<A> {
    pub(crate) addr: u8,
    pub(crate) buf: Box<[u8]>,
    adapters: PhantomData<A>,
}

impl<A> Apds9960Drv<A> {
    /// Sets up a new [`Apds9960Drv`].
    #[must_use]
    pub fn init() -> Self {
        Self {
            addr: DEFAULT_ADDR,
            buf: vec![0; BUF_SIZE].into_boxed_slice(),
            adapters: PhantomData,
        }
    }

    /// Changes the I²C slave address.
    pub fn set_addr(&mut self, addr: u8) {
        self.addr = addr;
    }

    pub(crate) async fn store_reg(
        &mut self,
        i2c: &mut impl Apds9960I2CPort<A>,
        value: u16,
        reg: u8,
        size: usize,
    ) {
        let mut buf = take(&mut self.buf);
        buf[0] = reg;
        buf[1..=size].copy_from_slice(&value.to_le_bytes()[..size]);
        self.buf = i2c.write(self.addr, buf, size + 1).await;
    }

    pub(crate) async fn load_reg(
        &mut self,
        i2c: &mut impl Apds9960I2CPort<A>,
        reg: u8,
        size: usize,
    ) -> u16 {
        let mut buf = take(&mut self.buf);
        buf[0] = reg;
        self.buf = i2c.read(self.addr, buf, size).await;
        let mut value = 0_u16.to_le_bytes();
        value[..size].copy_from_slice(&self.buf[..size]);
        u16::from_le_bytes(value)
    }

    pub(crate) async fn touch_reg(&mut self, i2c: &mut impl Apds9960I2CPort<A>, reg: u8) {
        let mut buf = take(&mut self.buf);
        buf[0] = reg;
        self.buf = i2c.write(self.addr, buf, 1).await;
    }
}
