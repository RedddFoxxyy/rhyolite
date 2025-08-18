#!/bin/bash

CURRENT_OS=$(uname -s)
echo "Running packaging tasks for Rhyolite..."
cd ..
cargo install cargo-packager --locked
cargo build --release
cargo packager --release --config Packager.toml

if [ "$CURRENT_OS" = "Linux" ]; then
  echo "Operating system is Linux. Running RPM and Flatpak builds..."
  cargo install cargo-generate-rpm
  strip -s target/release/Rhyolite
  cargo generate-rpm
  cd packaging/linux/flatpak || exit
  chmod +x build.sh
  ./build.sh
else
  echo "Operating system is '$CURRENT_OS', not Linux. Skipping RPM and Flatpak steps."
fi

echo "Script finished."