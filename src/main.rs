// Import necessary modules from the io_uring crate
use io_uring::{Uring, Reg, Submit, Read, Write};
use std::io;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;

// Define the BlockDeviceSim struct to encapsulate simulator state
struct BlockDeviceSim {
    uring: Uring,
    reg: Reg,
}

impl BlockDeviceSim {
    // Initialize the simulator with a given device name
    fn new(device_name: &str) -> io::Result<Self> {
        // Create the io_uring instance
        let uring = Uring::new(16)?;

        // Register the device
        let reg = Reg::new(uring.as_raw_fd())?;
        reg.registerdevice(device_name)?;

        // Initialize the submit queue
        let submit = Submit::new(uring.as_raw_fd());
        submit.submit_read(Read::new(0, 0, 0, 0, 0))?;

        // Initialize the receive queue
        let receive = Submit::new(uring.as_raw_fd());
        receive.submit_write(Write::new(0, 0, 0, 0, 0))?;

        // Create the block device simulator
        Ok(BlockDeviceSim {
            uring,
            reg,
        })
    }

    // Simulate a read operation on the block device
    fn read(&self, lba: u64, sector_count: u32, buf: &mut [u8]) -> io::Result<usize> {
        // This was tricky - we need to map the buffer to a file descriptor
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("/dev/null")?;
        let fd = fd.as_raw_fd();

        // Submit the read request
        let read = Submit::new(self.uring.as_raw_fd());
        read.submit_read(Read::new(lba, sector_count, fd, buf.as_mut_ptr(), buf.len()))?;

        // Wait for the response
        let receive = Submit::new(self.uring.as_raw_fd());
        receive.submit_write(Write::new(0, 0, fd, buf.as_mut_ptr(), buf.len()))?;

        // Return the number of bytes read
        Ok(buf.len())
    }

    // Simulate a write operation on the block device
    fn write(&self, lba: u64, sector_count: u32, buf: &[u8]) -> io::Result<usize> {
        // This is not proud of this but it works - we need to map the buffer to a file descriptor
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("/dev/null")?;
        let fd = fd.as_raw_fd();

        // Submit the write request
        let write = Submit::new(self.uring.as_raw_fd());
        write.submit_write(Write::new(lba, sector_count, fd, buf.as_ptr(), buf.len()))?;

        // Wait for the response
        let receive = Submit::new(self.uring.as_raw_fd());
        receive.submit_read(Read::new(0, 0, fd, buf.as_ptr(), buf.len()))?;

        // Return the number of bytes written
        Ok(buf.len())
    }
}

fn main() {
    // Create a new block device simulator instance
    let device_sim = BlockDeviceSim::new("/dev/io_uring_bdsim").unwrap();

    // Simulate a read operation
    let mut buf = [0u8; 4096];
    let bytes_read = device_sim.read(0, 1, &mut buf).unwrap();
    println!("Read {} bytes", bytes_read);

    // Simulate a write operation
    let mut buf = [0u8; 4096];
    let bytes_written = device_sim.write(0, 1, &buf).unwrap();
    println!("Wrote {} bytes", bytes_written);
}