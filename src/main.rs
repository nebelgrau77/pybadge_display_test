#![no_std]
#![no_main]

use edgebadge as hal;
use panic_halt as _;

use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::prelude::*;
use embedded_graphics::{egrectangle, primitive_style};
use embedded_graphics::fonts::{Font12x16, Text};
use embedded_graphics::style::TextStyle;
use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::delay::Delay;
use hal::prelude::*;

const BOOT_DELAY_MS: u32 = 100; 

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // short delay to allow the bus to start correctly
    delay.delay_ms(BOOT_DELAY_MS); 

    let (mut display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM4,
            &mut peripherals.MCLK,
            peripherals.TC2,
            &mut delay,
            &mut pins.port,
        )
        .unwrap();

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (160, 128),
        style = primitive_style!(stroke_width = 0, fill_color = RgbColor::BLACK)
    )
    .draw(&mut display)
    .unwrap();

    let text = "Hello Rust!";
    let width = text.len() as i32 * 6;
    let text_style = TextStyle::new(Font12x16, Rgb565::new(20,60,80));
    Text::new(text, Point::new(20, 20)).into_styled(text_style).draw(&mut display).unwrap();

    loop {}
}