#!/bin/bash
cargo build --example simple
rm -r simple.app
rm simple.log
mkdir simple.app
mkdir simple.app/Contents
mkdir simple.app/Contents/MacOS
mkdir simple.app/Contents/Resources
cp target/debug/examples/simple simple.app/Contents/MacOS/simple-bin
cp bundle/Info.plist simple.app/Contents/
cp bundle/rust-logo.icns simple.app/Contents/Resources
codesign --force --sign $CERTIFICATE -o runtime --entitlements ./bundle/simple.app.xcent --timestamp\=none --generate-entitlement-der ./simple.app
touch simple.log
open simple.app --stdout ./simple.log --stderr ./simple.log
