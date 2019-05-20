extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Missing env var: CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("graphite2-sys");
    let header_dir = path.to_str().expect("Invalid Unicode in env var: CARGO_MANIFEST_DIR");

    // Generate the bindings from the `graphite2` header files.
    let bindings = bindgen::Builder::default()
        .header(format!("{}/graphite2/include/graphite2/Segment.h", header_dir))
        // Generate Rust types for only these C types.
        .whitelist_type("gr_.*")
        // This type introduces Rust features that cause `ctest` to panic.
        .opaque_type("gr_faceinfo")
        // Constified `enum`s cause problems for `ctest`.
        .rustified_enum("gr_faceinfo_gr_space_contextuals")
        .rustified_enum("gr_encform")
        .rustified_enum("gr_face_options")
        .rustified_enum("gr_break_weight")
        .rustified_enum("gr_justFlags")
        .rustified_enum("gr_attrCode")
        .rustified_enum("gr_bidirtl")
        .generate()
        .expect("Can't generate bindings");

    path.push("src/bindings.rs");

    // Write the bindings to the `graphite2-sys` `lib.rs` file.
    bindings
        .write_to_file(&path)
        .expect(&format!("Can't write to '{}'", path.display()));
}
