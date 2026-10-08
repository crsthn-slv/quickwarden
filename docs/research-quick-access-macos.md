# Quick Access from 1Password 8 — is there a Bitwarden equivalent on macOS?

Investigation closed on **10 September 2026**. All dates given are those that appeared in the sources at that time.

Conventions used throughout the document:

- **[DOC]** — written in the manufacturer's official documentation.
- **[CODE]** — verified by direct reading of the public source code.
- **[COMMUNITY]** — comes from a forum, issue or third-party README.
- **[UNVERIFIED]** — could not be confirmed in a primary source.

Nothing was installed, executed or authorized during this investigation.

---

## Part 1 — How 1Password 8's Quick Access works

### Shortcut and invocation

| Action | macOS shortcut | Source |
|---|---|---|
| Open/close Quick Access | `⇧` `⌘` `Space` | [DOC] |
| Customize the shortcut | Settings › General › Shortcuts | [DOC] |
| Keyboard-free alternative | Right-click the menu bar icon › `Open Quick Access` | [DOC] |

There is also an option for Quick Access to open directly when clicking the menu bar icon, instead of opening the main app. [DOC]

### Search

When opened, Quick Access already shows suggestions before anything is typed: it cross-references the app or site in the foreground with the most frequently used items. This is the behavior that defines the feature — it is not a search box, it is a search box that already knows where you are. [DOC]

From there:

- typing filters the entire vault; [DOC]
- the account/collection icons in the search field narrow the scope, and `All Accounts` widens it; [DOC]
- `⌘` `1` to `⌘` `9` switch between collections, which persist across sessions; [DOC]
- advanced search accepts filters by tag, category, vault, favorites and untagged items. [DOC]

### Actions on the selected item

| Action | Shortcut | Source |
|---|---|---|
| Copy username / main field | `⌘` `C` | [DOC] |
| Copy password | `⇧` `⌘` `C` | [DOC] |
| Copy one-time code (TOTP) | `⌥` `⌘` `C` | [DOC] |
| Fill into the frontmost app | `⇧` `Return` | [DOC] |
| Open the site and fill | `⌥` `Return` | [DOC] |
| Open item in a separate window | `⌘` `O` | [DOC] |
| Open item in the 1Password app | `⇧` `⌘` `O` | [DOC] |

Available actions change depending on the item type. For a Wi-Fi item, you get copy base-station password, network password or network name; for an ID document, copy card number, name or expiry date. [DOC]

### Filling outside the browser

