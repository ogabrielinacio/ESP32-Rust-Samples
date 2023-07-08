# Blinky led

In the default creation of `main.rs`, we need to include three more packages.

```rust
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
```

The `delay` module within the `esp_idf_hal` crate likely provides functionality related to delaying or pausing the execution of code for a certain amount of time. The `FreeRtos` trait represents a specific implementation or mechanism for performing delays using the FreeRTOS operating system, which is commonly used in embedded systems.

By importing the `FreeRtos` trait, the code gains access to the associated methods or functions that allow for controlling delays using the FreeRTOS implementation provided by the `esp_idf_hal` crate. These methods might include functions for delaying for a specific number of milliseconds or microseconds, or other timing-related operations.

##### After `println!("Hello world!");` include the following parts:

```rust
  let peripherals = Peripherals::take().unwrap();
```

When working with peripherals or resources on microcontrollers, it is common to use a method called `take()` to safely acquire ownership of those resources. The `take()` method is typically implemented as an associated function on the  peripheral struct, allowing you to retrieve the resources from the  microcontroller.  The `Peripherals::take()` call attempts to acquire ownership of the peripheral resources from the ESP32.

The `unwrap()` function is then called on the result of `Peripherals::take()` to extract the owned value from the `Option<Peripherals>` type. It assumes that the `take()` operation was successful and unwraps the value. If the `take()` method returns `None`, indicating a failure to acquire the resources, `unwrap()` will cause the program to panic.

```rust
let mut led = PinDriver::output(peripherals.pins.gpio32).unwrap();
```

This line of code initializes a mutable variable `led` with a PinDriver instance configured as an output pin associated with GPIO pin 32 on the ESP32 microcontroller.

```rust
led.set_high().unwrap();
```

This method sets the GPIO pin to a high or logic level of 1, indicating an active high state.

Inside loop, add this lines:

```rust
led.toggle().unwrap();
FreeRtos::delay_ms(100);
```

- `led.toggle().unwrap();`: This line toggles the state of the LED or the GPIO pin represented by the `led` variable, The `toggle()` method changes the state of the GPIO pin to the opposite of its current state. The `unwrap()` function is used to handle any potential errors that might occur during the toggle operation. If the operation is successful, `unwrap()` will simply return the value, and if an error occurs, it will panic and cause the program to terminate.
- `delay.delay_ms(100);`: introduce a delay of 100 milliseconds using the FreeRTOS operating system on the ESP32 microcontroller.

![](https://github.com/ogabrielinacio/Esp32-Rust/blob/main/with_std/Images/blink_led.gif)
