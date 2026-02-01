use embedded_hal::spi::SpiDevice;

use crate::interface::{Interface, OutputBus};

/// Parallel + SPI interface
///
/// This interface uses parallel gpio pins to send pixel data
/// and SPI to send commands to the display
pub struct ParallelSpiInterface<BUS, SPI> {
    bus: BUS,
    spi: SPI,
}

/// Parallel Spi interface error
#[derive(Clone, Copy, Debug)]
pub enum ParallelSpiError<BUS, SPI> {
    /// Parallel Bus Error
    Bus(BUS),
    /// Spi Command Error
    Spi(SPI),
}

impl<BUS, SPI> ParallelSpiInterface<BUS, SPI>
where
    BUS: OutputBus,
    SPI: SpiDevice,
{
    /// Create new parallel + spi interface for communication with a display driver
    pub fn new(bus: BUS, spi: SPI) -> Self {
        Self { bus, spi }
    }

    fn send_word(
        &mut self,
        word: BUS::Word,
    ) -> Result<(), ParallelSpiError<BUS::Error, SPI::Error>> {
        self.bus.set_value(word).map_err(ParallelSpiError::Bus)?;
        Ok(())
    }
}

impl<BUS, SPI> Interface for ParallelSpiInterface<BUS, SPI>
where
    BUS: OutputBus,
    BUS::Word: From<u8> + Eq,
    SPI: SpiDevice,
{
    type Word = BUS::Word;

    type Error = ParallelSpiError<BUS::Error, SPI::Error>;

    const KIND: super::InterfaceKind = BUS::KIND;

    fn send_command(&mut self, command: u8, args: &[u8]) -> Result<(), Self::Error> {
        self.spi.write(&[command]).map_err(ParallelSpiError::Spi)?;
        self.spi.write(args).map_err(ParallelSpiError::Spi)?;
        Ok(())
    }

    fn send_pixels<const N: usize>(
        &mut self,
        pixels: impl IntoIterator<Item = [Self::Word; N]>,
    ) -> Result<(), Self::Error> {
        for pixel in pixels {
            for word in pixel {
                self.send_word(word)?;
            }
        }
        Ok(())
    }

    fn send_repeated_pixel<const N: usize>(
        &mut self,
        pixel: [Self::Word; N],
        count: u32,
    ) -> Result<(), Self::Error> {
        if count == 0 || N == 0 {
            return Ok(());
        }

        if let Some(word) = is_same(pixel) {
            self.send_word(word)?;
            Ok(())
        } else {
            self.send_pixels((0..count).map(|_| pixel))
        }
    }
}

fn is_same<const N: usize, T: Copy + Eq>(array: [T; N]) -> Option<T> {
    let (&first, rest) = array.split_first()?;
    for &x in rest {
        if x != first {
            return None;
        }
    }
    Some(first)
}
