#!/bin/bash
set -e

# You can run it from any directory.
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$DIR/.."

# Create separate build dir and copy project to it.
# That allows us safely modify project files during the build which is helpful for local development.
export BUILD_DIR="$PROJECT_DIR/build"

# Clean previous build state.
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Copy project to the build dir.
rsync --archive --exclude "/.idea" --exclude "/build" "$PROJECT_DIR/" "$BUILD_DIR/"
