#! /bin/bash
LIB_NAME="wcgcore"
FRAMEWORK="WinterCG"

cp -r ./crates/libs/ios/include packages/wcg-core/platforms/ios/src/cpp
cp -r target/$FRAMEWORK.xcframework packages/wcg-core/platforms/ios
rm -rf target/$FRAMEWORK.xcframework
rm target/simulator_fat/lib$LIB_NAME.dylib