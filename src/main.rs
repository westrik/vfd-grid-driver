//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOA and GPIOB peripherals
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // PA15, PB3, PB4 used for JTAG by default
    // https://docs.rs/stm32f1xx-hal/0.7.0/stm32f1xx_hal/afio/struct.MAPR.html#method.disable_jtag
    let (pa15, _pb3, _pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    let reset = gpioa.pa4.into_pull_down_input(&mut gpioa.crl);
    let write_strobe = gpioa.pa12.into_open_drain_output(&mut gpioa.crh);
    let busy = pa15.into_pull_down_input(&mut gpioa.crh);
    let d0 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let d1 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let d2 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let d3 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);
    let d4 = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let d5 = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
    let d6 = gpioa.pa10.into_push_pull_output(&mut gpioa.crh);
    let d7 = gpioa.pa11.into_push_pull_output(&mut gpioa.crh);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}
