#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
// use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;
use rp2040_hal as hal;

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    watchdog::Watchdog,
    Sio,
};



#[link_section = ".boot2"]
#[used]

pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
	external_xtal_freq_hz,
	pac.XOSC,
	pac.CLOCKS,
	pac.PLL_SYS,
	pac.PLL_USB,
	&mut pac.RESETS,
	&mut watchdog,
    )	.ok()
	.unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST,
						clocks.system_clock.freq()
						.to_Hz());

    let pins = hal::gpio::Pins::new(
	pac.IO_BANK0,
	pac.PADS_BANK0,
	sio.gpio_bank0,
	&mut pac.RESETS,
    );

    // Red, Green, Blue on the Xiao RP2040
    let mut led_pin1 = pins.gpio17.into_push_pull_output();
    let mut led_pin2 = pins.gpio16.into_push_pull_output();
    let mut led_pin3 = pins.gpio25.into_push_pull_output();	

    let mut iter = 0;
    
    loop {
	delay.delay_ms(500);
	
	iter = iter + 1;
	if (iter & 1) == 1 {
	    led_pin1.set_high().unwrap();
	} else {
	    led_pin1.set_low().unwrap();
	}

	if ((iter >> 1) & 1) == 1 {
	    led_pin2.set_high().unwrap();
	} else {
	    led_pin2.set_low().unwrap();
	}

	if ((iter >> 2) & 1) == 1 {
	    led_pin3.set_high().unwrap();
	} else {
	    led_pin3.set_low().unwrap();
	}
    }
}

 
