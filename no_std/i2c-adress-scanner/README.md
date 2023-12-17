# I2C-adress-scanner

In the default creation of `main.rs`, we need to include three more packages in hal library, the `IO` , `prelude` and i2c.

So the hal part, looks like this now:

```rust
use hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    i2c::I2C,
    timer::TimerGroup,
    Rtc, IO,
};
```

##### after `println!("Hello world!");` include the following parts:

```rust
let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
```

Creates a new instance of the `IO` struct by calling its `new` associated function. The `IO` struct is part of the `esp32-hal` crate and provides a low-level interface for manipulating the GPIO pins on the ESP32.

The `IO::new` function takes two arguments: `peripherals.GPIO` and `peripherals.IO_MUX`. These arguments represent the GPIO and IO_MUX peripherals on the ESP32.

Now, let's declare the sda and sdl pins with this lines of code:

```rust
let sda = io.pins.gpio21.into_open_drain_output();
let scl = io.pins.gpio22.into_open_drain_output();
```

Define the working speed of I2C connection: 

```rust
let i2c_speed = _fugit_RateExtU32::kHz(400);
```

Create a new I2C instance. This will enable the peripheral but the peripheral won’t get automatically disabled when this gets dropped.

```rust
let mut i2c = I2C::new(
        peripherals.I2C0,
        sda,
        scl,
        i2c_speed,
        &mut system.peripheral_clock_control,
        &clocks,
    );
```

Let's put a println to show the user the I2C scanning starts:

```rust
println!("Scanning I2C bus...");
```

Define the buffer size: 

```rust
let mut buffer: [u8; 32] = [0; 32]; // max 32 esp32
```

Define a mutable variable `i2c_address` of type u8, and initializes it to 0.

```rust
let mut i2c_address: u8 = 0;
```

Iterate through all possible I2C addresses ( 0 to 127) and attempst to read data from the device at the current address (addr) and stores the result in the result variable, if a device is found at a particular address, the code prints a message and updates the `i2c_address` variable with the address of the found device.

```rust
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
```

If no I2C device was found (address is 0), print 'Not found'; otherwise, print the found address.

```rust
if i2c_address == 0 {
        println!("\n\nWas not found\n\n");
    } else {
        println!("\n\nI2C Adress was found in: 0x{:02X}\n\n", i2c_address);
    }
```
