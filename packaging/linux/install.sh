#!/bin/bash

set -e

APP_NAME="Rhyolite"
BINARY_URL="https://github.com/rhyolite-org/rhyolite/releases/download/v0.1.5/Rhyolite_0.1.5_x86_64_linux_binary"
ICON_URL="https://raw.githubusercontent.com/rhyolite-org/Rhyolite/master/src-tauri/icons/icon.png"
INSTALL_DIR="$HOME/.local/bin"
DESKTOP_FILE_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons"

echo "Installing $APP_NAME..."

# Download the binary
curl -L $BINARY_URL -o /tmp/$APP_NAME
chmod +x /tmp/$APP_NAME

# Move the binary to the installation directory
mkdir -p $INSTALL_DIR
mv /tmp/$APP_NAME $INSTALL_DIR/$APP_NAME

# Download and install the icon
mkdir -p $ICON_DIR
curl -L $ICON_URL -o $ICON_DIR/$APP_NAME.png
# mv /tmp/$APP_NAME.png $ICON_DIR/$APP_NAME.png

# Create a .desktop file
mkdir -p $DESKTOP_FILE_DIR
DESKTOP_FILE=$DESKTOP_FILE_DIR/$APP_NAME.desktop

cat <<EOF > $DESKTOP_FILE
[Desktop Entry]
Type=Application
Name=$APP_NAME
Comment=A simple text editor written in Rust using Tauri.
Exec=$INSTALL_DIR/$APP_NAME
Icon=$ICON_DIR/$APP_NAME.png
Terminal=false
Categories=Utility;TextEditor;
Author=Suyog Tandel
EOF

chmod +x $DESKTOP_FILE

echo "$APP_NAME installed successfully!"

# Refresh desktop database
if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$HOME/.local/share/applications"
fi

echo "You can now find $APP_NAME in your application menu."
