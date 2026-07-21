//! Block device simulator implementation using io_uring
//!
//! This module provides a basic block device simulator using io_uring for kernel development and testing.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};
use std::thread;

use io_uring::{IoUring, IoVec};

// Block device simulator state
pub struct BdsimState {
    // io_uring instance
    io_uring: IoUring,
    // block device file descriptor
    bdev_fd: i32,
    // simulator thread handle
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl BdsimState {
    // Initialize a new block device simulator
    pub fn new(io_uring: IoUring, bdev_fd: i32) -> Self {
        BdsimState {
            io_uring,
            bdev_fd,
            thread_handle: None,
        }
    }

    // Start the simulator thread
    pub fn start(&mut self) {
        // Spin up a new thread to simulate block device I/O
        self.thread_handle = Some(thread::spawn(move || {
            // Simulate block device I/O using io_uring
            self.simulate_io();
        }));
    }

    // Stop the simulator thread
    pub fn stop(&mut self) {
        // Join the simulator thread to wait for completion
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }

    // Simulate block device I/O using io_uring
    fn simulate_io(&self) {
        // Prepare a ring buffer for I/O submission
        let mut ring = self.io_uring.ring();
        // Initialize I/O vector with a single buffer
        let mut iov = IoVec::new(vec![0; 4096]);
        // Prepare an I/O submission entry
        let mut sqe = ring.sq_entries_mut()[0];
        // Set up the submission entry with I/O details
        sqe.set_fd(self.bdev_fd);
        sqe.set_off(0);
        sqe.set_len(4096);
        sqe.set_flags(0);
        // Add the I/O vector to the submission entry
        sqe.add_iov(&iov);
        // Submit the I/O request
        self.io_uring.submit_entries(&mut ring, 1);
        // Wait for the I/O completion
        self.io_uring.get_cqe(&mut ring);
    }
}

// Block device simulator API
pub struct BdsimApi {
    // Block device simulator state
    state: Arc<Mutex<BdsimState>>,
}

impl BdsimApi {
    // Create a new block device simulator instance
    pub fn new(io_uring: IoUring, bdev_fd: i32) -> Self {
        // Initialize the simulator state
        let state = Arc::new(Mutex::new(BdsimState::new(io_uring, bdev_fd)));
        // Create a new simulator API instance
        BdsimApi { state }
    }

    // Start the simulator thread
    pub fn start(&self) {
        // Acquire the simulator state lock
        let mut state = self.state.lock().unwrap();
        // Start the simulator thread
        state.start();
    }

    // Stop the simulator thread
    pub fn stop(&self) {
        // Acquire the simulator state lock
        let mut state = self.state.lock().unwrap();
        // Stop the simulator thread
        state.stop();
    }

    // Simulate block device I/O using io_uring
    pub fn simulate_io(&self) {
        // Acquire the simulator state lock
        let mut state = self.state.lock().unwrap();
        // Simulate block device I/O
        state.simulate_io();
    }
}