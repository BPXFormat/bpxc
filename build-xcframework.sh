#!/bin/sh

cargo build --release
cargo build --release --target x86_64-apple-ios
cargo build --release --target aarch64-apple-ios

xcodebuild -create-xcframework -library target/release/libbpxc.a -headers include -library target/x86_64-apple-ios/release/libbpxc.a -headers include -library target/aarch64-apple-ios/release/libbpxc.a -headers include -output bpxc.xcframework
