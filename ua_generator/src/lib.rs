#![warn(missing_docs)]
//! Generate a random User-Agent to use.
//!
//! UA generator generates a randomized current User-Agent.
//!
//! # How to use UA_Generator
//!
//!
//! - **Generate** get a random UA to use.
//!   - [`spoof_ua`]: https://docs.rs/ua_generator/latest/ua_generator/ua/fn.spoof_ua.html
//!
//!
//! # Basic usage
//!
//! First, you will need to add `ua_generator` to your `Cargo.toml`.
//!
//! Next, simply add the ua_generator::ua::spoof_ua()
//! to get your random agent.

/// User agent management.
pub mod ua;
/// Auto generated latest User Agents for windows, mac, linux, and android.
pub mod ua_list;

extern crate fastrand;
