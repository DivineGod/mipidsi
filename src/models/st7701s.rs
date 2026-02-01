use embedded_graphics_core::pixelcolor::{Rgb565, Rgb666, Rgb888};
use embedded_hal::delay::DelayNs;

use crate::{
    dcs::{
        BitsPerPixel, EnterNormalMode, ExitSleepMode, InterfaceExt, PixelFormat, SetAddressMode,
        SetDisplayOn, SetInvertMode, SetPixelFormat,
    },
    interface::Interface,
    models::{Model, ModelInitError},
    options::ModelOptions,
};

/// Common init for all ST7701s models and color formats.
pub fn init_common<DELAY, DI>(
    di: &mut DI,
    delay: &mut DELAY,
    options: &ModelOptions,
    pixel_format: PixelFormat,
) -> Result<SetAddressMode, ModelInitError<DI::Error>>
where
    DI: Interface,
    DELAY: DelayNs,
{
    let madctl = SetAddressMode::from(options);

    di.write_raw(0xff, &[0x77, 0x01, 0x00, 0x00, 0x10])?; // Command2 BKx Selection - Enable BK0

    let (lneset, line_delta) = if options.display_size.1 % 8 == 0 {
        ((options.display_size.1 / 8) as u8 - 1, 0x00)
    } else {
        let lneset = (options.display_size.1 / 8) as u8 - 1;
        (
            0b10000000 | lneset,
            (options.display_size.1 - (lneset * 8) as u16) as u8 / 2,
        )
    };
    di.write_raw(0xc0, &[lneset, line_delta])?; // Set the Line Dimension

    di.write_raw(0xc1, &[0x0d, 0x02])?; // Porch Control

    di.write_raw(0xC2, &[0x31, 0x05])?; // Inversion Selection & Frame Rate Control

    di.write_raw(0xC7, &[0x04])?; // X-direction Control - 0x04 = source from 479 to 0

    di.write_raw(0xCD, &[0x08])?; // Color Control - 0x08 = INV_LED PWM=0 polarity normal, INV_LED_ON = 0 polarity normal, MDT=1 pixel collect to DB[17:0], EPF[2:0]=0 copy self MSB

    di.write_raw(
        0xB0, // Positive Voltage Gamma Control
        &[
            0x00, 0x11, 0x18, 0x0E, 0x11, 0x06, 0x07, 0x08, 0x07, 0x22, 0x04, 0x12, 0x0F, 0xAA,
            0x31, 0x18,
        ],
    )?;
    // IO_EXP_MULTI_WRITE_END();
    // IO_EXP_MULTI_WRITE_START();

    di.write_raw(
        0xB1, // Negative Voltage Gamma Control
        &[
            0x00, 0x11, 0x19, 0x0E, 0x12, 0x07, 0x08, 0x08, 0x08, 0x22, 0x04, 0x11, 0x11, 0xA9,
            0x32, 0x18,
        ],
    )?;

    di.write_raw(0xFF, &[0x77, 0x01, 0x00, 0x00, 0x11])?; // Command2 BKx Selection - Enable BK1

    di.write_raw(0xB0, &[0x60])?; // Vop Amplitude Setting

    di.write_raw(0xB1, &[0x32])?; // VCOM Amplitude Setting

    di.write_raw(0xB2, &[0x07])?; // VGH Voltrage Setting

    di.write_raw(0xB3, &[0x80])?; // TEST Command Setting

    di.write_raw(0xB5, &[0x49])?; // VGL Voltage Setting

    di.write_raw(0xB7, &[0x85])?; // Power Control 1

    di.write_raw(0xB8, &[0x21])?; // Power Control 2

    di.write_raw(0xC1, &[0x78])?; // Source pre_drive timing set1

    di.write_raw(0xC2, &[0x78])?; // Source EQ2 Setting
                                  // IO_EXP_MULTI_WRITE_END();

    delay.delay_ms(20);
    // IO_EXP_MULTI_WRITE_START();
    // Start GIP (no idea what it is) Section, E0 to ED I think.
    di.write_raw(0xE0, &[0x00, 0x1B, 0x02])?;

    di.write_raw(
        0xE1,
        &[
            0x08, 0xA0, 0x00, 0x00, 0x07, 0xA0, 0x00, 0x00, 0x00, 0x44, 0x44,
        ],
    )?;

    di.write_raw(
        0xE2,
        &[
            0x11, 0x11, 0x44, 0x44, 0xED, 0xA0, 0x00, 0x00, 0xEC, 0xA0, 0x00, 0x00,
        ],
    )?;
    // IO_EXP_MULTI_WRITE_END();
    // IO_EXP_MULTI_WRITE_START();

    di.write_raw(0xE3, &[0x00, 0x00, 0x11, 0x11])?;

    di.write_raw(0xE4, &[0x44, 0x44])?;

    di.write_raw(
        0xE5,
        &[
            0x0A, 0xE9, 0xD8, 0xA0, 0x0C, 0xEB, 0xD8, 0xA0, 0x0E, 0xED, 0xD8, 0xA0, 0x10, 0xEF,
            0xD8, 0xA0,
        ],
    )?;

    di.write_raw(0xE6, &[0x00, 0x00, 0x11, 0x11])?;

    di.write_raw(0xE7, &[0x44, 0x44])?;

    // IO_EXP_MULTI_WRITE_END();
    // IO_EXP_MULTI_WRITE_START();

    di.write_raw(
        0xE8,
        &[
            0x09, 0xE8, 0xD8, 0xA0, 0x0B, 0xEA, 0xD8, 0xA0, 0x0D, 0xEC, 0xD8, 0xA0, 0x0F, 0xEE,
            0xD8, 0xA0,
        ],
    )?;

    di.write_raw(0xEB, &[0x02, 0x00, 0xE4, 0xE4, 0x88, 0x00, 0x40])?;
    di.write_raw(0xEC, &[0x3C, 0x00])?;

    di.write_raw(
        0xED,
        &[
            0xAB, 0x89, 0x76, 0x54, 0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x20, 0x45, 0x67,
            0x98, 0xBA,
        ],
    )?;

    di.write_raw(0x36, &[0x10])?; // Display Data Access Control, ML = 1, BGR = 0

    // IO_EXP_MULTI_WRITE_END();
    // IO_EXP_MULTI_WRITE_START();

    di.write_raw(0xFF, &[0x77, 0x01, 0x00, 0x00, 0x13])?; // Command2 BKx Selection - Enable BK3

    di.write_raw(0xE5, &[0xE4])?;

    di.write_raw(0xFF, &[0x77, 0x01, 0x00, 0x00, 0x00])?; // Command2 BKx Selection - Disable BK0

    di.write_raw(
        0x3A, // Interface Pixel Format
        &[
            pixel_format.as_u8() & 0x07 << 4, //0x70 RGB888, 0x60 RGB666, 0x50 RGB565
        ],
    )?;

    match options.invert_colors {
        crate::options::ColorInversion::Normal => di.write_raw(0x20, &[])?, // Display Inversion Off
        crate::options::ColorInversion::Inverted => di.write_raw(0x21, &[])?, //Display Inversion On
    }

    di.write_command(ExitSleepMode)?; //Sleep Out
                                      // IO_EXP_MULTI_WRITE_END();
    delay.delay_ms(120);

    // IO_EXP_MULTI_WRITE_START();
    di.write_command(SetDisplayOn)?; //Display On
                                     // IO_EXP_MULTI_WRITE_END();
    delay.delay_ms(120);

    // IO_EXP_MULTI_WRITE_START();
    // CS(1);
    // SCK(1);
    // SDO(1);
    // IO_EXP_MULTI_WRITE_END();
    Ok(madctl)
}

