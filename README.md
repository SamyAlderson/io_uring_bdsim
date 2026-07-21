# io_uring_bdsim
A block device simulator for kernel development and testing

A simple io_uring based block device simulator for testing and developing kernel block device code.

## How to Install and Use

To install, add the following line to your `Cargo.toml`:
```toml
[dependencies]
io_uring_bdsim = "0.1"
```
Then, use it in your kernel development or testing code with:
```rust
use io_uring_bdsim as bdsim;

// Create a block device simulator
let bdsim = bdsim::BlockDeviceSimulator::new();

// Perform I/O operations on the simulated block device
// ...
```
## Building from Source

Clone the repository and run:
```bash
cargo build
```
This will build the project.

## Running Tests

To run the test suite, use:
```bash
cargo test
```
## Project Structure

* `Cargo.toml`: project metadata
* `src/lib.rs`: block device simulator implementation
* `src/block_device_simulator.rs`: block device simulator module
* `src/io_uring.rs`: io_uring support module
* `tests/test_block_device.rs`: test for block device simulation
* `tests/test_io_uring.rs`: test for io_uring support

## License

Copyright (c) 2026 SamyAlderson

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.