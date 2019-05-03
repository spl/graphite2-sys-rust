# Print commands and exit on any error
set -ex

# Install vcpkg.
git clone --depth 1 https://github.com/Microsoft/vcpkg.git $VCPKG_ROOT
cd $VCPKG_ROOT
./bootstrap-vcpkg.bat

# Install the library.
./vcpkg install graphite2

# Check that the library is installed.
./vcpkg list | grep graphite2
