# Decision: fork of the official client vs. new app

Complement to the investigation of 10 September 2026. Source conventions are the same as in the previous document: **[DOC]**, **[CÓDIGO]** (code), **[COMUNIDADE]** (community), **[NÃO VERIFICADO]** (unverified). In the translation: **[CODE]**, **[COMMUNITY]**, **[UNVERIFIED]**.

## Updated requirements

| # | Requirement | Impact |
|---|---|---|
| 1 | Bitwarden Cloud **and** Vaultwarden | Eliminates implementations that assume only the official server |
| 2 | Copy is enough — no filling needed | **Reduces the project by about two thirds** |
| 3 | No Raycast, no Alfred | Eliminates the previous recommendation |
| 4 | macOS, Windows and Linux | Eliminates native Swift |

Requirement 2 changes the most. Without filling, no Accessibility API, no `AXUIElement`, no `CGEvent`, no `ASCredentialProviderExtension`, no Automation permissions, and no per-app manual test matrix. Almost all of the complexity and almost all of the risk disappear.

Requirement 4 rules out a native Swift app. Writing three apps — Swift, WinUI, GTK — for one feature is disproportionate.

---

## First: what `rbw-agent` is

This was left unanswered. `rbw` is an unofficial command-line client for Bitwarden, written in Rust. `rbw-agent` is the background process that goes with it.

The problem it solves: the official `bw` keeps no state. Each command requires you to pass the session key in an environment variable, which in practice means `export BW_SESSION=...` passed around by scripts, files and shell history.

`rbw-agent` does what `ssh-agent` and `gpg-agent` do: it runs in the background, keeps the derived keys **in memory**, and answers requests over a local socket. You write `rbw get github` and it returns the password, without a session key ever travelling through environment variables. It has a configurable `lock_timeout` (default 3600 s) and its own SSH agent. [COMMUNITY]

It was the foundation I proposed for an app built from scratch. With the decision below, it is no longer needed.

---

## The discovery that decides this

I read the `bitwarden/clients` code to see how much work it would take to graft Quick Access onto the official app. What I found changes the picture.

### A global shortcut is already registered

`apps/desktop/src/autofill/main/main-desktop-autotype-mvp.service.ts` — 158 lines, in the Electron main process:

```ts
import { ipcMain, globalShortcut } from "electron";
import { autotype_mvp } from "@bitwarden/desktop-napi";
```

The service registers a global shortcut, validates it, allows reconfiguring it over IPC, and in the callback gets the foreground window title and sends it to the renderer. Default shortcut: `Control` `Alt` `B`. [CODE]

### An always-on-top modal window already exists

`apps/desktop/src/main/window.main.ts` has `loadUrl(targetPath, modal)` and `createWindow("modal-app")`. `apps/desktop/src/platform/popup-modal-styles.ts` applies to it: 600×600, `setResizable(false)`, `setAlwaysOnTop(true)`, menu bar hidden, centered or positioned at coordinates. There is a `modalMode$` in `DesktopSettingsService`, and when leaving modal mode the main window hides itself, because — in the words of the code comment — "modal is used in front of another app". [CODE]

It was built for passkey and SSH operations, which use the `/passkeys` route. It is exactly the same primitive a Quick Access needs.

### What is missing on macOS and Linux is literally this

`apps/desktop/desktop_native/autotype/src/mvp/macos.rs`, in full:

```rust
// MVP, delete with PM-41067

pub fn get_foreground_window_title() -> anyhow::Result<String> {
    todo!("Bitwarden does not yet support macOS autotype");
}

pub fn type_input(_input: &[u16], _keyboard_shortcut: &[String]) -> anyhow::Result<()> {
    todo!("Bitwarden does not yet support macOS autotype");
}
```

`linux.rs` is identical, with "Linux" in place of "macOS". The crate's `Cargo.toml` only declares `cfg(windows)` dependencies. [CODE]

The whole structure — global shortcut, IPC, validation, configuration, connection to the renderer — is TypeScript and already cross-platform. The gap is two Rust functions, and only on the *autotype* side.

And since you decided that copy is enough, **you need neither of the two.** `type_input` is filling. `get_foreground_window_title` is only for context-based suggestions, which is an extra.

### Status of the official feature

Two feature flags in `libs/common/src/enums/feature-flag.enum.ts`:

```ts
WindowsDesktopAutotype = "windows-desktop-autotype",
WindowsDesktopAutotypeGA = "windows-desktop-autotype-ga",
```

Both default to `FALSE`. The name starts with `Windows`. The comment `// MVP, delete with PM-41067` appears in seven files. [CODE]

Reading: Bitwarden is building this, Windows first, still behind a flag. There is no public timeline signal for macOS and Linux. **[UNVERIFIED]**

---

## Comparison

