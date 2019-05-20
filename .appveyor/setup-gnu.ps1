# This script performs the installation steps necessary for testing the `gnu`
# toolchain.
#
# In particular, it fixes the PATH for CMake with MinGW and installs
# `graphite2` with `pkg-config`.

# Print each command before execution and throw an exception if a variable is
# referenced before being defined.
set-psdebug -trace 1 -strict

# Exit on any error.
$ErrorActionPreference = 'Stop';

# Don't do anything for unexpected environment variables.
if ($env:TOOLCHAIN -ne 'gnu') { exit }

# Get the number of bits from the ARCH.
if ($env:ARCH -eq 'i686') {
  $bits = '32'
} elseif ($env:ARCH -eq 'x86_64') {
  $bits = '64'
} else {
  throw "Unexpected ARCH: $env:ARCH"
}

# Add the MingW tools (e.g. GCC) to PATH for use by CMake.
$env:PATH = 'C:\msys64\mingw' + $bits + '\bin;' + $env:PATH

# Print path and version.
C:\msys64\usr\bin\which gcc
gcc --version

# Don't do anything for unexpected environment variables.
if ($env:FEATURES -ne 'pkg-config') { exit }

# Print path.
C:\msys64\usr\bin\which pkg-config

# Print version.
C:\msys64\usr\bin\pacman --version

# Create the `pacman` package cache directory to avoid a warning.
#
# Sources:
# - https://bbs.archlinux.org/viewtopic.php?id=9
# - https://github.com/open62541/open62541/issues/2068
C:\msys64\usr\bin\mkdir -p /var/cache/pacman/pkg

# Install `graphite2` and show that it is installed.
appveyor-retry C:\msys64\usr\bin\pacman --sync --sysupgrade --needed --noconfirm "mingw-w64-$env:ARCH-graphite2"
C:\msys64\usr\bin\pacman --query --info "mingw-w64-$env:ARCH-graphite2"
pkg-config --libs --cflags graphite2
