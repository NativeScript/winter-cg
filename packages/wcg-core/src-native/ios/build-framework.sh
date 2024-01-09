#!/bin/sh

echo "Set exit on simple errors"
set -e
pushd packages/wcg-core/src-native/ios
rm -rf $(PWD)/dist

echo "Build for iphonesimulator"
xcodebuild \
    -project WinterCG.xcodeproj \
    -scheme WinterCG \
    -sdk iphonesimulator \
    -destination "generic/platform=iOS Simulator" \
    -configuration Release \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Build for iphoneos"
xcodebuild \
    -project WinterCG.xcodeproj \
    -scheme WinterCG \
    -sdk iphoneos \
    -destination "generic/platform=iOS" \
    -configuration Release \
    clean build \
    BUILD_DIR=$(PWD)/dist \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    SKIP_INSTALL=NO \
    BUILD_LIBRARY_FOR_DISTRIBUTION=YES

echo "Creating XCFramework"
xcodebuild \
    -create-xcframework \
    -framework $(PWD)/dist/Release-iphoneos/WinterCG.framework \
    -debug-symbols $(PWD)/dist/Release-iphoneos/WinterCG.framework.dSYM \
    -framework $(PWD)/dist/Release-iphonesimulator/WinterCG.framework \
    -debug-symbols $(PWD)/dist/Release-iphonesimulator/WinterCG.framework.dSYM \
    -output $(PWD)/dist/WinterCG.xcframework

popd

cp -r packages/wcg-core/src-native/ios/dist/WinterCG.xcframework packages/wcg-core/platforms/ios

rm -rf packages/wcg-core/src-native/ios/dist/WinterCG.xcframework