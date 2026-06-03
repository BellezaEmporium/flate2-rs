//! This module contains backend-specific code.

use crate::Compression;
use crate::mem::{CompressError, DecompressError, FlushCompress, FlushDecompress, Status};

/// Traits specifying the interface of the backends.
///
/// Sync + Send are added as a condition to ensure they are available
/// for the frontend.
pub trait Backend: Sync + Send {
    /// Returns the total number of bytes read and written by this backend.
    fn total_in(&self) -> u64;
    /// Returns the total number of bytes read and written by this backend.
    fn total_out(&self) -> u64;
}

///
/// Traits specifying the interface of the inflate backend. These are
/// used by the frontend to implement the various decoder types.
///
pub trait InflateBackend: Backend {
    /// Create the backend
    fn make(zlib_header: bool, window_bits: u8) -> Self;
    /// Options to decompress the input.
    fn decompress(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        flush: FlushDecompress,
    ) -> Result<Status, DecompressError>;
    /// Reset the backend to its initial state, optionally with a new zlib header and window bits.
    fn reset(&mut self, zlib_header: bool);
}

///
/// Traits specifying the interface of the deflate backends. These are
/// used by the frontend to implement the various encoder types.
///
pub trait DeflateBackend: Backend {
    /// Create the backend
    fn make(level: Compression, zlib_header: bool, window_bits: u8) -> Self;
    /// Options to compress the input.
    fn compress(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        flush: FlushCompress,
    ) -> Result<Status, CompressError>;
    /// Reset the backend to its initial state.
    fn reset(&mut self);
}

// Default to Rust implementation unless explicitly opted in to a different backend.
#[cfg(feature = "any_zlib")]
mod c;
#[cfg(feature = "any_zlib")]
pub use self::c::*;

#[cfg(all(not(feature = "any_zlib"), feature = "miniz_oxide"))]
mod rust;
#[cfg(all(not(feature = "any_zlib"), feature = "miniz_oxide"))]
pub use self::rust::*;

impl std::fmt::Debug for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
