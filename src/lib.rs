#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(float_gamma))]

mod binary;
mod count;
mod math;

pub use binary::*;
pub use count::*;
