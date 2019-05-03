# This script performs the installation steps necessary for testing the `gnu`
# toolchain.
#
# In particular, it fixes the PATH for CMake with MinGW and installs
# `graphite2` with `pkg-config`.

# Exit on any error.
$ErrorActionPreference = 'Stop';

# Don't do anything for unexpected environment variables.
if ($env:TOOLCHAIN -ne 'gnu') { exit }

# Check for expected environment variables.
if (!(test-path env:ARCH)) { throw 'Missing env var: ARCH' }

# `sh.exe` must not be in the path for CMake with "MinGW Makefiles" to work.
# This script finds its path and removes its directory from PATH.
#
# Sources:
# - https://gitlab.kitware.com/cmake/community/wikis/doc/cmake/platform_dependent_issues/MinGW-Compiler-Issues
# - https://help.appveyor.com/discussions/problems/3193

# Get the path of `sh.exe`.
$sh_path = (get-command sh.exe).path

# If the path exists, remove the directory containing `sh.exe` from PATH by
# splitting the entries, filtering, and joining the remaining entries.
if ($sh_path) {
  $sh_path = split-path $sh_path
  write-host "Removing from PATH: $sh_path"
  # Source: https://stackoverflow.com/a/54856614/545794
  $env:PATH = ($env:PATH.split(';') | where-object {$_ -ne $sh_path}) -join ';'
}

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

# Don't do anything for unexpected environment variables.
if ($env:FEATURES -ne 'pkg-config') { exit }

# Print version.
C:\msys64\usr\bin\pacman --version

# Create the `pacman` package cache directory to avoid a warning.
#
# Sources:
# - https://bbs.archlinux.org/viewtopic.php?id=9
# - https://github.com/open62541/open62541/issues/2068
C:\msys64\usr\bin\mkdir -p /var/cache/pacman/pkg

# Install `graphite2` and show that it is installed.
appveyor-retry C:\msys64\usr\bin\pacman --sync --sysupgrade --noconfirm "mingw-w64-$env:ARCH-graphite2"
C:\msys64\usr\bin\pacman --query --info "mingw-w64-$env:ARCH-graphite2"
