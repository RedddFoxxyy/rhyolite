#!/bin/bash

arch=$(uname -m)
bundle_name="Rhyolite-${arch}.flatpak"

# cd ../../../
# cargo build --release

# cd packaging/linux/flatpak

flatpak-builder --force-clean --repo=repo build-dir io.github.redddfoxxyy.rhyolite.yml
flatpak build-bundle repo "${bundle_name}" io.github.redddfoxxyy.rhyolite
