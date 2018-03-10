#!/bin/bash
set -e
# Runs against Cargo.toml in working directory.

# Script detects if build is triggered by git tag and sets it as Mainframer version, otherwise no-op.

if [ -z "$TRAVIS_TAG" ]; then
    echo "Non-tag build, using version from Cargo.toml."
else
    echo "Tag detected, overriding version in Cargo.toml."

    if ! [[ "$TRAVIS_TAG" == v* ]]; then
        echo "Git tag should start with 'v', ie 'v3.1.4'."
        exit 1
    fi

    NEW_VERSION=${TRAVIS_TAG#"v"}
    OLD_VERSION="3.0.0-dev"

    if [[ "$OSTYPE" == "linux-gnu" ]]; then
        sed --in-place "s/$OLD_VERSION/$NEW_VERSION/g" Cargo.toml
    elif [[ "$OSTYPE" == "darwin"* ]]; then # macOS.
        sed -i "" "s/$OLD_VERSION/$NEW_VERSION/g" Cargo.toml
    fi

    if ! grep --quiet "$NEW_VERSION" Cargo.toml; then
        echo "Couldn't override version, make sure Cargo.toml and ci/mainframer_version.sh are in sync."
        echo "Expected to replace Mainframer version to '$NEW_VERSION' in Cargo.toml."
        exit 1
    fi

    echo "Mainframer version was overridden to '$NEW_VERSION'."
fi

