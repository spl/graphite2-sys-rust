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

    // Build `graphite2` and install it in $OUT_DIR.
    let install_dir = cmake::Config::new("graphite2")
        .profile("Release")
        // Disable shared libraries to build a static library
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}", install_dir.join("lib").display());
    println!("cargo:rustc-link-lib=static=graphite2");
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
