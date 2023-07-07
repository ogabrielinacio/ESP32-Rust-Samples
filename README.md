## 

## Goals:

This repository has as its main objective the study of Rust on an Esp32.

## Introdution:

##### For more information consult `[Introduction - The Rust on ESP Book](https://esp-rs.github.io/book/introduction.html)`

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
2.  When you might want to use bare metal (`no_std`):
   
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

2.  Install espup, a required tool to develop Rust application for the `Xtensa` and `RISC-V` architectures

```bash
cargo install espup
```

    2.2  Instal all the necessary tools to develop Rust applications

```bash
espup install
```

    3. Set up the environment variables:









## References

1. [Embedded Rust on ESP32 - Juraj Michálek - Rust Linz November 2022 - YouTube](https://www.youtube.com/watch?v=0PPPdqoDBQs)

2. [Introduction - The Rust on ESP Book](https://esp-rs.github.io/book/)
