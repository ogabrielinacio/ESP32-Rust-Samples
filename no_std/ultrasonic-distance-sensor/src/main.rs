#![no_std]
#![no_main]
use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO, Delay};

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
    let mut trig = io.pins.gpio5.into_push_pull_output();
    let echo = io.pins.gpio18.into_floating_input();
    let mut delay = Delay::new(&clocks);

    loop {
         // 1) Set pin ouput to low for 5 us to get clean low pulse 5microsegos
        trig.set_low().unwrap();
        // delay.delay_us(5_u32);
        delay.delay_us(2_u32);

        // 2) Set pin output to high (trigger) for 10us 10micro
        trig.set_high().unwrap();
        delay.delay_us(10_u32);
        trig.set_low().unwrap();

        // Wait until pin goes high
        while !echo.is_high().unwrap() {}

        // Collect current timer count
        let echo_start = rtc.get_time_ms();

        // Wait until pin goes low
        while !echo.is_low().unwrap() {}

        // Collect current timer count
        let echo_end =  rtc.get_time_ms();
        // Calculate the elapsed timer count
        let echo_dur = echo_end.wrapping_sub(echo_start);
        let echo_dur_float = echo_dur as f32;
        // Calculate the distance in cms using formula in datasheet
        // #define SOUND_SPEED 0.034
        // let distance_cm = (echo_dur_float * 0.034) / 2.0;
        // let distance_cm = ((echo_dur_float * 340.0)/2.0) * 0.0001;
        let distance_cm = (echo_dur_float * 0.034) / 2.0;

        // Print the distance output
         println!("\n\nDistance {} cm\r", distance_cm);
    }

}
