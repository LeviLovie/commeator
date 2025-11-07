#!/usr/bin/env bash
set -e

DIST_DIR="dist/macos"
APP_DIR="${DIST_DIR}/Commeator.app"
BUILD_DIR="client/target/dx/commeator/release/macos"
CONFIG_DIR="config/macos"

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
rm -rf "${DIST_DIR}"
mkdir -p "${DIST_DIR}/"

cecho blue "Building the MacOS app..."

cecho blue "Building for arm64..."
cd client && dx build --macos --release --target aarch64-apple-darwin && cd ..
ARM64_EXEC="${DIST_DIR}/commeator-arm64"
mv "client/target/dx/commeator/release/macos/Commeator.app/Contents/MacOS/commeator" "$ARM64_EXEC"

cecho blue "Building for x86_64..."
cd client && dx build --macos --release --target x86_64-apple-darwin && cd ..
X64_EXEC="${DIST_DIR}/commeator-x64"
mv "client/target/dx/commeator/release/macos/Commeator.app/Contents/MacOS/commeator" "$X64_EXEC"

cp -r client/target/dx/commeator/release/macos/Commeator.app "${DIST_DIR}/"

cecho blue "Combining binaries into universal..."
lipo -create "${X64_EXEC}" "${ARM64_EXEC}" -output "${APP_DIR}/Contents/MacOS/commeator"

cecho blue "Generating manifests..."
rm "${DIST_DIR}/Commeator.app/Contents/Info.plist"
cp "${CONFIG_DIR}/Info.plist" "${APP_DIR}/Contents/"
cp "${CONFIG_DIR}/Commeator.entitlements" "${APP_DIR}/Contents/"
cp "${CONFIG_DIR}/embedded.provisionprofile" "${APP_DIR}/Contents/"

cecho blue "Generating icons..."
mkdir -p "${APP_DIR}/Contents/AppIcons.iconset"
sips -Z 16   config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_16x16.png"
sips -Z 32   config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_16x16@2x.png"
sips -Z 32   config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_32x32.png"
sips -Z 64   config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_32x32@2x.png"
sips -Z 128  config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_128x128.png"
sips -Z 256  config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_128x128@2x.png"
sips -Z 256  config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_256x256.png"
sips -Z 512  config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_256x256@2x.png"
sips -Z 512  config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_512x512.png"
sips -Z 1024 config/icon.png --out "${APP_DIR}/Contents/AppIcons.iconset/icon_512x512@2x.png"
iconutil -c icns "${APP_DIR}/Contents/AppIcons.iconset" -o "${APP_DIR}/Contents/Resources/AppIcons.icns"
rm -rf "${APP_DIR}/Contents/AppIcons.iconset"

xattr -cr "${DIST_DIR}/Commeator.app"

cecho blue "Signing the app..."
codesign --force --deep --options runtime \
    --sign "${SIGN}" \
    --entitlements "${CONFIG_DIR}/Commeator.entitlements" \
    "${DIST_DIR}/Commeator.app"

productbuild \
  --component ${DIST_DIR}/Commeator.app /Applications \
  --sign "${PKG_SIGN}" \
  dist/macos/Commeator.pkg
