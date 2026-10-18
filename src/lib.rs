//! `base64` — base64 codec. Impl is now pure-handl (`src/encode.hnl`,
//! `src/decode.hnl`); the Rust handls below remain until consumers switch
//! to the `.hnl` module (prune-after-parity).

pub const BASE64_HNL: &str = include_str!("index.hnl");

pub mod ops;

pub use ops::{base64_op_registry, BASE64_OPS};

/// `.hnl` package source set — index + sibling files (wired into kits via
/// `handl_module_from_sources`).
pub const BASE64_FUNCS: &[&str] = &["encode","decode","codec"];
pub const BASE64_SOURCES: &[(&str, &str, &[&str])] = &[
    ("encode", include_str!("encode.hnl"), &["encode"]),
    ("decode", include_str!("decode.hnl"), &["decode"]),
];

/// Flattened single-source `.hnl` (for `stdlib::MODULES` table rows).
pub const BASE64_FLAT_HNL: &str = include_str!("index_flat.hnl");
