#!/bin/sh

VERSION=${VERSION:-stable-2.7.1}
INSTALLROOT=${INSTALLROOT:-"${HOME}/.snyfy"}

OS=$(uname -s)
arch=$(uname -m)

happyexit() {
  echo ""
  echo "Add the the CLI to your path with:"
  echo ""
  echo "  export PATH=\$PATH:${INSTALLROOT}/bin"
  echo ""
  echo "Looking for more? Visit https://github.com/mayankshah1607/snyfy"
  echo ""
  exit 0
}

if [ "$OS" != "Linux" ]; then
    echo "Currently, only Linux is supporteed"
    exit 1
fi

url="https://github.com/mayankshah1607/snyfy/releases/download/stable-1.0/snyfy"

(
    mkdir -p "${INSTALLROOT}/bin"
    cd "${INSTALLROOT}/bin"
    echo "Downloading binary"
    curl -fLO "${url}"
    chmod +x "snyfy"
    echo "Done! Setting things up for you..."
    happyexit
)
