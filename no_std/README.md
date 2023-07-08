# This folder of repository represent the projects without std, a.k.a Standard library

Generate a Project with the command

```rust
cargo generate -a esp-rs/esp-template
```

After running the command, there will be a few prompts:

- `Project Name`: Name of the crate. Example: hello-world

- `Which MCU to target?`: SoC model. For this repository, esp32

- `Configure advanced template options?`: If `false`, skips the rest of the prompts and uses their default value. If `true`, you will be prompted some configurations options, like `Enable allocations via the esp-alloc crate?`, 

`Configure project to support Wokwi simulation with Wokwi VS Code extension?` and `Configure project to use Dev Containers (VS Code and GitHub Codespaces)?`. 

I usually used as false.

it's will be create the following files:

```bash
$ tree hello-world
hello-world/
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── rust-toolchain.toml
└── src
    └── main.rs
```

- [.cargo/config.toml](https://doc.rust-lang.org/cargo/reference/config.html)
  - The Cargo configuration
  - This defines a few options to correctly build the project
  - Contains `runner = "espflash flash --monitor"` - this means you can just use `cargo run` to flash and monitor your code
- src/main.rs
  - The main source file of the newly created project
  - For details, see the [`main.rs`](https://esp-rs.github.io/book/writing-your-own-application/generate-project/esp-template.html#mainrs) section below.
- [.gitignore](https://git-scm.com/docs/gitignore)
  - Tells `git` which folders and files to ignore
- [Cargo.toml](https://doc.rust-lang.org/cargo/reference/manifest.html)
  - The usual Cargo manifest declaring some meta-data and dependencies of the project
- LICENSE-APACHE, LICENSE_MIT
  - Those are the most common licenses used in the Rust ecosystem
  - If you want to apply a different license, you can delete these files and change the license in `Cargo.toml`
- [rust-toolchain.toml](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)
  - Defines which Rust toolchain to use
    - The toolchain will be `nightly` or `esp` depending on your target.

###### main.rs

```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc};
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

    loop {}
}
```

Well ... this is a quite a lot of code, Let's see what it is good for:

- `#![no_std]`
  - This tells the Rust compiler that this code doesn't use `libstd`
- `#![no_main]`
  - The `no_main` attribute says that this 
    program won't use the standard main interface, which is tailored for 
    command-line applications that receive arguments. Instead of the 
    standard main, we'll use the entry attribute from the `riscv-rt` crate to define a custom entry point. In this program, we have named the entry point `main`, but any other name could have been used. The entry point function must be a [diverging function](https://doc.rust-lang.org/beta/rust-by-example/fn/diverging.html). I.e. it has the signature `fn foo() -> !`; this type indicates that the function never returns – which means that the program never terminates.
- `use esp_backtrace as _;`
  - Since we are in a bare-metal environment, we need a panic-handler that runs if a panic occurs in code
  - There are a few different crates you can use (e.g `panic-halt`) but `esp-backtrace` provides an implementation that prints the address of a backtrace - together with `espflash`/`espmonitor` these addresses can get decoded into source code locations
- `use esp_println::println;`
  - Provides `println!` implementation
- `use hal:{...}`
  - We need to bring in some types we are going to use
  - These are from `esp-hal`
- `let peripherals = Peripherals::take().unwrap();`
  - HAL drivers usually take ownership of peripherals accessed via the PAC
  - Here we take all the peripherals from the PAC to pass them to the HAL drivers later
- `let system = peripherals.SYSTEM.split();`
  - Sometimes a peripheral (here the System peripheral) is 
    coarse-grained and doesn't exactly fit the HAL drivers - so here we 
    split the System peripheral into smaller pieces which get passed to the 
    drivers
- `let clocks = ClockControl::boot_defaults(system.clock_control).freeze();`
  - Here we configure the system clocks - in this case, we are fine with the defaults
  - We freeze the clocks, which means we cannot change them later
  - Some drivers need a reference to the clocks to know how to calculate rates and durations
- The next block of code instantiates some peripherals (namely RTC and
   the two timer groups) to disable the watchdog, which is armed after 
  boot
  - Without that code, the SoC would reboot after some time
  - There is another way to prevent the reboot: [feeding](https://docs.rs/esp32c3-hal/0.2.0/esp32c3_hal/prelude/trait._embedded_hal_watchdog_Watchdog.html#tymethod.feed) the watchdog
- `println!("Hello world!");`
  - Prints "Hello Wolrd!"
- `loop {}`
  - Since our function is supposed to never return, we just "do nothing" in a loop

Now. just run the code:

```shell
cargo run
```

You can reboot with `CTRL+R` or exit with `CTRL+C`.
