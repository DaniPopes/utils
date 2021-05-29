//! Pure Rust implementation of a big integer library designed for cryptography.
//!
//! # About
//! This library has been designed  from the ground-up for use in cryptographic
//! applications. It provides constant-time, `no_std`-friendly implementations
//! of modern formulas implemented using const generics.
//!
//! # `generic-array` interop
//! When the optional `generic-array` feature is enabled, this library provides
//! an [`ArrayEncoding`] trait which can be used to serialize/deserialize big
//! integer values as `GenericArray<u8, N>`.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_root_url = "https://docs.rs/crypto-bigint/0.0.0"
)]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod macros;

mod ops;
mod traits;
mod uint;

#[cfg(feature = "generic-array")]
mod array;

pub use crate::{
    traits::{NumBits, NumBytes},
    uint::*,
};

#[cfg(feature = "generic-array")]
pub use {
    self::array::{ArrayEncoding, ByteArray},
    generic_array::{self, typenum::consts},
};

/// Big integers are modeled as an array of smaller integers called "limbs".
#[cfg(target_pointer_width = "32")]
pub type Limb = u32;

/// Big integers are modeled as an array of smaller integers called "limbs".
#[cfg(target_pointer_width = "64")]
pub type Limb = u64;

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("this crate builds on 32-bit and 64-bit platforms only");

/// Number of bytes in a [`Limb`].
#[cfg(target_pointer_width = "32")]
pub const LIMB_BYTES: usize = 4;

/// Number of bytes in a [`Limb`].
#[cfg(target_pointer_width = "64")]
pub const LIMB_BYTES: usize = 8;