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
rm -rf dist/macos

cd client

cecho blue "Building the MacOS app..."
dx build --macos --release

cd ..

mkdir -p dist/macos/
cp -r client/target/dx/commeator/release/macos/Commeator.app dist/macos/
rm dist/macos/Commeator.app/Contents/Info.plist
cp config/macos/Info.plist dist/macos/Commeator.app/Contents/Info.plist
cp config/macos/Commeator.entitlements dist/macos/

cd dist/macos/

cecho blue "Signing the app..."
codesign --force --deep --options runtime \
    --entitlements "../../config/macos/Commeator.entitlements" \
    --sign "$SIGN" \
    Commeator.app

cecho blue "Notorizing the app with Apple..."
zip -r Commeator.zip Commeator.app
xcrun notarytool submit Commeator.zip \
    --apple-id "$ICLOUD" \
    --team-id "$TEAMID" \
    --password "$PASS" \
    --wait

cecho blue "Stapling the notarization ticket to the app..."
xcrun stapler staple Commeator.app

echo ""
cecho yellow "Verify the notarization status:"
echo ""
spctl --assess --verbose Commeator.app

echo ""
read -r -p "$(cecho yellow 'Accepted? [y/N] ')" response
case "$response" in
    [yY][eE][sS]|[yY])
        echo ""
        true
        ;;
    *)
        echo "Notorization did not succeed :("
        exit 1
        ;;
esac

cecho blue "Creating the DMG installer..."
create-dmg \
    --volname "Commeator Installer" \
    --window-pos 200 120 \
    --window-size 600 400 \
    --icon-size 100 \
    --icon Commeator.app 175 120 \
    --hide-extension Commeator.app \
    --app-drop-link 425 120 \
    Commeator.dmg \
    Commeator.app

cecho blue "Signing the DMG installer..."
codesign --sign "$SIGN" --timestamp Commeator.dmg

cecho blue "Notorizing the DMG installer with Apple..."
xcrun notarytool submit Commeator.dmg \
    --apple-id "$ICLOUD" \
    --team-id "$TEAMID" \
    --password "$PASS" \
    --wait

cecho blue "Stapling the notarization ticket to the DMG installer..."
xcrun stapler staple Commeator.dmg

cecho green "Complete! at $(pwd)/Commeator.dmg"
