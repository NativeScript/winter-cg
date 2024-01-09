#! /bin/bash
LIB_NAME="wcgcore"
FRAMEWORK="WCGCore"

cp -r ./crates/libs/ios/include packages/wcg-core/src-native/ios/WinterCG
cp -r target/$FRAMEWORK.xcframework packages/wcg-core/src-native/ios/libs
rm -rf target/$FRAMEWORK.xcframework
rm target/simulator_fat/lib$LIB_NAME.dylib