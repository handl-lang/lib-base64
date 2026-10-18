//! `base64` — base64 codec. Impl is now pure-handl (`src/encode.hnl`,
//! `src/decode.hnl`); the Rust handls below remain until consumers switch
//! to the `.hnl` module (prune-after-parity).

pub const BASE64_HNL: &str = include_str!("index.hnl");

pub mod codec;
pub mod ops;

pub use codec::{register, Base64};
pub use ops::{base64_op_registry, BASE64_OPS};
