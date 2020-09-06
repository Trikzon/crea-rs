#!/usr/bin/env bash

# set the sdk version to use
# 21 is the minimum supported version by omni-rs
min_ver=21

# verifies that you have the proper targets installed then builds app for the targets
cargo ndk --target aarch64-linux-android --android-platform ${min_ver} -- build --release
cargo ndk --target armv7-linux-androideabi --android-platform ${min_ver} -- build --release
cargo ndk --target i686-linux-android --android-platform ${min_ver} -- build --release
cargo ndk --target x86_64-linux-android --android-platform ${min_ver} -- build --release

jniLibs=android/app/src/main/jniLibs
# TODO: get name from file so it's easy for a user to change it
libName=libapp.so

# clean old builds
rm -rf ${jniLibs}

# re-make directories
mkdir ${jniLibs}
mkdir ${jniLibs}/arm64-v8a
mkdir ${jniLibs}/armeabi-v7a
mkdir ${jniLibs}/x86
mkdir ${jniLibs}/x86_64

# moves the built app libraries to the android project
cp target/aarch64-linux-android/release/${libName} ${jniLibs}/arm64-v8a/${libName}
cp target/armv7-linux-androideabi/release/${libName} ${jniLibs}/armeabi-v7a/${libName}
cp target/i686-linux-android/release/${libName} ${jniLibs}/x86/${libName}
cp target/x86_64-linux-android/release/${libName} ${jniLibs}/x86_64/${libName}

# verifies that you have the proper targets installed then builds engine for the targets
# TODO: support for pre-compiled engine
cd engine/
cargo ndk --target aarch64-linux-android --android-platform ${min_ver} -- build --release
cargo ndk --target armv7-linux-androideabi --android-platform ${min_ver} -- build --release
cargo ndk --target i686-linux-android --android-platform ${min_ver} -- build --release
cargo ndk --target x86_64-linux-android --android-platform ${min_ver} -- build --release

jniLibs=../android/app/src/main/jniLibs
# TODO: get name from file so it's easy for a user to change it
libName=libengine.so

# moves the built app libraries to the android project
cp target/aarch64-linux-android/release/${libName} ${jniLibs}/arm64-v8a/${libName}
cp target/armv7-linux-androideabi/release/${libName} ${jniLibs}/armeabi-v7a/${libName}
cp target/i686-linux-android/release/${libName} ${jniLibs}/x86/${libName}
cp target/x86_64-linux-android/release/${libName} ${jniLibs}/x86_64/${libName}