#!/bin/bash

set -e

# Detect operating system
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

# Detect architecture
ARCH=$(uname -m)

# Map architecture to Rust target triples
case "$ARCH" in
    "x86_64")
        ARCH="x86_64"
        ;;
    "arm64" | "aarch64")
        ARCH="arm64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Define target triple
case "$OS" in
    "linux")
        TARGET="$ARCH-unknown-linux-gnu"
        INSTALL_DIR="/usr/local/bin"
        ;;
    "darwin")
        TARGET="$ARCH-apple-darwin"
        INSTALL_DIR="/usr/local/bin"
        ;;
    "msys"* | "cygwin"* | "mingw"*)
        TARGET="$ARCH-pc-windows-msvc"
        INSTALL_DIR="C:/tools" # Adjust as needed or make this configurable
        ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

# Define download URL
VERSION="v0.1.9" # Replace with the correct version
FILENAME="dockpack-${VERSION}-${TARGET}.zip"
DOWNLOAD_URL="https://github.com/maxwellflitton/dockpack/releases/download/v/$FILENAME" # Replace with actual download URL
# Download the zip file
echo "Downloading $FILENAME..."
curl -LO "$DOWNLOAD_URL"

# Unzip the downloaded file
echo "Unzipping $FILENAME..."
unzip -o "$FILENAME"

# Remove existing dockpack binary in the target install directory if it exists
if [ -f "$INSTALL_DIR/dockpack" ]; then
    echo "Removing existing dockpack binary..."
    sudo rm -f "$INSTALL_DIR/dockpack"
fi

# Move the new dockpack binary to the install directory
echo "Installing dockpack binary..."
sudo mv dockpack "$INSTALL_DIR/dockpack"
sudo chmod +x "$INSTALL_DIR/dockpack"

# Clean up downloaded files
echo "Cleaning up..."
rm -f README.md "$FILENAME"

# Completion message
echo "Dockpack has been installed successfully in $INSTALL_DIR!"
