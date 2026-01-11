//! Print "Hello world!" with "Hello rust!" underneath. Uses the `embedded_graphics` crate to draw
//! the text with a 10x20 pixel font, followed by a running horse animation.
//!
//! This example is for the STM32C092KCT6 using I2C2.
//!
//! Wiring connections are as follows:
//!
//! ```
//! Display -> STM32C092KCT6
//!     GND -> GND
//!     +5V -> VCC
//!     SCL -> PA7
//!     SDA -> PA6
//! ```
//!
//! Run with `cargo run --release`.

#![no_std]
#![no_main]

pub mod horses;

use crate::horses::HORSES;
// use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::rcc::{Hsi, HsiKerDiv, HsiSysDiv};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::Timer;
use embedded_graphics::image::Image;
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_10X20},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use panic_probe as _;
use ssd1306::Ssd1306Async;
use ssd1306::{I2CDisplayInterface, prelude::*};

bind_interrupts!(struct Irqs {
    I2C2 => i2c::EventInterruptHandler<peripherals::I2C2>, i2c::ErrorInterruptHandler<peripherals::I2C2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut sysconfig = embassy_stm32::Config::default();
    sysconfig.rcc.hsi = Some(Hsi {
        sys_div: HsiSysDiv::DIV1,
        ker_div: HsiKerDiv::DIV1,
    });
    let peripherals = embassy_stm32::init(sysconfig);
    // let peripherals = embassy_stm32::init(Default::default());

    let i2c_peri = peripherals.I2C2;
    let scl = peripherals.PA7;
    let sda = peripherals.PA6;

    let mut i2c_config = Config::default();
    i2c_config.frequency = Hertz(1_000_000);

    // Wait for slave device to be ready
    Timer::after_millis(100).await;

    let tx_dma = peripherals.DMA1_CH2;
    let rx_dma = peripherals.DMA1_CH3;

    let i2c = I2c::new(i2c_peri, scl, sda, Irqs, tx_dma, rx_dma, i2c_config);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().await.unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Hello Rust!", Point::new(0, 20), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().await.unwrap();

    Timer::after_millis(3000).await;

    let mut index: usize = 0;
    loop {
        let image = Image::new(&HORSES[index % 8], Point::zero());
        image.draw(&mut display).unwrap();
        display.flush().await.unwrap();
        index += 1;
    }
}
