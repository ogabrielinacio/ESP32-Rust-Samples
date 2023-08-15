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


