cd ..
cargo install cargo-generate-rpm
cargo install cargo-packager --locked
cargo build --release
cargo packager --release --config Packager.toml
strip -s target/release/Rhyolite
cargo generate-rpm
cd packaging/linux/flatpak || exit
chmod +x build.sh
./build.sh