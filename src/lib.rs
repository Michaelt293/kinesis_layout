//! # kinesis_layout
//!
//! This project allows keyboard layouts for the Kinesis Advantage 2 to be generated
//! programmatically using Rust. Support for remapping keys and macros is provided.

extern crate either;
#[macro_use]
extern crate maplit;

pub mod configure;
pub mod keys;
pub mod layout;
pub mod macros;
