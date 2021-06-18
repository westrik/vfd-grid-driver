#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use alloc_cortex_m::CortexMHeap;
use cortex_m_rt::entry;
// use cortex_m_semihosting::{hprintln, debug};
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};
use vfd_grid_driver::display::VfdDisplay;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024;

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

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

    let mut display = VfdDisplay {
        d0,
        d1,
        d2,
        d3,
        d4,
        d5,
        d6,
        d7,
        write_strobe,
        reset,
    };

    display.reset().unwrap_or(());

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    let test_string = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ÀÁÂÄÅÆÇÈÉÊËÌÍÎÏÑÒÓÔÖÙÚÛÜßàáâäåæçèéêëìíîïñòóôöøùúûü!?#%&*+÷±=.,'\"`°-_^:;/()[]{}<>@$¢€￥⊙○♦";
    // TODO: send characters in a loop, with configurable delay after each

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}
