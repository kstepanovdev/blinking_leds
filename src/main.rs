#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f407g_disc as board;
use board::{
    hal::stm32,
    hal::{delay::Delay, prelude::*},
    led::{LedColor, Leds}
};
use cortex_m::peripheral::Peripherals;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Turn LEDs on one after the other with 500ms delay between them
            leds[LedColor::Orange].on();
            leds[LedColor::Red].on();
            leds[LedColor::Blue].on();
            leds[LedColor::Green].on();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(1000_u16);

            // Turn LEDs off one after the other with 500ms delay between them
            leds[LedColor::Orange].off();
            leds[LedColor::Red].off();
            leds[LedColor::Blue].off();
            leds[LedColor::Green].off();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(1000_u16);
        }
    }

    loop {
        continue;
    }
}
