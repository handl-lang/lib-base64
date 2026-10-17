//! `base64` — Handl stdlib codec (`src/index.hnl`).
//!
//! Transitional: the ops are Rust until plan `stdlib-lib-migration` Phase 7
//! rewrites them as `ffi.call`.

pub const BASE64_HNL: &str = include_str!("index.hnl");

pub mod ops;

pub use ops::{BASE64_OPS, base64_op_registry};
