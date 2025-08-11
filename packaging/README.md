# App Packaging Commands:

## Linux:

- `cargo build --release`
- `cd target/release`
- `linuxdeploy.appimage --appdir AppDir --executable ../target/release/Rhyolite --output appimage --desktop-file ./linux/Rhyolite.desktop --icon-file ./linux/Rhyolite.svg`