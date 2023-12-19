First we need to scan i2c address.

first you need to add the `hd44780-driver`, use the follow cargo command:

```rust
cargo add hd44780-driver
```

add the following dependencies to your main.rs

```rust
use hd44780_driver::{HD44780, DisplayMode, Cursor, CursorBlink, Display};
```

```rust
use hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc, IO,
};
```

we also need to include i2c library: 

```rust
use hal::i2c::I2C;
```

Also, using the `i2c-address-scanner` define a variable to hold the i2c address: 

```rust
const I2C_ADDRESS: u8 = 0x3F;
```

| **I2C LCD** | **ESP32** |
| ----------- | --------- |
| GND         | GND       |
| VCC         | VIN       |
| SDA         | GPIO 21   |
| SCL         | GPIO 22   |

##### after `wdt1.disable();` include the following parts:

Define the SDA and SCL pins:

```rust
let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
let sda = io.pins.gpio21.into_open_drain_output();
let scl = io.pins.gpio22.into_open_drain_output();
```

Sets the I2C bus speed to 400 KHz, create a delay object using clocks reference, and create a I2C object with this variables.

```rust
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
```
