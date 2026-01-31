//! Standalone library for testing
//! 
//! This library target allows testing code on the host platform
//! while the binary target remains no_std for embedded deployment

#![cfg_attr(not(test), no_std)]

pub mod game;
pub mod animator;
