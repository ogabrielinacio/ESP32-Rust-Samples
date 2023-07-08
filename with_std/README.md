# This folder of repository represent the projects with std, a.k.a Standard library

Generate a Project with the command

```shell
cargo generate esp-rs/esp-idf-template cargo
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
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");
}
```

The first line is an import that defines the esp-idf entry-point when
 the root crate is a binary crate that defines a main function.

Then, we have a usual main function with a few lines on it:

- A call to `esp_idf_sys::link_patches` function that makes sure that a few patches to the ESP-IDF which are implemented in Rust are linked to the final executable.
- a call to `esp_idf_svc::log::EspLogger::initialize_default();` to create ESP logging.
- We print on our console the famous "Hello World!".
