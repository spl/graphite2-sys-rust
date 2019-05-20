extern crate cc;
#[cfg(feature = "static")]
extern crate cmake;
#[cfg(feature = "pkg-config")]
extern crate pkg_config;
#[cfg(feature = "vcpkg")]
extern crate vcpkg;

fn main() {

    // 1. Use pkg-config to find a shared library. (feature = "pkg-config")

    // 2. Use vcpkg to find a shared or static library. (feature = "vcpkg")

    // 3. Build a static library. (feature = "static")

    if use_pkg_config() || use_vcpkg() || build_static_lib() {
        return
    }

    // 4. Use an installed shared library with no package manager. (no-default-features)
    //
    // This is only done by disabling both of the features "pkg-config" and "static". This cannot
    // be disabled; otherwise, there's no library available.

    use_installed()
}

#[cfg(feature = "pkg-config")]
fn use_pkg_config() -> bool {
    println!("[build.rs] Trying pkg-config...");
    match pkg_config::probe_library("graphite2") {
        Ok(lib) => {
            println!("[build.rs] pkg-config succeeded: {:?}", lib);
            true
        }
        Err(err) => {
            println!("[build.rs] pkg-config failed: {}", err);
            false
        }
    }
}

#[cfg(not(feature = "pkg-config"))]
fn use_pkg_config() -> bool {
    false
}

#[cfg(feature = "vcpkg")]
fn use_vcpkg() -> bool {
    println!("[build.rs] Trying vcpkg...");
    match vcpkg::find_package("graphite2") {
        Ok(lib) => {
            println!("[build.rs] vcpkg succeeded: {:?}", lib);
            true
        }
        Err(err) => {
            println!("[build.rs] vcpkg failed: {}", err);
            false
        }
    }
}

#[cfg(not(feature = "vcpkg"))]
fn use_vcpkg() -> bool {
    false
}

#[cfg(feature = "static")]
fn build_static_lib() -> bool {
    println!("[build.rs] Compiling a static library...");

    // Create a `cmake` configuration for building in the `graphite2` directory.
    let mut cfg = cmake::Config::new("graphite2");

    // Set the `cmake` build profile based on the Rust profile.
    // https://doc.rust-lang.org/cargo/reference/manifest.html#the-profile-sections
    #[cfg(debug_assertions)]
    cfg.profile("Debug");
    #[cfg(not(debug_assertions))]
    cfg.profile("Release");

    // Disable shared libraries to build a static library
    cfg.define("BUILD_SHARED_LIBS", "OFF");

    // Avoid problem with -G "MinGW Makefiles" when `sh.exe` is in the $PATH.
    // https://stackoverflow.com/a/45104058/545794
    #[cfg(all(windows, target_env = "gnu"))]
    cfg.define("CMAKE_SH", "CMAKE_SH-NOTFOUND");

    // Build `graphite2` and install it in $OUT_DIR.
    let install_dir = cfg.build();

    // Use the installation directory to find the library for linking.
    println!("cargo:rustc-link-search=native={}", install_dir.join("lib").display());

    // Link statically.
    println!("cargo:rustc-link-lib=static=graphite2");

    // For MinGW:
    //   1. We must tell `rustc` to link with `libstdc++`.
    //   2. This must come after `static=graphite2`.
    #[cfg(all(windows, target_env = "gnu"))]
    println!("cargo:rustc-link-lib=stdc++");

    // Pass along the header file directory.
    println!("cargo:include={}", install_dir.join("include").display());

    println!("[build.rs] Compiling done.");
    true
}

#[cfg(not(feature = "static"))]
fn build_static_lib() -> bool {
    false
}

fn use_installed() {
    println!("[build.rs] Checking for a shared library...");

    // Use only the compiler. The alternative `.get_compiler().to_command()` includes a bunch of
    // flags that are superfluous to the purpose of just checking if this file builds.
    let mut cmd = std::process::Command::new(cc::Build::new().get_compiler().path());

    // Link to some installed `graphite2` shared library and build a simple executable.
    cmd.arg("-o").arg("/dev/null").arg("graphite2/tests/examples/simple.c").arg("-lgraphite2");
    println!("[build.rs] Command: {:?}", cmd);

    // Run the command and check the status.
    let status = cmd.status().expect("[build.rs] Command failed to execute");
    if status.success() {
        println!("[build.rs] Command succeeded.");
    } else {
        match status.code() {
            Some(code) => panic!("[build.rs] Command exited with status code: {}", code),
            None       => panic!("[build.rs] Command terminated by signal.")
        }
    }
}
