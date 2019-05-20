// This file includes `bindings.rs` and relaxes constraints on syntax.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs"));
