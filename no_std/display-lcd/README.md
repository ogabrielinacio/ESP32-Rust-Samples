First we need to scan i2c address.

first you need to add the `hd44780-driver`, use the follow cargo command:

```rust
cargo add hd44780-driver
```

add the following dependencies to your main.rs

```rust
use hd44780_driver::{HD44780, DisplayMode, Cursor, CursorBlink, Display};
```

we also need to include i2c library: 

```rust
use hal::i2c::I2c;
```

| **I2C LCD** | **ESP32** |
| ----------- | --------- |
| GND         | GND       |
| VCC         | VIN       |
| SDA         | GPIO 21   |
| SCL         | GPIO 22   |
