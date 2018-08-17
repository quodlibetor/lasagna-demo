#![feature(rust_2018_preview, nll)]
#![warn(rust_2018_compatibility, rust_2018_idioms)]

pub use remote_cache::{RemoteCache, IntoCache, RemoteInsert};

mod remote_cache;
