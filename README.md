# emstr

`no_std` and alloc-free string building and encoding for embedded firmware in rust.

This crate provides an [`EncodeStr`]() trait for string encoding and implementations for basic types, as well as a [`write!`]() macro for building strings from encodable objects, roughly equivalent to `alloc::format!` except using pre-allocated buffers.

## Status

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/emstr.svg)](https://github.com/ryankurte/emstr)
[![Build Status](https://github.com/ryankurte/emstr/actions/workflows/rust.yml/badge.svg)](https://github.com/ryankurte/emstr/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/emstr.svg)](https://crates.io/crates/emstr)
[![Docs.rs](https://docs.rs/emstr/badge.svg)](https://docs.rs/emstr)
