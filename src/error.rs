use core::fmt::{self, Debug, Display};
use embedded_hal::spi::SpiDevice;


mod private {
    #[derive(Debug)]
    pub enum Private {}
}

/// The error type used by this library.
///
/// This can encapsulate an SPI or GPIO error, and adds its own protocol errors
/// on top of that.
pub enum Error<SPI: SpiDevice> {
    /// An SPI transfer failed.
    Spi(SPI::Error),

    /// Status register contained unexpected flags.
    ///
    /// This can happen when the chip is faulty, incorrectly connected, or the
    /// driver wasn't constructed or destructed properly (eg. while there is
    /// still a write in progress).
    UnexpectedStatus,

    #[doc(hidden)]
    __NonExhaustive(private::Private),
}

impl<SPI: SpiDevice> Debug for Error<SPI>
where
    SPI::Error: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Spi(spi) => write!(f, "Error::Spi({:?})", spi),
            Error::UnexpectedStatus => f.write_str("Error::UnexpectedStatus"),
            Error::__NonExhaustive(_) => unreachable!(),
        }
    }
}

impl<SPI: SpiDevice> Display for Error<SPI>
where
    SPI::Error: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Spi(spi) => write!(f, "SPI error: {}", spi),
            Error::UnexpectedStatus => f.write_str("unexpected value in status register"),
            Error::__NonExhaustive(_) => unreachable!(),
        }
    }
}
