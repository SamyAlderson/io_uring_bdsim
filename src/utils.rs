// Utility functions for the simulator

pub mod utils {
    use std::fmt;

    /// Converts bytes to a human-readable string with units.
    pub fn bytes_to_str(bytes: u64) -> String {
        let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
        let mut i = 0;
        while bytes >= 1024 && i < units.len() - 1 {
            bytes /= 1024;
            i += 1;
        }
        format!("{bytes} {units[i]}")
    }

    /// Maps an error to a more informative message.
    pub fn error_map<T, E>(error: E) -> Result<T, String>
    where
        E: fmt::Debug,
    {
        let error_message = format!("{:?}", error);
        Err(error_message)
    }

    /// Returns the number of bytes in a buffer.
    pub fn buffer_bytes(buffer: &[u8]) -> u64 {
        buffer.len() as u64
    }

    /// Calculates the total size of a buffer.
    pub fn buffer_total_size(buffer: &[u8]) -> u64 {
        buffer.len() as u64
    }
}