# To-dos

## Known limitations

- [ ] Show over full-screen apps. `FullScreenAuxiliary` is set in `macos.rs`, but the README says it did not work; re-test, then try NSPanel via `tauri-nspanel`.
- [ ] Sync lock/unlock with Bitwarden.app. Blocked: the app exposes no lock event; "shared unlock" is behind unreleased flags.
- [ ] `⇧⌘O` opens the item, not just Bitwarden.app. Blocked: no deep link.
- [ ] Store the master password in a `biometryCurrentSet` Keychain item, so the Keychain enforces Touch ID. Needs a Developer ID (see `pinentry.rs`).

## Watch upstream

- [ ] Unlock through Bitwarden.app's Touch ID. Since Desktop/CLI 2026.10.0, `bw unlock` without a password asks the running desktop app for biometric unlock ([#22448](https://github.com/bitwarden/clients/pull/22448)). Would solve lock sync and the Keychain limit above, and remove our stored-password pinentry. Blocked: behind the `biometrics-sdk-ipc` flag (off, server-controlled), only tested end-to-end on Linux, and it is `bw`, not `rbw`. When the flag ships: prototype with `bw` on macOS and measure whether its speed is acceptable vs. `rbw`.

## Out of scope for this phase

From `docs/spec.md`:

- [ ] Autofill
- [ ] Suggestions based on the frontmost app (`NSWorkspace.frontmostApplication`)
- [ ] Real favicons
- [ ] Advanced filters by tag/category
