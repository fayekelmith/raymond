// Shared, hardware-agnostic contracts and utilities for Raymond subsystems.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod common;
pub mod diagnostics;
pub mod display;
pub mod motion;
pub mod prelude;
pub mod protocol;
pub mod sensors;
