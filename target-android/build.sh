#!/bin/bash
set -e #fail on first error

cargo clean
cargo apk build
