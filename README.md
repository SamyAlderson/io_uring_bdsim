# io_uring_bdsim
A simple block device simulator for kernel development and testing

## What it does
io_uring_bdsim is a block device simulator that uses io_uring for kernel development and testing. It allows you to test and experiment with block device functionality without needing to set up a physical device. This can be useful for kernel developers who want to test and debug their code without the overhead of a physical device.

## Install
```bash
cargo add io_uring_bdsim
```

## Usage
```rust
use io_uring_bdsim::*;

fn main() {
    // Create a new block device simulator
    let mut bdsim = BlockDeviceSim::new();

    // Add a new block device to the simulator
    let device = bdsim.add_device("my_device");

    // Read 10 blocks from the device
    let mut buffer = [0; 4096];
    for _ in 0..10 {
        device.read(&mut buffer).unwrap();
    }
}
```

## Build from source
```bash
cargo build
```

## Tests
```bash
cargo test
```

## Project structure
- `src/main.rs`: Entry point of the program
- `src/block_device_sim.rs`: Implementation of the block device simulator
- `src/block_device.rs`: Implementation of the block device
- `tests/block_device_sim.rs`: Tests for the block device simulator
- `Cargo.toml`: Rust project configuration file

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