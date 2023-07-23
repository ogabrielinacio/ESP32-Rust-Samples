#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::i2c::I2C;
use hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::{_fugit_RateExtU32, *},
    timer::TimerGroup,
    Rtc, IO,
};

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
    println!("Hello world!");
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sda = io.pins.gpio21.into_open_drain_output();
    let scl = io.pins.gpio22.into_open_drain_output();
    let i2c_speed = _fugit_RateExtU32::kHz(400);

    let mut i2c = I2C::new(
        peripherals.I2C0,
        scl,
        sda,
        i2c_speed,
        &mut system.peripheral_clock_control,
        &clocks,
    );

    println!("Scanning I2C bus...");
    let mut buffer: [u8; 32] = [0; 32]; // max 32 esp32
    let mut i2c_address: u8 = 0;
    for addr in 0..=127 {
        let result = i2c.read(addr, &mut buffer);
        match result {
            Ok(_) => {
                println!("Found device at address: 0x{:02X}", addr);
                i2c_address = addr;
            }
            Err(_) => println!("Error encountered while scanning address: 0x{:02X}", addr),
        }
    }
    if i2c_address == 0 {
        println!("\n\nWas not found\n\n");
    } else {
        println!("\n\nI2C Adress was found in: 0x{:02X}\n\n", i2c_address);
    }

    loop {}
}
