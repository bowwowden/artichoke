//! Build hashers and hash values

/// A trait for creating instances of RandomState
pub trait BuildHasher {
    /// Concrete error type for errors encountered when outputting hash errors.
    type Error;
    
    /// Concrete build hasher type.
    type BuildHasher: core::hash::BuildHasher;
    /// Build a RandomState hasher
    fn build_hasher(&mut self) -> Result<&B, Self::Error>;
}
