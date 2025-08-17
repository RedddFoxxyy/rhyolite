#!/bin/bash

arch=$(uname -m)
bundle_name="Rhyolite-${arch}.flatpak"

flatpak-builder --force-clean --repo=repo build-dir io.github.redddfoxxyy.rhyolite.yml
flatpak build-bundle repo "${bundle_name}" io.github.redddfoxxyy.rhyolite