| Criterion | Fork of `bitwarden/clients` | New app |
|---|---|---|
| Vault encryption | Already exists, audited, maintained | To build or delegate |
| Sync, KDF, key rotation | Already exists | To build; this is what broke the Raycast extension in April 2026 |
| Bitwarden Cloud + Vaultwarden | Already supported | To build |
| Organizations, collections, shared items | Already exists | A lot of work |
| TOTP | Already exists | To build |
| Biometric unlock on 3 OSes | Already exists | To build three times |
| Auto-lock, timeout, sleep, screen lock | Already exists | To build |
| SSH agent | Already exists | To build |
| Three operating systems | Electron, already solved | Three implementations or another Electron |
| Global shortcut | **Already there** (`globalShortcut`) | To build |
| Always-on-top modal window | **Already there** (`popup-modal-styles`) | To build |
| New code surface | One Angular route and one main-process service | A whole app |
| New attack surface | Small and localized | The whole app |
| Keeping up to date | Rebase onto `main` | Chasing protocol changes alone |
| License | GPL-3.0 — fork allowed | Free |
| Brand | Must change name and icons | No problem |
| Automatic updates | Inherit Sparkle/electron-updater, but pointed at you | To build |
| Signing and notarization | Required, and paid | Same |

---

## Recommendation

**Fork `bitwarden/clients`.** Given the requirements, this is not a hard decision.

The argument is not "save work". It is that the alternative means reimplementing the encryption of a password manager, and keeping it in step with protocol changes that Bitwarden makes without warning to anyone — the April 2026 KDF incident, which left the Raycast extension unusable with `toWrappedAccountCryptographicState`, is exactly that risk materializing in a third-party project.

The fork inverts the relationship: instead of reimplementing the client and chasing it, you add a window to a client that is already correct.

### What changes in the code

Three pieces. None touches cryptography.

**1. `MainQuickAccessService`** — in the main process, modeled on `MainDesktopAutotypeMvpService`. Registers a configurable `globalShortcut`; in the callback, calls `windowMain.loadUrl("/quick-access", true)`, which already applies the modal styles and shows the window. On close, it leaves modal mode, which already hides the main window.

**2. `/quick-access` route** — a new Angular component in `apps/desktop/src/vault/app/quick-access/`. Search field, results list, arrow-key navigation, `Enter` copies. Uses the existing vault services — `CipherService`, `SearchService`, `TotpService`. Does not talk to the network or the disk directly.

**3. Settings** — panel to enable it, choose the shortcut, set what `Enter` copies by default, and the clipboard clearing time. Reuses `DesktopSettingsService`. Clipboard clearing already exists in the app (`ClipboardMain`).

Shortcuts inside the panel, mirroring 1Password where it makes sense:

| Action | Shortcut |
|---|---|
| Open/close | configurable, suggestion `⌥` `⌘` `Space` / `Ctrl` `Alt` `Space` |
| Copy username | `⌘` `C` / `Ctrl` `C` |
| Copy password | `⇧` `⌘` `C` / `Ctrl` `Shift` `C` |
| Copy TOTP | `⌥` `⌘` `C` / `Ctrl` `Alt` `C` |
| Open item in app | `⇧` `⌘` `O` / `Ctrl` `Shift` `O` |
| Close | `Esc` |

### Points of attention

- **Locked vault.** The shortcut must open something useful when the vault is locked — an unlock panel with biometrics, then search. It must not fail silently.
- **Per-item re-prompt.** Items marked for re-prompt must ask for the master password here too. Easy to forget on a new surface.
- **`setAlwaysOnTop(true)` is already in `applyPopupModalStyles`.** Still to confirm whether the window steals focus from the previous app on macOS; if it does, a non-activating `NSPanel` is needed, and that may not be trivial in Electron. **[UNVERIFIED]** — this is the first risk to test.
- **Screen capture.** The app has a setting to block capture; confirm it applies to the modal window.
- **Shortcut conflicts.** `globalShortcut.register()` returns `false` if the shortcut is already taken. The autotype service already handles this; copy the pattern and show an error to the user.

### Legal and practical constraints

- **License: GPL-3.0** in `apps/desktop`. Fork allowed, with the obligation to keep GPL-3.0 and publish the code. The `/bitwarden_license` folder has a different license and should be left out. [DOC]
- **Brand.** "Bitwarden" is a registered trademark of Bitwarden Inc. A distributed fork must change name, icon and app identifiers. I found no brand policy file in the repository — when in doubt, rebrand completely. **[UNVERIFIED]**
- **Signing.** Developer ID and notarization on macOS; a code-signing certificate on Windows. Without these, users see warnings. It is a real annual cost.
- **Updates.** Point `electron-updater` at your own feed. Never leave it pointing at Bitwarden's.

