#!/usr/bin/env bash
set -e

cecho() {
    local color=$1
    local message=$2
    local reset="\033[0m"
    case "$color" in
        red)    color="\033[0;31m" ;;
        green)  color="\033[0;32m" ;;
        yellow) color="\033[0;33m" ;;
        blue)   color="\033[0;34m" ;;
        magenta)color="\033[0;35m" ;;
        cyan)   color="\033[0;36m" ;;
        *)      color="\033[0m" ;;
    esac
    echo -e "${color}${message}${reset}"
}

echo red "Cleaning up the old build..."
rm -rf dist/ios

cd client

cecho blue "Building the iOS app..."

unset CC CXX CFLAGS CXXFLAGS SDKROOT LIBRARY_PATH CPATH PKG_CONFIG_PATH

export REAL_CLANG="$(xcrun --sdk iphoneos --find clang)"
export REAL_CLANGXX="$(xcrun --sdk iphoneos --find clang++)"

export SDKROOT="$(xcrun --sdk iphoneos --show-sdk-path)"
export IPHONEOS_DEPLOYMENT_TARGET=14.0

export CC="$REAL_CLANG"
export CXX="$REAL_CLANGXX"

unset NIX_CFLAGS_COMPILE
unset NIX_LDFLAGS
unset NIX_CC
unset NIX_CC_WRAPPER_TARGET_HOST_${target//-/_}

export RUSTFLAGS="-C linker=$REAL_CLANG -C link-arg=-isysroot$SDKROOT -C link-arg=-miphoneos-version-min=16.0"

dx build --ios --release --target aarch64-apple-ios

cd ..

mkdir -p dist/ios/
cp -r client/target/dx/commeator/release/ios/Commeator.app dist/ios/
rm dist/ios/Commeator.app/Info.plist
cp config/ios/Info.plist dist/ios/Commeator.app/
cp config/ios/embedded.mobileprovision dist/ios/Commeator.app/
cp config/ios/Commeator.entitlements dist/ios/

cd dist/ios/

mkdir -p Commeator.app/AppIcons
sips -Z 20   ../../config/icon.png --out Commeator.app/AppIcons/Icon-20.png
sips -Z 29   ../../config/icon.png --out Commeator.app/AppIcons/Icon-29.png
sips -Z 40   ../../config/icon.png --out Commeator.app/AppIcons/Icon-40.png
sips -Z 58   ../../config/icon.png --out Commeator.app/AppIcons/Icon-58.png
sips -Z 60   ../../config/icon.png --out Commeator.app/AppIcons/Icon-60.png
sips -Z 76   ../../config/icon.png --out Commeator.app/AppIcons/Icon-76.png
sips -Z 80   ../../config/icon.png --out Commeator.app/AppIcons/Icon-80.png
sips -Z 87   ../../config/icon.png --out Commeator.app/AppIcons/Icon-87.png
sips -Z 120  ../../config/icon.png --out Commeator.app/AppIcons/Icon-120.png
sips -Z 152  ../../config/icon.png --out Commeator.app/AppIcons/Icon-152.png
sips -Z 167  ../../config/icon.png --out Commeator.app/AppIcons/Icon-167.png
sips -Z 180  ../../config/icon.png --out Commeator.app/AppIcons/Icon-180.png
sips -Z 1024 ../../config/icon.png --out Commeator.app/AppIcons/Icon-1024.png

cecho blue "Signing the app..."
codesign --force --deep --options runtime \
    --entitlements "../../config/ios/Commeator.entitlements" \
    --sign "$SIGN" \
    Commeator.app

mkdir Payload
mv Commeator.app Payload/Commeator.app
zip -r Commeator.ipa Payload