/// ST7701s Display in Rgb888 color mode
pub struct ST7701sRgb888;
/// ST7701s Display in Rgb666 color mode
pub struct ST7701sRgb666;
/// ST7701s Display in Rgb565 color mode
pub struct ST7701sRgb565;

impl Model for ST7701sRgb888 {
    type ColorFormat = Rgb888;

    const FRAMEBUFFER_SIZE: (u16, u16) = (480, 960);

    fn init<DELAY, DI>(
        &mut self,
        di: &mut DI,
        delay: &mut DELAY,
        options: &ModelOptions,
    ) -> Result<SetAddressMode, ModelInitError<DI::Error>>
    where
        DELAY: DelayNs,
        DI: Interface,
    {
        let pixel_format =
            PixelFormat::with_all(BitsPerPixel::from_rgb_color::<Self::ColorFormat>());
        init_common(di, delay, options, pixel_format)
    }
}

impl Model for ST7701sRgb666 {
    type ColorFormat = Rgb666;

    const FRAMEBUFFER_SIZE: (u16, u16) = (480, 960);

    fn init<DELAY, DI>(
        &mut self,
        di: &mut DI,
        delay: &mut DELAY,
        options: &ModelOptions,
    ) -> Result<SetAddressMode, ModelInitError<DI::Error>>
    where
        DELAY: DelayNs,
        DI: Interface,
    {
        let pixel_format =
            PixelFormat::with_all(BitsPerPixel::from_rgb_color::<Self::ColorFormat>());
        init_common(di, delay, options, pixel_format)
    }
}

impl Model for ST7701sRgb565 {
    type ColorFormat = Rgb565;

    const FRAMEBUFFER_SIZE: (u16, u16) = (480, 960);

    fn init<DELAY, DI>(
        &mut self,
        di: &mut DI,
        delay: &mut DELAY,
        options: &ModelOptions,
    ) -> Result<SetAddressMode, ModelInitError<DI::Error>>
    where
        DELAY: DelayNs,
        DI: Interface,
    {
        let pixel_format =
            PixelFormat::with_all(BitsPerPixel::from_rgb_color::<Self::ColorFormat>());
        init_common(di, delay, options, pixel_format)
    }
}
