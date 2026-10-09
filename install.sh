#!/bin/bash
# Builds Quickwarden (Tauri), installs to /Applications and starts it at login.
set -euo pipefail
cd "$(dirname "$0")"

ID="Quick Access Local Signing"
APP="/Applications/Quickwarden.app"
BINARY="$APP/Contents/MacOS/quick-access"
BIN="$HOME/.local/bin"; PLIST="$HOME/Library/LaunchAgents/local.quick-access.plist"
mkdir -p "$BIN" "$HOME/Library/LaunchAgents"

# Stable identity: keeps the Touch ID item's Keychain ACL valid across builds
# (ad-hoc signatures change on every build).
if ! security find-identity -p codesigning | grep -q "$ID"; then
  T=$(mktemp -d); trap 'rm -rf "$T"' EXIT  # the private key never stays on disk
  cat > "$T/c.cnf" <<CNF
[req]
distinguished_name=dn
x509_extensions=ext
prompt=no
[dn]
CN=$ID
[ext]
basicConstraints=critical,CA:false
keyUsage=critical,digitalSignature
extendedKeyUsage=critical,codeSigning
CNF
  /usr/bin/openssl req -x509 -newkey rsa:2048 -nodes -keyout "$T/k.pem" -out "$T/c.pem" -days 3650 -config "$T/c.cnf"
  P=$(/usr/bin/openssl rand -hex 16)
  /usr/bin/openssl pkcs12 -export -inkey "$T/k.pem" -in "$T/c.pem" -out "$T/i.p12" -passout "pass:$P"
  security import "$T/i.p12" -k "$HOME/Library/Keychains/login.keychain-db" -P "$P" -T /usr/bin/codesign
fi

(cd quick-access/src-tauri && cargo tauri build --bundles app)

# bootout before pkill: with KeepAlive launchd would relaunch the old version.
launchctl bootout "gui/$UID/local.quick-access" 2>/dev/null || true
pkill -x quick-access || true
# Don't re-sign after copying.
rm -rf "$APP" "/Applications/Quick Access.app"  # old name
ditto "quick-access/src-tauri/target/release/bundle/macos/Quickwarden.app" "$APP"

# rbw pinentry (absolute path: rbw doesn't expand ~)
cat > "$BIN/qa-pinentry" <<PE
#!/bin/sh
exec "$BINARY" --pinentry "\$@"
PE
chmod +x "$BIN/qa-pinentry"
rbw config set pinentry "$BIN/qa-pinentry"

cat > "$PLIST" <<PL
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>Label</key><string>local.quick-access</string>
  <key>ProgramArguments</key><array><string>$BINARY</string></array>
  <key>RunAtLoad</key><true/><key>KeepAlive</key><true/>
  <key>StandardErrorPath</key><string>/tmp/quick-access.log</string>
</dict></plist>
PL
launchctl bootstrap "gui/$UID" "$PLIST"

if ! security find-generic-password -s local.quick-access -a rbw-master-password >/dev/null 2>&1; then
  echo "To enable Touch ID run: \"$BINARY\" --enroll"
fi
echo "Installed. ⇧⌘Space opens Quickwarden."