### Contribute upstream — worth trying, but not blocking

Bitwarden accepts contributions, with two conditions: sign the Contributor Agreement at `cla-assistant.io/bitwarden/clients`, and **discuss significant features before writing code**, by creating a post in the Password Manager category of GitHub Discussions. [DOC]

I recommend opening that discussion early, for two practical reasons. First, to learn whether there is already internal work in progress — given `PM-41067` and the autotype flags, it is quite possible there is. Second, if they accept it, you stop maintaining a fork, which is the hidden cost of this whole approach.

But do not wait for the answer to start. The fork works either way, and a working implementation is a better argument than a proposal.

---

## Plan

Estimates for one person part-time. These are my guesses, not measurements. **[UNVERIFIED]**

| Phase | What | How you know it is done | Effort |
|---|---|---|---|
| 0 | Clone, build on all three OSes, run the app from source | `npx nx serve desktop` starts and unlocks against Vaultwarden and Cloud | 2–4 days |
| 1 | **Spike on the biggest risk**: modal window that appears via global shortcut without stealing focus, on all three OSes | Short video of the behavior on each OS | 3–5 days |
| 2 | `MainQuickAccessService` + `/quick-access` route with search and copy | Shortcut opens, searches, `Enter` copies, `Esc` closes | 1–2 weeks |
| 3 | Locked vault, re-prompt, TOTP, clipboard clearing | State matrix tested | 1 week |
| 4 | Settings panel and shortcut configuration | Shortcut configurable and persistent | 3–5 days |
| 5 | Rebrand, signing, notarization, update feed, builds on three OSes | Signed installers that update | 1–2 weeks |
| 6 | *Optional*: implement `get_foreground_window_title` on macOS and Linux for context suggestions | Suggestions appear before typing | 1 week per OS |

Phase 1 exists because it is where the project can die. If Electron cannot show a window over another app without stealing focus on macOS, the experience degrades, and it is better to know that in the first week than in the sixth.

Phase 6 is what separates "quick search" from "Quick Access". It comes after something works, and the macOS implementation goes through `NSWorkspace.frontmostApplication` via FFI — not hard, but not the critical path.

---

## Sources for this second part

All code cited was read in `bitwarden/clients@main` on **10 September 2026**, `apps/desktop` version **2026.9.0**.

- [`LICENSE.txt`](https://github.com/bitwarden/clients/blob/main/LICENSE.txt) — GPL-3.0 by default; Bitwarden License only in `/bitwarden_license`
- [`apps/desktop/package.json`](https://github.com/bitwarden/clients/blob/main/apps/desktop/package.json) — `"license": "GPL-3.0"`, version 2026.9.0
- [`main-desktop-autotype-mvp.service.ts`](https://github.com/bitwarden/clients/blob/main/apps/desktop/src/autofill/main/main-desktop-autotype-mvp.service.ts) — `globalShortcut`, IPC, callback
- [`main-autotype-keyboard-shortcut.ts`](https://github.com/bitwarden/clients/blob/main/apps/desktop/src/autofill/models/main-autotype-keyboard-shortcut.ts) — `DEFAULT_KEYBOARD_SHORTCUT = ["Control", "Alt", "B"]`, Windows-only support note
- [`window.main.ts`](https://github.com/bitwarden/clients/blob/main/apps/desktop/src/main/window.main.ts) — `loadUrl(path, modal)`, `createWindow("modal-app")`, `modalMode$`
- [`popup-modal-styles.ts`](https://github.com/bitwarden/clients/blob/main/apps/desktop/src/platform/popup-modal-styles.ts) — 600×600, not resizable, always-on-top
- [`autotype/src/mvp/macos.rs`](https://github.com/bitwarden/clients/blob/main/apps/desktop/desktop_native/autotype/src/mvp/macos.rs) and [`linux.rs`](https://github.com/bitwarden/clients/blob/main/apps/desktop/desktop_native/autotype/src/mvp/linux.rs) — `todo!()`
- [`feature-flag.enum.ts`](https://github.com/bitwarden/clients/blob/main/libs/common/src/enums/feature-flag.enum.ts) — `WindowsDesktopAutotype`, `WindowsDesktopAutotypeGA`, both `FALSE`
- [`autofill_provider/README.md`](https://github.com/bitwarden/clients/blob/main/apps/desktop/desktop_native/autofill_provider/README.md) — modal mode and PR `#13963`
- [Contributing Guidelines](https://contributing.bitwarden.com/contributing/) — CLA required, prior discussion for large features
- [Clients — Desktop](https://contributing.bitwarden.com/getting-started/clients/desktop/) — `npx nx serve desktop`, Rust native module compiled separately
- [doy/rbw](https://github.com/doy/rbw) — `rbw-agent` model
