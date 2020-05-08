use async_trait::async_trait;

/// I²C port for APDS-9960.
///
/// Refer to "I²C-bus Protocol" section in APDS-9960 datasheet for details.
#[async_trait]
pub trait Apds9960I2CPort<A> {
    /// I²C error.
    type Error;

    /// I²C-bus write transaction.
    ///
    /// Implementation writes `count` bytes from `buf` to `addr` slave address,
    /// then returns `buf`.
    async fn write(
        &mut self,
        addr: u8,
        buf: Box<[u8]>,
        count: usize,
    ) -> Result<Box<[u8]>, (Box<[u8]>, Self::Error)>;

    /// I²C-bus combined transaction.
    ///
    /// Implementation writes the first byte from `buf` to `addr` slave address,
    /// reads `count` bytes into `buf` from the same slave address, then returns
    /// `buf`.
    async fn read(
        &mut self,
        addr: u8,
        buf: Box<[u8]>,
        count: usize,
    ) -> Result<Box<[u8]>, (Box<[u8]>, Self::Error)>;
}