Universal Autofill (`⌘` `\`) fills desktop apps and system dialogs. When several items match, Quick Access presents the options. [DOC]

Since **29 May 2026**, in version 8.12.22, 1Password has been in public beta as a native macOS AutoFill provider, through Apple's Passwords API. It requires **macOS 14 Sonoma on Apple Silicon** and works alongside Universal Autofill, not as a replacement. [DOC]

### Confirmed item types

Logins, credit cards, delivery addresses, medical records, software license keys and one-time codes. [DOC]

### Unlocking

Quick Access inherits the state of the main app. `⇧` `⌘` `L` locks globally. Biometric unlock with Touch ID or Apple Watch is configured in the app, not in Quick Access. [DOC]

### Security

The setting `Remove copied information and authentication codes after 90 seconds` is **on by default** and lives in Settings › Security. [DOC]

### What is not documented

- **Passkeys in Quick Access** — none of the four official pages consulted mention passkeys in the Quick Access action list. **[UNVERIFIED]**
- **SSH keys in Quick Access** — 1Password's SSH agent is a separate feature. The private key never leaves the app, and each SSH client must be explicitly authorized; the user controls when approval is requested and how long the agent remembers it. No documentation links the agent to Quick Access. [DOC]
- **Extra fields** — an open request has existed since **9 May 2022** in the official forum to expose custom fields in Quick Access (the concrete case was SSH key passphrases). On 12 May 2022 a staff member logged it internally as `IDEA-I-969`, with no commitment or timeline. Several users described Quick Access as "very focused on basic logins" compared with the old 1Password mini. I found no indication it has been resolved. [COMMUNITY]

---

## Part 2 — What Bitwarden officially offers for the same problem

Short answer: **there is no equivalent.** Not even a partial one. There is no global shortcut that opens a vault search box on macOS.

### Desktop app

All documented shortcuts are internal — they only work when the Bitwarden window is already focused. [DOC]

| Action | Shortcut |
|---|---|
| Search the vault | `⌘` `F` |
| Copy username | `⌘` `U` |
| Copy password | `⌘` `P` |
| Copy TOTP | `⌘` `T` |
| Lock vault | `⌘` `L` |
| Hide in menu bar | `⌘` `⇧` `M` |
| Generator | `⌘` `G` |

Relevant settings: Touch ID unlock, PIN unlock, `Keep running in background` (gives access from the menu bar), automatic clipboard clearing with a timer, minimize on copy, block screen capture, launch at login. [DOC]

The menu bar gives access to the app. It does not allow searching. There is an open forum request about exactly that. [COMMUNITY]

### Browser extension

| Action | Shortcut |
|---|---|
| Open the extension | `⌘` `⇧` `Y` |
| Autofill last login (repeat cycles through items) | `⌘` `⇧` `L` |
| Generate password | `⌘` `⇧` `9` |
| Lock vault | `⌘` `⇧` `N` |

These shortcuts only act inside the browser. [DOC]

### SSH agent

It is official and available on Windows, macOS (App Store and .dmg), Linux, Snap and Flatpak. It is enabled in settings and has an `Ask for authorization when using SSH agent` option. With the vault unlocked, requests pass without an extra prompt; locked, signing requests require unlocking. You must point `SSH_AUTH_SOCK` to the Bitwarden socket, whose path varies by platform and install method. [DOC]

### Spotlight

I found no documentation of any Spotlight integration. **[UNVERIFIED]** — most likely it simply does not exist.

### Native macOS AutoFill provider

This deserves a note, because it is in motion.

The `bitwarden/clients` repository already contains an `autofill_provider` crate with a README describing in detail a **macOS Native Passkey Provider** introduced in PR `#13963`: a native Swift extension packaged in `PlugIns` (like the Safari extension), IPC over a unix socket implemented in Rust with UniFFI + NAPI bindings, and a "modal mode" in the Electron app for passkey and SSH operations. The README itself says that, in that PR, **only passkeys are provided** — not passwords. [CODE]

On the public side:

- the request `Register as macOS Password Provider`, opened on **10 February 2026**, still describes the option as nonexistent on macOS; [COMMUNITY]
- the request `Support macOS Native AutoFill Provider Framework`, from **22 June 2026**, was closed as a duplicate by a moderator, pointing to the earlier requests; [COMMUNITY]
- the release notes consulted do not mention a macOS passkey provider. The closest references are `SSH agent forwarding` (2025.3.3) and `FIDO2 two-step login for macOS desktop` (2025.2.1), which are something else. [DOC]

Conclusion: **[UNVERIFIED]** whether it is already active in a stable version. The code exists, the public feature is not confirmed, and even when it arrives it covers passkeys — it is not a Quick Access.

### CLI

This is where practically the whole community is based.

- Installation: native executables (Windows/macOS/Linux x64), `npm install -g @bitwarden/cli`, Chocolatey, Snap, Flatpak. **On ARM64, Bitwarden recommends npm** — relevant for Apple Silicon. [DOC]
- Authentication: `bw login` with email and master password, `bw login --apikey` with `client_id`/`client_secret`, or `bw login --sso`. [DOC]
- Unlocking: `bw unlock` returns a session key, exported as `BW_SESSION` or passed with `--session`. Accepts `--passwordenv <var>` and `--passwordfile <path>`. [DOC]
- The documentation explicitly says that chaining the factors into a single command **"isn't recommended for security reasons"**, and that the password file should be accessible only to the user running `bw unlock`. [DOC]
- `bw lock` and `bw logout` invalidate the active session. [DOC]
- `bw serve` starts a local Express server (port 8087, bound to localhost) that exposes the CLI actions over REST. By default it blocks any request with an `Origin` header; disabling that protection "is not recommended", and `--hostname all` opens it to the whole network. [DOC]

**Known risk with no official answer:** a forum thread from **28 January 2025** describes that, after unlock, an unlocked `data.json` remains on disk, and that `export BW_SESSION=...` typically ends up in `~/.bash_history`. The author also notes that versioned backups and Time Machine then contain those values. No official reply in the thread; it was referred to the HackerOne program. [COMMUNITY]

---

## Part 3 — Alternatives found

### Comparison table

| # | Project | URL | Last activity | License | Technology | Installation | Apple Silicon | Vaultwarden | Needs the CLI | Status |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | Bitwarden Vault (Raycast) | `github.com/raycast/extensions/tree/main/extensions/bitwarden` | 13 Jul 2026 | MIT | TypeScript / React | Raycast Store | Yes | Yes | Bundles its own | **Active** |
| 2 | Bitwarden Accelerator | `github.com/ajrosen/Bitwarden-Accelerator` | Active | GPL-3.0 | Shell + AppleScript (Alfred) | `.alfredworkflow` or `brew tap ajrosen/tap` | Yes | Not mentioned | Yes (+ `jq`) | Active, with reservations |
| 3 | bitwarden-alfred-workflow | `github.com/blacs30/bitwarden-alfred-workflow` | **Archived 10 Jun 2024** | MIT | Go | GitHub release | n/a | Yes (`SERVER_URL`) | Yes (≥1.19) | **Abandoned** |
| 4 | alfred-bitwarden | `github.com/twio142/alfred-bitwarden` | Recent | GPL-3.0 | Swift | Manual | Yes | Not mentioned | Yes | **Immature** (21 commits, 0 stars, no releases) |
| 5 | Wenigwarden | `github.com/cyprieng/wenigwarden` | v0.0.4, 9 Mar 2025 | MIT | Swift | Unsigned DMG | Yes | Yes | No | **Immature** (4 stars) |
| 6 | bitwarden-menubar | `github.com/jnsdrtlf/bitwarden-menubar` | Stalled | GPL-3.0 | Swift + extension v1.48.1 | Unsigned DMG | n/a | n/a | No | **Dead** |
| 7 | Swiftwarden | `github.com/jesse231/Swiftwarden` | WIP | n/a in README | Swift / SwiftUI | Manual build | Yes | Yes | No | **WIP** |
| 8 | Goldwarden | `github.com/quexten/goldwarden` | **Paused indefinitely** | n/a in README | Go | Binary / Flatpak | Build exists | Yes | No | **Discontinued** |
| 9 | rbw + rbw-agent | `github.com/doy/rbw` | v1.15.0, 31 Dec 2025 | MIT | Rust | `brew install rbw` | Yes (bottle) | Yes (`base_url`) | No — it replaces it | **Stable, CLI only** |

### Functional coverage against Quick Access

| Quick Access capability | Raycast | Accelerator | Wenigwarden | rbw |
|---|---|---|---|---|
| Global shortcut from any app | Yes (hotkey per command) | Yes (Alfred) | Yes (configurable hotkey) | No |
| Suggestions from the active app's context | **No** | Partial — reads the domain of the browser tab | No | No |
| Fuzzy search in the vault | Yes | Yes | Yes | Yes |
| Copy username / password / TOTP | Yes | Yes | Copy credentials; TOTP not mentioned | Yes |
| Paste into the frontmost app | Yes (`Paste to Active App`) | Yes | Not mentioned | No |
| True filling outside the browser | **No** | **No** | **No** | **No** |
| Passkeys | No | No | No | No |
| SSH keys | No | No | No | Yes (own agent) |
| Per-item master password re-prompt | Yes | Not mentioned | Not mentioned | n/a |
| Automatic clipboard clearing | Via Raycast | Yes, configurable | Not mentioned | n/a |

### Individual audits

#### 1. Bitwarden Vault for Raycast — the serious option

Author `jomifepe`, 15 listed contributors, MIT, about 61,156 installs in the Raycast Store. It lives inside the `raycast/extensions` monorepo, which means peer review by Raycast before each publication — a real difference from a `.alfredworkflow` downloaded loose.

Recent activity in the CHANGELOG: `2026-07-13` TOTP fix for secrets containing spaces, `2026-05-28` session handling fix in sync, `2026-05-27` CLI update to v2026.4.2, `2026-04-03` bundled CLI update from v2025.11.0 to v2026.2.0 because of `Invalid session token` errors caused by KDF upgrades on the server. [CODE]

Published commands: `Search Vault`, `Authenticator`, `Generate Password`, `Generate Password (Quick)`, `Create Folder`, `Create Login`, `Lock Vault`, `Logout`, `Create Send`, `Search Sends`, `Receive Send`. [CODE]

What I verified in the code, and what matters most here:

- **The master password does not go through arguments.** Unlock is `bw unlock --passwordenv BW_PASSWORD --raw`, with `BW_PASSWORD` injected into the child process environment. Nothing appears in `ps`. [CODE]
- **The session key goes in the environment**, never in `--session` on the command line. [CODE]
- **Session persistence:** the token is kept in Raycast's `LocalStorage`, key `sessionToken`. Raycast documents that storage as "Raycast's local encrypted database" and guarantees that "Extensions can *not* access the storage of other extensions". [CODE] + [DOC]
- **Re-prompt:** there is `sessionRepromptHash` and a `repromptIgnoreDuration` preference, meaning it honors the per-item re-prompt field from Bitwarden. [CODE]
- **Cache:** stores only the visible, non-sensitive part of the vault, encrypted with a key derived from the master password. Passwords, identity fields, cards and secure notes are never stored. Can be disabled in preferences. [CODE] + [COMMUNITY]
- **Data isolation:** sets `BITWARDENCLI_APPDATA_DIR` to the extension's own `supportPath`, so the bundled CLI's `data.json` does not mix with the `~/.config` of a `bw` installed by the user. [CODE]
- **TOTP is generated locally** with `@otplib`, without calling the CLI. [CODE]
- **Vaultwarden:** there are `serverUrl` and `serverCertsPath` preferences (path to a self-signed TLS certificate). [CODE]

Negatives, plainly:

- **The personal API `clientId` and `clientSecret` are stored in the extension's preferences.** They are of type `password`, which in Raycast means Keychain, but it is still a long-lived credential at rest.
- **Depends on Raycast**, which is proprietary, closed-source software. This is a trust decision, not a detail.
- **No context detection.** It is search, not suggestion. It lacks exactly the part that makes Quick Access fast.
- **Pasting is not filling.** `Paste to Active App` sends `⌘V` into the focused field; it does not identify fields, does not distinguish username from password, and does not respond to system dialogs.
- **The CLI's `data.json` is on disk** — it fully inherits the concern raised in the Bitwarden forum in January 2025.
- Issue `#26173`, opened **9 March 2026**, reported the extension unusable with `Cannot read properties of null (reading 'toWrappedAccountCryptographicState')`. It is closed, and the 3 April 2026 CHANGELOG entry about the bundled CLI update matches the cause described. [COMMUNITY] + [CODE]

**Auditability: high.** Readable TypeScript code, in a public repository with history and review.

#### 2. Bitwarden Accelerator (Alfred) — functional, with a design decision that stops me

99 stars, GPL-3.0, installable with `brew tap ajrosen/tap && brew install bitwarden-accelerator`, which resolves the `bitwarden-cli` and `jq` dependencies. Functionally it is rich: login by password or API key, 2FA by authenticator app, YubiKey OTP or email, automatic search by the domain of the active browser tab, copy username/password/TOTP/notes, edit items without leaving Alfred, download attachments, background sync via Launch Agent, automatic clipboard clearing, inactivity timeout and lock on screen lock. [COMMUNITY]

The problem: Touch ID unlock relies on storing the master password and, in the README's own words, "It does this by using *sudo* to store and retrieve your password in a secure location". Storing the master password is a defensible decision; doing it through `sudo` instead of the Keychain is not obviously so. I did not audit the concrete mechanism. **[UNVERIFIED]** — and that is precisely why I do not recommend it without a prior audit.

The README does not mention Vaultwarden. That does not make it incompatible, but it is neither tested nor supported. **[UNVERIFIED]**

Side note from the README: `SSO` is not supported, and `FIDO2`/`Duo` are not supported by the Bitwarden CLI.

#### 3. blacs30/bitwarden-alfred-workflow — discarded

The project with the most stars of all I found (439) and the easiest to recommend by reflex. It has been **archived since 10 June 2024**, with the note "If someone wants to take this project over please contact me". Worse: the README itself admits that "The workflow's internal decryption mechanism is currently not working" since v2.2.0, forcing reliance on the CLI to decrypt. Archived secret-management software with admittedly broken internal crypto is not installed.

#### 4. twio142/alfred-bitwarden — too early

A Swift rewrite of Accelerator, with ranking by browser domain, recent items and favorites. 21 commits, 0 stars, no releases, no security discussion. It stores the master password in the macOS Keychain for silent re-unlock, without any analysis of the trade-off. A new project, by a single author and without external review, is not where you put the vault key.

#### 5. Wenigwarden — the right architecture, the wrong maturity

It is the only one that does what I would do: native Swift client, no CLI, in the menu bar, with global search, keyboard navigation, configurable shortcuts and auto-lock. It supports Vaultwarden. MIT.

It has 4 stars, last release **v0.0.4 on 9 March 2025**, and is not signed with an Apple certificate — the README tells you to run `sudo xattr -rd com.apple.quarantine` before opening it. It has no password generator, does not manage the vault, does not support organizations, and TOTP is not mentioned.

Worth it as a design reference. Not worth putting a vault into.

#### 6, 7, 8. bitwarden-menubar, Swiftwarden, Goldwarden — discarded

- **bitwarden-menubar** packages the Bitwarden browser extension **at version 1.48.1**, and the author says it receives no updates, with a "Use at your own risk!" at the top.
- **Swiftwarden** calls itself WIP, and its own task list includes "Properly implement other Bitwarden features" and "Other encryption methods".
- **Goldwarden** has at the top of the README: "Development paused indefinitely". The reason is good news for Bitwarden users — SSH items, SSH agent, memory security and biometrics on Linux were all integrated into the official client. But the Mac builds are described by the author as "somewhat feature-stripped" and **untested**.

#### 9. rbw — not the solution, the foundation

Unofficial CLI client in Rust, 1.4k stars, MIT, `brew install rbw` with bottles for Apple Silicon, v1.15.0 from **31 December 2025**. Homebrew counts: 78 installs in 30 days, 227 in 90, 1101 in 365.

What sets it apart from the official `bw` is in the first paragraph of the README: the Bitwarden CLI is *stateless* and forces temporary keys to be passed in environment variables, "which makes it very difficult to use". `rbw` solves this with a background process, `rbw-agent`, that **keeps the keys in memory** — the same model as `ssh-agent` or `gpg-agent`. There is no `BW_SESSION` circulating through scripts or ending up in shell history.

Other relevant properties: configurable `base_url`, so Vaultwarden works; configurable `lock_timeout` (default 3600s) and `sync_interval`; profiles via `RBW_PROFILE`, each with its own vault and agent; built-in SSH agent; prompts through `pinentry`.

The author's own honest limitations: he considers the project "essentially feature-complete" and says new features are unlikely to be implemented on his own initiative, although PRs are accepted. 2FA supported: email, authenticator app and YubiKey OTP — **WebAuthn/passkey and Duo are not supported**. Against the official server you need to run `rbw register` with a personal API key, because Bitwarden tends to classify command-line traffic as bot traffic.

It has no graphical interface. The frontends listed are all Linux ones (rofi, fuzzel, ulauncher). **There is no frontend for macOS.**

---

## Part 4 — Recommendation

### The verdict

**There is no equivalent today to 1Password 8's Quick Access for Bitwarden on macOS.** What exists covers half the problem: finding and copying. The other half — the system knowing which app is in front and writing into the right fields — is not implemented by anyone, neither officially nor in the community.

### What to install now

**The Bitwarden Vault extension for Raycast.** It is the only one with active maintenance, peer review, a permissive license, correct secret handling and declared Vaultwarden support. It closes perhaps 60% of the distance to Quick Access.

I do not recommend it for being the most popular. I recommend it because it was the only one whose code I read and found nothing that stopped me: the master password enters via environment variable and not `argv`, the session token sits in Raycast's encrypted, isolated storage, the cache explicitly excludes sensitive fields, the per-item re-prompt is honored, and the CLI's `data.json` is confined to the extension's directory.

What it does not do, and what you should accept before installing: it does not suggest items from context, it does not fill — it pastes — and it does not touch passkeys or SSH keys.

### How to install and validate safely

Proposed order. None of this was executed.

**Before installing**

1. Create the personal API key in the Bitwarden account (Settings › Security › Keys › API Key). Do not use the master password as the login method for the extension.
2. If the target is Vaultwarden, have the server URL at hand and, if the certificate is self-signed, the certificate path.

**Installation**

3. Install Raycast and, from the Store, the `Bitwarden Vault` extension by `jomifepe`. Let it use the bundled CLI instead of pointing `cliPath` at a separately installed `bw` — this keeps `data.json` isolated in the extension's `supportPath`.
4. Fill in `clientId`, `clientSecret` and, if applicable, `serverUrl` and `serverCertsPath`.

**Hardening**

5. Set `repromptIgnoreDuration` to the lowest value you can tolerate.
6. Disable `shouldCacheVaultItems` if you prefer not to have even metadata cached, accepting the speed cost.
7. Assign a global shortcut to the `Search Vault` command — Raycast documents that a hotkey "launches a Raycast command from anywhere on your system", including with Raycast in the background. `⌥` `⌘` `Space` avoids a clash with Spotlight. This is the closest thing to `⇧` `⌘` `Space`.
8. In the Bitwarden desktop app, confirm that automatic clipboard clearing is enabled. It is the analog of the 90 seconds 1Password has by default.

**Validation**

9. Confirm that the CLI binary the extension downloads matches the version announced in the CHANGELOG and that the origin is Bitwarden.
10. Check the permissions and contents of the extension's `supportPath`: confirm that `data.json` is encrypted at rest and that no file is readable by other users.
11. Run once with `Console.app` open, filtered by the extension, and confirm no secret appears in logs.
12. Confirm that on screen lock the extension locks the vault (`VAULT_LOCK_MESSAGES` includes `SYSTEM_LOCK` and `SYSTEM_SLEEP`, but verify in practice).
13. Test with an item marked for re-prompt and confirm the master password is requested.

If any of steps 9 to 13 fails, stop and reassess.

### And then

Installing the Raycast extension does not solve the problem that motivated this research; it solves the easy part. Part 5 exists because the hard part — context-based suggestion and real filling — remains to be done, and it is implementable.

---

## Part 5 — Initial specification of a native Quick Access app for Bitwarden

This is a starting point for discussion, not a closed plan.

### Goal

A macOS menu bar app that, with a global shortcut, shows a search box already populated with suggestions based on the frontmost app, and that lets you copy or fill credentials in native apps — covering Bitwarden Cloud and Vaultwarden.

### Non-negotiable principles

1. No secret in `argv`, in inheritable environment variables, in logs or in unencrypted files.
2. The master password is never persisted. Not even in the Keychain.
3. Derived keys live only in memory, in a process separate from the interface.
4. The clipboard is always temporary, with guaranteed clearing.
5. Auditable code: no opaque dependencies on the secret path.

### Suggested architecture

Three processes, with minimal surface between them:

```
┌──────────────────────────┐
│  QuickAccess.app         │  Swift + SwiftUI, LSUIElement
│  menu bar + panel        │  never sees the master key
└───────────┬──────────────┘
            │ XPC (NSXPCConnection, code-signing requirement)
┌───────────▼──────────────┐
│  QuickAccessAgent        │  daemon; keeps keys in memory
│  index + crypto          │  auto-lock on timeout/sleep/lock
└───────────┬──────────────┘
            │ HTTPS
┌───────────▼──────────────┐
│  Bitwarden Cloud         │
│  or Vaultwarden          │
└──────────────────────────┘
```

**Open decision and the most important one in the project: how to talk to the server.**

| Option | For | Against |
|---|---|---|
| A — wrap `rbw-agent` | Crypto already implemented and tested by 1.4k users; in-memory agent model already solved; Vaultwarden supported; MIT | Author declared the project feature-complete; no WebAuthn/Duo; external dependency in Rust |
| B — wrap the official `bw` | Supported by the manufacturer; follows KDF upgrades | `data.json` on disk; session key crossing process boundaries; latency; this is exactly what broke the Raycast extension in April 2026 |
| C — implement the protocol in Swift/Rust | Full control; no external processes; trivial Vaultwarden support | Crypto from scratch in a password manager; no external audit; disproportionate risk for a personal project |

My inclination is **A**, with `rbw-agent` isolated and `pinentry` replaced by a native prompt with Touch ID. Still to be decided.

### Authentication and session

- Login by personal API key (`client_id`/`client_secret`), stored in the Keychain with `kSecAttrAccessibleWhenUnlockedThisDeviceOnly` and an ACL requiring `LAContext`.
- Master password requested in its own panel, kept in a locked `Data` buffer in memory (`mlock`) and erased by explicitly writing zeros after deriving the key.
- Subsequent unlock by Touch ID releasing the derived key stored in the Secure Enclave, with `kSecAccessControlBiometryCurrentSet` — automatic invalidation when biometrics change is the desired behavior.
- Auto-lock on three triggers: configurable inactivity timeout (default 15 min), `NSWorkspace.willSleepNotification` and screen lock.

### Local search

- In-memory index with metadata only: `id`, name, username, `uris`, folder, favorite, re-prompt flag. Never passwords.
- The secret is fetched from the agent **only at the moment of action**, item by item.
- Fuzzy matching with scoring by: domain match with the frontmost app, recency, favorites, name prefix match.
- Context detection: `NSWorkspace.frontmostApplication` for the bundle ID; for browsers, the domain of the active tab via Apple Events, with explicit permission request and a disable option.
- `bundleID → domain` map kept locally and editable by the user (e.g. `com.tinyspeck.slackmacgap → slack.com`).

### Global shortcut

`HotKey` via Carbon `RegisterEventHotKey` or `MASShortcut`, configurable, default `⌥` `⌘` `Space`. The panel is an `NSPanel` `.nonactivatingPanel` so it does not steal focus from the target app — a necessary condition for filling to work.

### Temporary clipboard

- Write with `NSPasteboard` marking `org.nspasteboard.ConcealedType`, which decent clipboard-history managers respect.
- Clearing by timer (default 45 s) **and** verification that the content is still ours before clearing, so as not to erase what the user copied in the meantime.
- Do not write to the `.general` pasteboard when direct filling is available.

### Filling

Three levels, from most correct to most fragile:

1. **Native AutoFill provider (`ASCredentialProviderExtension`)** — the right path; requires macOS 14+ on Apple Silicon; it is what 1Password moved to in May 2026. Covers apps that adopt the API. Medium-term target.
2. **Accessibility API (`AXUIElement`)** — locate `AXTextField`/`AXSecureTextField` in the focused window and write to it directly. Requires `AXIsProcessTrustedWithOptions`. This is how most apps that do not adopt Apple's API are covered.
3. **Keyboard event sending (`CGEvent`)** — last resort. Fragile, sensitive to keyboard layout and to apps that intercept events. Only with explicit user confirmation.

### Required permissions

| Permission | For what | Degradation without it |
|---|---|---|
| Accessibility | Filling levels 2 and 3 | Falls back to copy/paste |
| Automation (per browser) | Domain of the active tab | Loses contextual suggestion in browsers |
| Keychain | API key and derived key | Asks for credentials at every launch |
| Network (outbound) | Sync | Cached vault only |

None is requested at launch. Each is requested the first time the corresponding feature is used, with an explanation of why.

### Threat model

| Threat | Mitigation |
|---|---|
| Malware reading process memory | Keys only in the agent; `mlock`; hardened runtime; no `com.apple.security.get-task-allow` in release |
| Another process of the same user reading `data.json` | Vault encrypted at rest; files at `0600`; never write unlocked material |
| Secrets in logs or crash reports | `Secret` type that does not implement `CustomStringConvertible`; `os_log` with `%{private}`; crash reporter disabled or without payload |
| Secrets in `ps` or shell history | Zero CLI invocations with secrets in `argv`; communication only via XPC |
| Malicious app calling the agent | XPC with `NSXPCConnection` validating the client's code-signing requirement |
| Screenshot or screen recording of the panel | `NSWindow.sharingType = .none` |
| Clipboard persistence | `ConcealedType` + verified clearing |
| Shoulder-surfing | Fields hidden by default, reveal by explicit action |
| Compromise of the Vaultwarden server | End-to-end crypto preserved; optional certificate pinning |
| User filling into the wrong app | Always show the name of the target app before filling |

Out of scope, and accepted: an attacker with root or physical access to the unlocked machine.

### Tests

- **Unit** — key derivation against known vectors, vault parsing, search scoring engine, lock state machine transitions.
- **Integration** — against a local Vaultwarden instance in Docker: login, sync, unlock, timeout, re-prompt, KDF rotation.
- **Security** — automated test that runs the app and `grep`s logs, temporary files and the pasteboard for known secrets; verification that `ps -E` exposes nothing; test that the key is erased from memory after lock.
- **Interface** — XCTest UI tests for keyboard-only navigation.
- **Manual** — filling matrix per app: Safari, Chrome, Firefox, Slack, Terminal, iTerm2, system dialogs, VPN, Electron apps.

### Distribution

- Developer ID + notarization. **Not** the App Store, because the sandbox blocks Accessibility and complicates XPC.
- Sparkle 2 for updates, with a feed signed with EdDSA.
- Homebrew cask as the main installation method.
- Public repository, reproducible builds, published checksums.
- A clear note in the README: it is an unofficial client, not affiliated with Bitwarden Inc.

### Suggested phasing

| Phase | Deliverable | Estimated effort |
|---|---|---|
| 0 | Decide A/B/C; prototype of the agent authenticating and syncing against Vaultwarden | 1–2 weeks |
| 1 | Menu bar, global shortcut, search, copy with temporary clipboard | 2–3 weeks |
| 2 | Context detection and suggestions | 1–2 weeks |
| 3 | Filling via Accessibility | 2–3 weeks |
| 4 | Hardening, security tests, notarization, cask | 2 weeks |
| 5 | `ASCredentialProviderExtension` and passkeys | to be defined |

Estimates for one person working part-time. **[UNVERIFIED]** — these are my guesses, not a measurement.

---

## Sources

**1Password (official)**

- [Get to know Quick Access](https://support.1password.com/quick-access/?mac) — accessed 10 Sep 2026
- [How to Use Quick Access to View Your Passwords](https://1password.com/features/how-to-use-quick-access-in-1password-8) — accessed 10 Sep 2026
- [How to navigate 1Password like a pro with Quick Access](https://1password.com/blog/navigate-1password-quick-access) — accessed 10 Sep 2026
- [1Password keyboard shortcuts](https://support.1password.com/keyboard-shortcuts/) — accessed 10 Sep 2026
- [1Password SSH agent](https://www.1password.dev/ssh/agent/) — accessed 10 Sep 2026
- [May 2026 at 1Password: Native macOS AutoFill](https://www.1password.community/announcements-52/may-2026-at-1password-native-macos-autofill-a-new-developer-site-and-more-24666) — public beta 29 May 2026, v8.12.22

**1Password (community)**

- [1Password 8 Quick Access Mac — more fields](https://www.1password.community/1password-at-home-31/1password-8-quick-access-mac-more-fields-12802) — 9 to 30 May 2022, `IDEA-I-969`

**Bitwarden (official)**

- [App Settings](https://bitwarden.com/help/app-settings/) — accessed 10 Sep 2026
- [Keyboard Shortcuts](https://bitwarden.com/help/keyboard-shortcuts/) — accessed 10 Sep 2026
- [Bitwarden SSH Agent](https://bitwarden.com/help/ssh-agent/) — accessed 10 Sep 2026
- [Password Manager CLI](https://bitwarden.com/help/cli/) — accessed 10 Sep 2026
- [Release Notes](https://bitwarden.com/help/releasenotes/) — accessed 10 Sep 2026

**Bitwarden (code and community)**

- [`autofill_provider` README](https://github.com/bitwarden/clients/blob/main/apps/desktop/desktop_native/autofill_provider/README.md) — PR `#13963`, macOS Native Passkey Provider
- [In the OSX app, global hotkey to search vault](https://community.bitwarden.com/t/in-the-osx-app-global-hotkey-to-search-vault/9141) — opened 27 Nov 2019, no official reply
- [Register as macOS Password Provider](https://community.bitwarden.com/t/register-as-macos-password-and-passkey-provider/93723) — 10 Feb 2026
- [Support macOS Native AutoFill Provider Framework](https://community.bitwarden.com/t/support-macos-native-autofill-provider-framework-for-passkeys-passwords/97993) — 22 Jun 2026, closed as duplicate
- [Bitwarden CLI usage can easily result in secrets stored on disk](https://community.bitwarden.com/t/bitwarden-cli-usage-can-easily-result-in-secrets-stored-on-disk/80009) — 28 Jan 2025, no official reply

**Alternatives**

- [Bitwarden Vault (Raycast Store)](https://www.raycast.com/jomifepe/bitwarden) — 61,156 installs
- [Extension source code](https://github.com/raycast/extensions/tree/main/extensions/bitwarden) — CHANGELOG up to 13 Jul 2026
- [Issue #26173](https://github.com/raycast/extensions/issues/26173) — 9 Mar 2026, closed
- [ajrosen/Bitwarden-Accelerator](https://github.com/ajrosen/Bitwarden-Accelerator) — GPL-3.0, 99 stars
- [blacs30/bitwarden-alfred-workflow](https://github.com/blacs30/bitwarden-alfred-workflow) — archived 10 Jun 2024
- [twio142/alfred-bitwarden](https://github.com/twio142/alfred-bitwarden) — GPL-3.0, 21 commits
- [cyprieng/wenigwarden](https://github.com/cyprieng/wenigwarden) — MIT, v0.0.4 on 9 Mar 2025
- [jnsdrtlf/bitwarden-menubar](https://github.com/3j14/bitwarden-menubar) — stalled at extension v1.48.1
- [jesse231/Swiftwarden](https://github.com/jesse231/Swiftwarden) — WIP
- [quexten/goldwarden](https://github.com/quexten/goldwarden) — development paused
- [doy/rbw](https://github.com/doy/rbw) — MIT, 1.4k stars, v1.15.0 on 31 Dec 2025
- [Homebrew: rbw](https://formulae.brew.sh/formula/rbw) — 78/227/1101 installs over 30/90/365 days

**Raycast (official)**

- [Storage API](https://developers.raycast.com/api-reference/storage) — "local encrypted database", isolation between extensions
- [Command Aliases & Hotkeys](https://manual.raycast.com/command-aliases-and-hotkeys) — global hotkeys per command
