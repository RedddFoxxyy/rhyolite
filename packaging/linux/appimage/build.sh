#!/bin/bash

arch=$(uname -m)
if [[ "$arch" == x86_64 ]]; then
  curl -L -O https://github.com/linuxdeploy/linuxdeploy/releases/latest/download/linuxdeploy-x86_64.AppImage
else
  curl -L -O https://github.com/linuxdeploy/linuxdeploy/releases/latest/download/linuxdeploy-aarch64.AppImage
fi

chmod +x linuxdeploy-x86_64.AppImage

cd ../../../
cargo build --release

cd packaging/linux/appimage || exit
./linuxdeploy-x86_64.AppImage --appdir AppDir --executable ../../../target/release/Rhyolite --output appimage --desktop-file ../Rhyolite.desktop --icon-file ../Rhyolite.svg