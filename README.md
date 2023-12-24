# Shared Resources 

[![CI](https://github.com/AlexiWolf/shared_resources/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/shared_resources/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/shared_resources)](https://github.com/AlexiWolf/shared_resources#license)
[![Crates.io](https://img.shields.io/crates/v/shared_resources)](https://crates.io/crates/shared_resources)

This crate provides a `Resources` struct, which stores a collection of
shared `Resoruce` types added in at run-time.  The store contains up to 1 
of each type.  Borrowing rules are checked at run-time.

The resources container has a thread-safe handle, which provides safe, 
concurrent access to shared resources.  The API is lock-free.

The design is based heavily on the `Resources` struct found in 
[Legion](https://crates.io/crates/legion).  It also takes ideas from
the [Resources](https://crates.io/crates/resources) crate, and combines them
into a library that's stand-alone, and generally more well-behaved for general
usage.

## Status

Shared Resources is mostly complete, but is not quite ready for release yet.
Things may still change until release 1.0. 

# License

Shared Resources is licensed under either:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

At your option.

Unless you explicitly state otherwise, any contribution intentionally 
submitted for inclusion in the work by you, as defined in the Apache-2.0 
license, shall be dual licensed as above, without additional terms or 
conditions.

