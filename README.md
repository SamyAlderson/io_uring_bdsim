# io_uring_bdsim
A block device simulator using io_uring for kernel development and testing

## What and Why
This project aims to provide a block device simulator using io_uring for kernel development and testing. It allows developers to test and experiment with io_uring-based block device drivers without the need for physical hardware.

## Install
To use this project, you'll need Rust installed on your system. You can install it from the official [Rust installation page](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can clone this repository and run the following command to build and run the simulator:
```bash
cargo run
```
This will compile the project and run the simulator.

## Usage
The simulator can be used as follows:
```bash
cargo run -- --help
```
This will display a help message with usage instructions.

## Build from Source
To build the project from source, you'll need to have Rust installed on your system. You can clone this repository and run the following command:
```bash
cargo build
```
This will compile the project.

## Project Structure
The project is structured as follows:
```plaintext
Cargo.toml
src/
main.rs
bd_sim.rs
utils.rs
io_uring.rs
tests/
main.rs
Cargo.lock
rustfmt.toml
README.md
.gitignore
```
## License
This project is licensed under the MIT License.

## Features
The project supports the following features:

* `io_uring_support`: Enables support for io_uring.
* `block_device_simulation`: Enables block device simulation.

## Dependencies
The project depends on the `io_uring` crate version 0.4.0.

## Contributing
Contributions are welcome! Please submit a pull request with your changes.

## Architecture
The simulator is implemented as a Rust program that uses the `io_uring` crate to interact with the kernel. The simulator uses a block device simulator implementation that provides a virtual block device that can be used for testing and experimentation.

The simulator is structured as follows:

* `src/main.rs`: The main entry point of the simulator.
* `src/bd_sim.rs`: The block device simulator implementation.
* `src/utils.rs`: Utility functions for the simulator.
* `src/io_uring.rs`: io_uring wrapper and utility functions.

The simulator uses a number of idiomatic Rust conventions, including Rust's error handling mechanism and its idiomatic use of iterators and closures.

The simulator is designed to be extensible and flexible, allowing developers to easily add new features and functionality.