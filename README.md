# Shared Resources 

[![CI](https://github.com/AlexiWolf/shared_resources/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/shared_resources/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/shared_resources)](https://github.com/AlexiWolf/shared_resources#license)
[![Crates.io](https://img.shields.io/crates/v/shared_resources)](https://crates.io/crates/shared_resources)

This crate provides a shared resources container which is thread-safe,
and lock-free.

The `Resources` struct, is a container of `Resource` objects.  Resources are 
inserted at run-time.  The container stores up to 1 instance of each type.  
Stored resources can be accessed by the rest of the system through
an immutable reference.  Borrowing rules are checked at run-time.

Thread-safe access is provided by the `ResourcesSync` struct.  It's similar to
the `Resources` struct, except it only allows access to thread-safe resources,
and can, itself, be sent to other threads.

The design is based heavily on the
[`Resources`](https://docs.rs/legion/latest/legion/struct.Resources.html) 
struct found in [Legion](https://crates.io/crates/legion), with the goal of
making it better-suited for general use-cases.

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

