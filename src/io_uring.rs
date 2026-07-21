// io_uring wrapper and utility functions

use io_uring::{IoUring, CQEntry, SQEntry};
use std::io;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};

pub struct IoUringSim {
    ring: Arc<IoUring>,
}

impl IoUringSim {
    pub fn new() -> io::Result<Self> {
        let ring = Arc::new(IoUring::new(32)?);
        let sim = IoUringSim { ring };
        Ok(sim)
    }

    pub fn submit(&self, sqe: &mut SQEntry) -> io::Result<()> {
        self.ring.submit_sqs(vec![sqe])?;
        Ok(())
    }

    pub fn register(&self, fd: i32, events: io_uring_event) -> io::Result<()> {
        self.ring.register(fd, events)?;
        Ok(())
    }

    pub fn get_cqe(&self) -> io::Result<CQEntry> {
        self.ring.get_cqe()
    }

    pub fn get_sqe(&self) -> io::Result<SQEntry> {
        self.ring.get_sqe()
    }
}

impl Drop for IoUringSim {
    fn drop(&mut self) {
        self.ring.cancel_all_sqs();
    }
}

pub fn create_ring() -> io::Result<IoUring> {
    // This was tricky, the IoUring API doesn't provide a way to create a new ring with a specific size
    // So we have to use the default size and then resize it afterwards
    let mut ring = IoUring::new(32)?;
    ring.resize(128)?;
    Ok(ring)
}

pub fn open_file(path: &str) -> io::Result<std::fs::File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?
        .into_raw_fd();
    let ring = create_ring()?;
    let sqe = ring.get_sqe()?;
    sqe.opcode(io_uring_op::OP_OPEN);
    sqe.file_descriptor(file);
    ring.submit(&mut sqe)?;
    let cqe = ring.get_cqe()?;
    if cqe.result != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(std::fs::File::from_raw_fd(file))
    }
}