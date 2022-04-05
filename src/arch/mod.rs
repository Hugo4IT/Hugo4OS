//! Code specific to architecture, mostly initialization sequences
//! 
//! ## Contributors
//! 
//! The multi-architecture implementation is kinda wack, but
//! I couldn't think of a better way.

// TODO: Add docs on adding a new architecture

#[cfg(target_arch = "x86_64")] pub mod _x86_64;