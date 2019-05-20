# This script installs the Rust components.

# Print each command before execution and throw an exception if a variable is
# referenced before being defined.
set-psdebug -trace 1 -strict

$rust = "$env:CHANNEL-$env:ARCH-pc-windows-$env:TOOLCHAIN"

# Download `rustup-init.exe`.
# Don't put it in the working directory because `cargo package` will complain.
appveyor-retry appveyor downloadfile 'https://win.rustup.rs/' -filename 'C:\rustup-init.exe' -timeout 60000

# Above: set timeout to 1 minute.
# More on downloading: https://www.appveyor.com/docs/how-to/download-file/

# Print version.
C:\rustup-init --version

# Install the toolchain.
C:\rustup-init -y --default-toolchain $rust 2>&1 | %{ "$_" }

# The last bit above is to suppress the NativeCommandError:
# https://stackoverflow.com/a/20950421/545794

# Update the PATH.
$env:PATH = $env:USERPROFILE + '\.cargo\bin;' + $env:PATH;

# Print versions.
rustc --version
cargo --version
