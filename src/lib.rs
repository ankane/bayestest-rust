#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "no_std", no_std)]
#![cfg_attr(feature = "nightly", feature(float_gamma))]

extern crate alloc;

mod binary;
mod count;
mod error;
mod math;

pub use binary::BinaryTest;
pub use count::CountTest;
pub use error::Error;
