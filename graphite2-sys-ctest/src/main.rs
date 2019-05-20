// #![allow(bad_style, improper_ctypes)]

extern crate graphite2_sys;

use graphite2_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
