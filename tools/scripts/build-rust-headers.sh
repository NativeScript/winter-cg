#!/bin/bash

cbindgen --config crates/libs/c/cbindgen.toml  crates/libs/c/src/lib.rs -l c > crates/libs/c/include/wcg_core.h
cbindgen --config crates/libs/c/cbindgen.toml  crates/libs/c/src/lib.rs -l c > packages/core/platforms/ios/src/cpp/include/wcg_core.h
cbindgen --config crates/libs/c/cbindgen.toml  crates/libs/c/src/lib.rs -l c > packages/wcg-core/src-native/android/wcg-core/src/main/cpp/include/wcg_core.h
