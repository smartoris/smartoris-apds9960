use crate::{Apds9960I2CPort, DEFAULT_ADDR};
use core::{marker::PhantomData, mem::take};

/// Internal buffer size.
const BUF_SIZE: usize = 32 * 4;

const GFIFO: u8 = 0xFC;

/// APDS-9960 driver.
pub struct Apds9960Drv<A> {
    pub(crate) addr: u8,
    pub(crate) buf: Box<[u8]>,
    adapters: PhantomData<A>,
}

impl<A> Apds9960Drv<A> {
    /// Sets up a new [`Apds9960Drv`].
    ///
    /// This method allocates an internal buffer of [`BUF_SIZE`] bytes.
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

    /// Performs a page read of `level` number of gesture datasets from FIFO.
    ///
    /// # Errors
    ///
    /// If `i2c` implementation returns `Err`, it's propagated to the caller.
    pub async fn drain_fifo<P: Apds9960I2CPort<A>>(
        &mut self,
        i2c: &mut P,
        level: u8,
    ) -> Result<&[u8], P::Error> {
        let mut buf = take(&mut self.buf);
        buf[0] = GFIFO;
        let size = (level << 2) as usize;
        match i2c.read(self.addr, buf, size).await {
            Ok(buf) => {
                self.buf = buf;
                Ok(&self.buf[0..size])
            }
            Err((buf, err)) => {
                self.buf = buf;
                Err(err)
            }
        }
    }

    pub(crate) async fn store_reg<P: Apds9960I2CPort<A>>(
        &mut self,
        i2c: &mut P,
        value: u16,
        reg: u8,
        size: usize,
    ) -> Result<(), P::Error> {
        let mut buf = take(&mut self.buf);
        buf[0] = reg;
        buf[1..=size].copy_from_slice(&value.to_le_bytes()[..size]);
        match i2c.write(self.addr, buf, size + 1).await {
            Ok(buf) => {
                self.buf = buf;
                Ok(())
            }
            Err((buf, err)) => {
                self.buf = buf;
                Err(err)
            }
        }
    }

    pub(crate) async fn load_reg<P: Apds9960I2CPort<A>>(
        &mut self,
        i2c: &mut P,
        reg: u8,
        size: usize,
    ) -> Result<u16, P::Error> {
        let mut buf = take(&mut self.buf);
        buf[0] = reg;
        match i2c.read(self.addr, buf, size).await {
            Ok(buf) => {
                self.buf = buf;
                let mut value = 0_u16.to_le_bytes();
                value[..size].copy_from_slice(&self.buf[..size]);
                Ok(u16::from_le_bytes(value))
            }
            Err((buf, err)) => {
                self.buf = buf;
                Err(err)
            }
        }
    }

    pub(crate) async fn touch_reg<P: Apds9960I2CPort<A>>(
        &mut self,
        i2c: &mut P,
        reg: u8,
    ) -> Result<(), P::Error> {
        let mut buf = take(&mut self.buf);
        buf[0] = reg;
        match i2c.write(self.addr, buf, 1).await {
            Ok(buf) => {
                self.buf = buf;
                Ok(())
            }
            Err((buf, err)) => {
                self.buf = buf;
                Err(err)
            }
        }
    }
}
