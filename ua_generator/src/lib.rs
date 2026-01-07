#![warn(missing_docs)]
//! Generate a random User-Agent to use.
//!
//! UA generator generates a randomized current User-Agent.
//!
//! # How to use UA_Generator
//!
//!
//! - **Generate** get a random UA to use.
//!   - [`spoof_ua`][]: ua/fn.spoof_ua.html
//!
//! # Basic usage
//!
//! First, you will need to add `ua_generator` to your `Cargo.toml`.
//!
//! Next, simply add the ua_generator::ua::spoof_ua()
//! to get your random agent.

/// Auto generated list of real Chrome Linux User Agents.
pub mod chrome_linux_ua_list;
/// Auto generated list of real Chrome Mac User Agents.
pub mod chrome_mac_ua_list;
/// Auto generated latest real chrome mobile user-agents.
pub mod chrome_mobile_ua_list;
/// Auto generated latest real chrome tablet user-agents.
pub mod chrome_tablet_ua_list;
/// Auto generated list of real Chrome User Agents.
pub mod chrome_ua_list;
/// Auto generated list of real Chrome Windows User Agents.
pub mod chrome_windows_ua_list;

/// Auto generated list of real Firefox Linux User Agents.
pub mod firefox_linux_ua_list;
/// Auto generated list of real Firefox Mac User Agents.
pub mod firefox_mac_ua_list;
/// Auto generated latest real Firefox mobile user-agents.
pub mod firefox_mobile_ua_list;
/// Auto generated latest real Firefox tablet user-agents.
pub mod firefox_tablet_ua_list;
/// Auto generated list of real Firefox User Agents.
pub mod firefox_ua_list;
/// Auto generated list of real Firefox Windows User Agents.
pub mod firefox_windows_ua_list;

/// Auto generated list of real Safari Mac User Agents.
pub mod safari_mac_ua_list;
/// Auto generated latest real Safari mobile user-agents.
pub mod safari_mobile_ua_list;
/// Auto generated latest real Safari tablet user-agents.
pub mod safari_tablet_ua_list;
/// Auto generated list of real Safari User Agents.
pub mod safari_ua_list;

/// User agent management.
pub mod ua;
/// Auto generated latest User Agents for windows, mac, linux, and android.
pub mod ua_list;

pub extern crate fastrand;
