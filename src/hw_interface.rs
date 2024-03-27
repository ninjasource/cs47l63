#![allow(async_fn_in_trait)]

// This is the hardware interface to be implemented by whoever uses this driver
// Both i2c and spi protocols are supported
// Big-endian format is used for multi-byte communication to and from the device
// E is the error type of the communication bus used

pub trait Bus<E> {
    /// Read the value from a register
    ///
    /// The process is as follows:
    /// Apply the read bit to the register address (reg | 0x80000000)
    /// Write 4 the byte register address calculated above
    /// Write 4 byte zero padding
    /// Write another 4 byte zero padding and read back 4 bytes at the same time
    /// For example, SPI would be cs low, write 4 bytes, write 4 bytes, read 4 bytes, cs high
    async fn read(&mut self, reg: u32) -> Result<u32, E>;

    /// Write a value to a register
    ///
    /// The process is as follows:
    /// Write 4 the byte register address
    /// Write 4 byte zero padding
    /// Write 4 byte value
    /// For example, SPI would be cs low, write 4 bytes, write 4 bytes, write 4 bytes, cs high
    async fn write(&mut self, reg: u32, val: u32) -> Result<(), E>;

    /// Write bytes to a register
    ///
    /// The process is as follows:
    /// Write 4 the byte register address
    /// Write 4 byte zero padding
    /// Write n bytes
    /// For example, SPI would be cs low, write 4 bytes, write 4 bytes, write n bytes, cs high
    async fn write_block(&mut self, reg: u32, val: &[u8]) -> Result<(), E>;

    /// Async wait for a set number of milliseconds
    async fn delay_ms(&self, millis: u64);
}
