//void ST7735_TFT ::TFTPowerDown(void)
//{
//	TFTchangeMode(TFT_Display_off_mode);
//	TFT_DC_SetLow;
//	TFT_RST_SetLow;
//
//if (_hardwareSPI == false)
//{
//	TFT_SCLK_SetLow;
//	TFT_SDATA_SetLow;
//	TFT_CS_SetLow;
//}else{
//	bcm2835_spi_end();
//	}
//}
//
//
//
//void ST7735_TFT ::TFTchangeMode(TFT_modes_e mode) {
//	switch (mode) {
//		case TFT_Partial_mode:
//		case TFT_Idle_mode:
//		case TFT_Sleep_mode:
//		case TFT_Invert_mode:
//		case TFT_Display_on_mode:
//		case TFT_Display_off_mode:
//	}//switch
//}
//
//
//void ST7735_TFT_graphics ::TFTdrawPixel(uint8_t x, uint8_t y, uint16_t color) {
//	if ((x >= _widthTFT) || (y >= _heightTFT))
//		return;
//	TFTsetAddrWindow(x, y, x + 1, y + 1);
//	writeData(color >> 8);
//	writeData(color & 0xFF);
//}

trait SpiWritable {
    fn to_arr(&self) -> &[u8];
}
struct Data(u8);

impl SpiWritable for Data {
    fn to_arr(&self) -> &[u8] {
        &[self.0]
    }
}
struct Command(u8);

impl SpiWritable for Command {
    fn to_arr(&self) -> &[u8] {
        &[self.0]
    }
}

enum TftMode {
    NormalMode,
    PartialMode,
    IdleMode,
    SleepMode,
    InvertMode,
    DisplayOnMode,
    DisplayOffMode,
}
use rppal;

use rppal::gpio::Gpio;
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::io;
use std::io::prelude::*;

struct Tft {
    curr_mode: TftMode,
    spi: Spidev,
    dc: rppal::gpio::OutputPin,
    rst: rppal::gpio::OutputPin,
}

impl Tft {
    fn new() -> Self {
        let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(20_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options).unwrap();
        let gpio = Gpio::new().unwrap();
        let dc = gpio.get(23).unwrap().into_output();
        let rst = gpio.get(22).unwrap().into_output();

        Self {
            curr_mode: TftMode::NormalMode,
            spi,
            dc,
            rst,
        }
    }

    fn change_mode(&self, mode: TftMode) {
        match mode {
            TftMode::NormalMode => {}
            _ => {}
        }
    }

    fn write_spi(&mut self, cmd: impl SpiWritable) {
        self.dc_set_low();
        //some verfication
        self.spi.write(cmd.to_arr());
        //some verfication
    }

    fn dc_set_high(&mut self) {
        self.dc.set_high();
    }

    fn dc_set_low(&mut self) {
        self.dc.set_low();
    }

    fn rst_set_low(&mut self) {
        self.rst.set_low();
    }

    fn rst_set_high(&mut self) {
        self.rst.set_high();
    }

    //fn create_spi() -> io::Result<Spidev> {}
    //
    //    fn half_duplex(spi: &mut Spidev) -> io::Result<()> {
    //        let mut rx_buf = [0_u8; 10];
    //        spi.write(&[0x01, 0x02, 0x03])?;
    //        spi.read(&mut rx_buf)?;
    //        println!("{:?}", rx_buf);
    //        Ok(())
    //    }
}
