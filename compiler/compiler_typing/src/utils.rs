//! Utilities related to types.

/// Determines the size of a pointer / memory addressess based on the current platform.
pub fn get_pointer_size() -> usize {
    if cfg!(target_pointer_width = "32") {
        return 32;
    } else if cfg!(target_pointer_width = "64") {
        return 64;
    } else {
        return 0;
    }
}
