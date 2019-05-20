extern crate ctest;
extern crate pkg_config;
extern crate vcpkg;

use std::env;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    // Get the include paths from `graphite2-sys` (static build), `pkg-config`, or `vcpkg`.
    if let Some(path) = &env::var_os("DEP_GRAPHITE2_INCLUDE") {
        cfg.include(path);
    } else if let Ok(lib) = pkg_config::probe_library("graphite2") {
        for path in lib.include_paths {
            cfg.include(path);
        }
    } else if let Ok(lib) = vcpkg::find_package("graphite2") {
        for path in lib.include_paths {
            cfg.include(path);
        }
    }

    // Include the main C header file.
    cfg.header("graphite2/Segment.h");

    // Skip the following `struct`s.
    cfg.skip_struct(|s| {
        // Empty `struct`s
        s == "gr_face" ||
        s == "gr_font" ||
        s == "gr_feature_ref" ||
        s == "gr_feature_val" ||
        s == "gr_char_info" ||
        s == "gr_segment" ||
        s == "gr_slot" ||
        // Created with bindgen::Builder::opaque_type
        s == "gr_faceinfo"
    });

    // Generate the tests.
    //
    // NOTE: Use the bindgen-generated `bindings.rs` and _not_ `lib.rs` to avoid a `ctest` panic:
    // https://github.com/gnzlbg/ctest/issues/23
    cfg.generate("../graphite2-sys/src/bindings.rs", "all.rs");
}
