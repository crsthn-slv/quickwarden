# Quickwarden

macOS quick access panel (1Password-style) for Bitwarden/Vaultwarden vaults via [rbw](https://github.com/doy/rbw), with Touch ID unlock.

> Not affiliated with Bitwarden Inc. "Bitwarden" is a trademark of Bitwarden Inc.

## What it is

Quickwarden is a search panel for macOS. It runs alongside the official Bitwarden.app, not as a fork. It reads the vault through `rbw` / `rbw-agent` against the same Vaultwarden (or Bitwarden Cloud) server.

| Aspect | Value |
|---|---|
| Purpose | Search, copy, open URL |
| Editing | No. Editing stays in Bitwarden.app |
| Vault | `rbw` / `rbw-agent`, same server |
| Secrets | Never reach the webview/JS |
| Open shortcut | `⇧⌘Space` |
| Stack | Tauri 2 (Rust) + static HTML/JS, no bundler |

The app lives in [`quick-access/`](quick-access/). Install script is [`install.sh`](install.sh).

## Requirements

- macOS (Apple Silicon assumed)
- [rbw](https://github.com/doy/rbw) and `rbw-agent`, logged in to your server
- pinentry-mac: `brew install pinentry-mac`
- Rust, then the Tauri CLI: `cargo install tauri-cli --version "^2" --locked`

## Install

```sh
./install.sh
"/Applications/Quickwarden.app/Contents/MacOS/quick-access" --enroll
```

| Step | What it does |
|---|---|
| `./install.sh` | Builds, signs with the local identity `Quick Access Local Signing`, copies to `/Applications`, installs `~/.local/bin/qa-pinentry`, runs `rbw config set pinentry`, creates the LaunchAgent `local.quick-access` |
| `--enroll` | Stores the master password in the Keychain. pinentry-mac asks for it twice. Run once |

Changed your master password? Run `--enroll` again.

## Shortcuts

| Action | Shortcut |
|---|---|
| Open / close | `⇧⌘Space` |
| Copy username | `⌘C` |
| Copy password | `⇧⌘C` |
| Copy one-time code (TOTP) | `⌥⌘C` |
| Open in browser (first http/https URL) | `⌥↩` |
| Open in Bitwarden (opens the app, not the item) | `⇧⌘O` |
| More actions | `→` |
| Back to list | `←` |
| Navigate list or actions menu | `↑` `↓` |
| Run | `↩` (with menu: selected action; without menu: opens URL if any, otherwise copies password) |
| Collection 1–9 | `⌘1`…`⌘9` |
| Clear search / back / close | `Esc` (clears search or collection; in menu returns to list; with empty field closes) |

Actions only appear if the item supports them (no username → no "Copy username"; no URL → no "Open in browser").

## How Touch ID works

The same binary acts as the `rbw` pinentry:

| Step | Who |
|---|---|
| `rbw` asks for the master password | `rbw-agent` calls `~/.local/bin/qa-pinentry` → `quick-access --pinentry` |
| `GETPIN` → Touch ID (`LAContext`, biometry only) | `pinentry.rs` |
| Touch ID ok → master password from Keychain (service `local.quick-access`, account `rbw-master-password`) | `pinentry.rs` |
| Cancelled / failed / not enrolled / any request other than the master password → forwarded to `pinentry-mac` (replays the received commands) | `pinentry.rs` |

This also covers the per-item master password re-prompt from `rbw`.

## Security model and limitations

| Topic | How it works | Limit |
|---|---|---|
| Keychain | Without a Developer ID, the protected Keychain returns `-34018` (`errSecMissingEntitlement`). So there is no item with `biometryCurrentSet` | Touch ID is enforced by our code (`LAContext`), not by the Keychain |
| Item ACL | Bound to the binary's signature | That is why a stable local identity matters: a rebuild with a different signature loses access |
| Clipboard | `org.nspasteboard.ConcealedType`; cleared after 45 s only if `changeCount` has not changed | If you copy something else before 45 s, it is not erased |
| Screen capture | Window is `contentProtected` (excluded from captures) | Depends on the system |
| Locking | `rbw lock` on sleep, screen off and screen lock | Also, rbw `lock_timeout` = 14400 s (240 min), same as the Bitwarden.app vault timeout |
| Webview | Strict CSP, no inline scripts | — |
| Tauri commands | Return only items without secrets, plus `Ok`/error | Secrets leave only through Rust, into the clipboard |

## Known limitations

| Limitation | Status |
|---|---|
| Does not appear over full-screen apps | `FullScreenAuxiliary` did not work. Next step: NSPanel via `tauri-nspanel` |
| No lock/unlock sync with Bitwarden.app | The app exposes no lock event. Bitwarden's "shared unlock" is in unreleased flags |
| `⇧⌘O` opens Bitwarden.app, not the item | No deep link |
| macOS only | `rbw` does not run on Windows |
| No favicons | Out of scope |

## Development

| Task | Command |
|---|---|
| Search tests | `node --test quick-access/src/` |
| Run in dev | `cd quick-access/src-tauri && cargo tauri dev` |
| Debug build | `cd quick-access/src-tauri && cargo tauri build --debug --bundles app` |

Design docs: [`docs/`](docs/) (spec, research, fork-vs-new-app decision).

## History

Replaces an Electron fork of `bitwarden/clients` and a Swift prototype. The fork had 10 commits, kept locally as a git bundle (base `bitwarden/clients@037a68b`), not published. The shortcut and search spec survives in [`docs/spec.md`](docs/spec.md).

## License

[MIT](LICENSE)
