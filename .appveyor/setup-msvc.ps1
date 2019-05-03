# This script performs the installation steps necessary for testing the `msvc`
# toolchain.
#
# In particular, it installs `graphite2` with `vcpkg`. Note that it does not do
# anything without the `vcpkg` feature.

# Exit on any error.
$ErrorActionPreference = 'Stop';

# Don't do anything for unexpected environment variables.
if ($env:TOOLCHAIN -ne 'msvc') { exit }
if ($env:FEATURES -ne 'vcpkg') { exit }

# Check for expected environment variables.
if (!(test-path env:VCPKG_DEFAULT_TRIPLET)) { throw 'Missing env var: VCPKG_DEFAULT_TRIPLET' }

# Print version.
vcpkg version

# Install `graphite2` and show that it is installed.
appveyor-retry vcpkg install graphite2
vcpkg list
