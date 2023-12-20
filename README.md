# ESP32 + Rust Samples:  

## Goals:

This repository has as its main objective the study of Rust on an Esp32.

## Introdution:

##### For more information consult: `https://esprs.github.io/book/introduction.html`

There are two approaches to use Rust on ESP32

- Using the `std`  library a.k.a Standard library.

- Using the `core` library (`no_std`) a.k.a Bare metal development.

###### You should make a decision base on your project's needs.

1. When you might want to use the Standart Library (std):
- Rich functionality: If your embedded system requires lots of 
  functionality like support for networking protocols, file I/O, or 
  complex data structures, you will likely want to use hosted-environment 
  approach because std libraries provide a wide range of functionality 
  that can be used to build complex applications relatively quickly and 
  efficiently
- Portability: The `std` crate provides a 
  standardized set of APIs that can be used across different platforms and
   architectures, making it easier to write code that is portable and 
  reusable.
- Rapid development: The `std` crate provides a
   rich set of functionality that can be used to build applications 
  quickly and efficiently, without worrying, too much, about low-level 
  details.
2. When you might want to use bare metal (`no_std`):
   
   - Small memory footprint: If your embedded system has limited 
     resources and needs to have a small memory footprint, you will likely 
     want to use bare-metal because `std` features add a significant amount of final binary size and compilation time.
   - Direct hardware control: If your embedded system requires more 
     direct control over the hardware, such as low-level device drivers or 
     access to specialized hardware features you will likely want to use 
     bare-metal because `std` adds abstractions that can make it harder to interact directly with the hardware.
   - Real-time constraints or time-critical applications: If your 
     embedded system requires real-time performance or low-latency response 
     times because `std` can introduce unpredictable delays and overhead that can affect real-time performance.
   - Custom requirements: bare-metal allows more customization and 
     fine-grained control over the behavior of an application, which can be 
     useful in specialized or non-standard environments.

## Instalation:

1. First you need to install Rust and Cargo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install espup, a required tool to develop Rust application for the `Xtensa` and `RISC-V` architectures

```bash
cargo install espup
```

    2.2  Instal all the necessary tools to develop Rust applications

```bash
espup install
```

`espup` will create an export file that contains some environment variables required to build projects.

    3. Set up the environment variables:

       There are 3 ways of set up the environment variables. I used this:

```bash
cat export-esp.sh >> ~/.bashrc
```

For more information, consult:

 `https://esp-rs.github.io/book/installation/index.html#3-set-up-the-environment-variables`

4. You also need the `std development requirements` :
- [`python`](https://www.python.org/downloads/): Required by ESP-IDF
- [`git`](https://git-scm.com/downloads): Required by ESP-IDF
- [`ldproxy`](https://github.com/esp-rs/embuild/tree/master/ldproxy) binary crate: A tool that forwards linker arguments to the actual linker that is also given as an argument to `ldproxy`. Install it by running:

```sh
cargo install ldproxy
```

You dont need to install the ESP-IDF (Espressif IoT Development Framework), is automatically downloaded and installed by [esp-idf-sys](https://github.com/esp-rs/esp-idf-sys).

5.    Generating Projects from Templates

There are two tools to create projects in Rust on ESP32

- [esp-template](https://github.com/esp-rs/esp-template) - `no_std` template.
- [esp-idf-template](https://github.com/esp-rs/esp-idf-template) - `std` template.

First, you need to instal the [cargo-generate](https://github.com/cargo-generate/cargo-generate), a tool that allows you to create a new project based on some existing template.

    Install `cargo-generate`:

```bash
cargo install cargo-generate
```

Generate a project based in one of the templates:

esp-template (no_std):

- ```shell
  cargo generate -a esp-rs/esp-template
  ```

- esp-idf-template (with std):

```shell
cargo generate esp-rs/esp-idf-template cargo
```

Once you execute the `cargo generate` subcommand, a series of
 questions will be presented to gather information about your 
application's target. After successfully providing the required answers,
 you will obtain a project that can be built, complete with the 
appropriate configuration.

- Build/Run the generated project:
  
  - Using `cargo build` will compile the project using the appropriate toolchain and target.
  - Using `cargo run` will compile the project, flash it, and open a serial monitor with our chip. You need to press  `BOOT` button to flash esp32,  just for about 3 seconds after running the command `cargo run`.

## References

1. [Embedded Rust on ESP32 - Juraj Michálek - Rust Linz November 2022 - YouTube](https://www.youtube.com/watch?v=0PPPdqoDBQs)

2. [Introduction - The Rust on ESP Book](https://esp-rs.github.io/book/)
