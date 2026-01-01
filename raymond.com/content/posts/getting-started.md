---
title: "Getting Started with Raymond"
date: "2026-01-01"
description: "The first steps in building an embedded Rust project"
---

# Hello, World!

This is the first entry in the Raymond dev log. Here's what we've accomplished so far:

## The Vision

Raymond is an embedded Rust project that aims to [describe your project goals here].

## Tech Stack

- **Framework**: Rust + Embassy
- **Hardware**: RP2040
- **Progress Site**: Next.js 16

## What's Next

Stay tuned for more updates as the journey continues!

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Raymond begins...
}
```
