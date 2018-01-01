//! Rust library wrapping the [Shadertoy REST API](http://shadertoy.com/api) to be able to easily search through and download Shadertoy assets.

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

extern crate reqwest;
extern crate serde_json;

mod types;
mod client;
mod errors;

pub use types::*;
pub use client::*;
pub use errors::*;
