#![no_std]
#![no_main]

use esp_backtrace as _;
use hal::i2c::I2C;
use hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc, IO,
};
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};

// I2C Adress was found in: 0x3F
const I2C_ADDRESS: u8 = 0x3F;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    ///////STARTING APPLICATION
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sda = io.pins.gpio21.into_open_drain_output();
    let scl = io.pins.gpio22.into_open_drain_output();
    let i2c_speed = _fugit_RateExtU32::kHz(400);
    let mut delay = Delay::new(&clocks);
    let  i2c = I2C::new(
        peripherals.I2C0,
        sda,
        scl,
        i2c_speed,
        &mut system.peripheral_clock_control,
        &clocks,
    );
    let mut lcd = HD44780::new_i2c(i2c, I2C_ADDRESS, &mut delay).expect("Init LCD failed");
    let _ = lcd.reset(&mut delay);
    let _ = lcd.clear(&mut delay);
    let _ = lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Visible,
            cursor_blink: CursorBlink::On,
        },
        &mut delay,
    );
    let _ = lcd.write_str("RUST IS AMAZING!", &mut delay);

    loop {}
}
