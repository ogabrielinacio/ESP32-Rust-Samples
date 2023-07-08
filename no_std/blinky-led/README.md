# Blinky led

In the default creation of `main.rs`, we need to include two more packages in hal library, the `IO` and `Delay`.

So the hal part, looks like this now:

```rust
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO, Delay};
```

##### After `println!("Hello world!");` include the following parts:

```rust
let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
```

Creates a new instance of the `IO` struct by calling its `new` associated function. The `IO` struct is part of the `esp32-hal` crate and provides a low-level interface for manipulating the GPIO pins on the ESP32.

The `IO::new` function takes two arguments: `peripherals.GPIO` and `peripherals.IO_MUX`. These arguments represent the GPIO and IO_MUX peripherals on the ESP32. 

Now, let's declare the pin of led with this line of code:

```rust
let mut led = io.pins.gpio32.into_push_pull_output();
```

Use the io instance io to access the GPIO pin 32.

The function `into_push_pull_output()` is then called on the GPIO pin object retrieved (`io.pins.gpio32`). This function configures the GPIO pin to operate in push-pull output mode. In push-pull mode, the GPIO pin can both drive the output signal high and low actively.

Finally, the resulting configured GPIO pin is assigned to the mutable variable `led`. Therefore, the variable `led` would represent the GPIO pin `gpio32` configured as a push-pull output on the ESP32

Configurate the led to high:

```rust
 led.set_high().unwrap();
```

This method sets the GPIO pin to a high or logic level of 1, indicating an active high state.

The `unwrap()` function is used to handle the potential error that may occur when setting the GPIO pin to a high state. If the operation is successful, `unwrap()` will simply return the value, and if an error occurs, it will panic and cause the program to terminate.

Creation of delay instance:

```rust
let mut delay = Delay::new(&clocks);
```

creates a new instance of the `Delay` struct and assigns it to the mutable variable `delay`. The `Delay` struct is typically used for introducing time delays or pauses in the execution of a program.

The `new()` function is called on the `Delay` struct, and it takes a reference to the `clocks` object as its argument. The `clocks` object likely represents the clock configuration or a source of timing information for the microcontroller. By passing the `clocks` object to the `new()` function, the `Delay` instance is initialized and configured to use the provided clock configuration or timing information for its operations.

Inside  loop, add this lines:

```rust
led.toggle().unwrap();
delay.delay_ms(500u32);
```

1. `led.toggle().unwrap();`: This line toggles the state of the LED or the GPIO pin represented by the `led` variable,  The `toggle()` method changes the state of the GPIO pin to the opposite of its current state. The `unwrap()` function is used to handle any potential errors that might occur during the toggle operation. If the operation is successful, `unwrap()` will simply return the value, and if an error occurs, it will panic and cause the program to terminate.
2. `delay.delay_ms(500u32);`: This line introduces a delay of 500 milliseconds using the `delay_ms()` method of the `delay` object. The `delay_ms()` method suspends the program execution for the specified number of milliseconds (in this case, 500 milliseconds). The `500u32` argument represents an unsigned 32-bit integer literal specifying the duration of the delay.

   
![](https://github.com/ogabrielinacio/Esp32-Rust/blob/main/no_std/Images/blink_led.gif)
