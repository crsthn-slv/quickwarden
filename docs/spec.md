# Quickwarden spec — port from the Swift prototype to the fork

Source: 1Password 8 documentation (read 10 Sep 2026) + what is already implemented and tested
in the `QuickAccess.swift` prototype. This file is what survives from the prototype. The rest (rbw, Swift panel,
LaunchAgent) is dropped, because the official app already provides the vault, lock, `globalShortcut`,
modal window and clipboard clearing.

## Shortcuts

| Action | Shortcut | Notes |
|---|---|---|
| Open/close | `⇧` `⌘` `Space` | same as 1Password; configurable |
| Copy username | `⌘` `C` | |
| Copy password | `⇧` `⌘` `C` | |
| Copy one-time code | `⌥` `⌘` `C` | |
| Open in browser | `⌥` `↩` | first `http(s)` URI of the item |
| Open item in app | `⇧` `⌘` `O` | |
| More actions | `→` | `←` goes back |
| Run selected action | `↩` | in the actions menu |
| Navigate | `↑` `↓` | list **or** actions menu, depending on mode |
| Switch collection | `⌘` `1`…`⌘` `9` | folders/collections; selection persists |
| Clear search | `Esc` | only closes if the search is already empty |
| Close | `Esc` with empty field, click outside, or X in the field | |

`↩` without modifier: opens in browser if the item has a URI, otherwise copies the password.

## Search

Score per term; all terms must appear. Comparison is always done on folded text
(`diacriticInsensitive` + `caseInsensitive`) — without this "Apple" did not find "ID Apple" and
"acao" did not find "Ação Social". Both were real bugs in the prototype.

| Condition | Points |
|---|---|
| Name starts with the term | 4 |
| Some word in the name starts with the term | 3 |
| Name contains the term | 2 |
| Username, URI or folder contains the term | 1 |
| None | item discarded |

Tie → alphabetical order by folded name.

Cases the self-check covers (`quick-access --selftest` in the old prototype; now in `quick-access/src/search.test.js`, run with `node --test`):
`apple` → `ID Apple` · `id apple` → `ID Apple` · `APPLE` → `ID Apple` · `acao` → `Ação Social` ·
`icloud` → `ID Apple` (finds by username) · `zzz` → empty ·
`a` → `Amazon` before `ID Apple` · collection filter isolates the collection.

## State and navigation

**Separate indexes for the list and for the actions menu.** Sharing one index was a bug:
navigating the actions menu changed the selected item underneath.

The actions shown depend on the item: no username → no "copy username"; no URI → no "open in browser"; notes do not show "copy password".

## Security

- Clipboard marked `org.nspasteboard.ConcealedType` and cleared after 45 s, **only if the
  content is still ours** — do not erase what the user copied in the meantime.
  In the fork this already exists: `ClipboardMain`.
- Window excluded from screen capture (`sharingType = .none`; in Electron, `setContentProtection`).
- Locked vault: the shortcut must open the unlock panel, never fail silently.
- Items with master-password *re-prompt* must ask for the master password here too.
- Lock on sleep, on screen lock and on timeout — in the fork this is inherited from the app.

## Accessibility

- Each row is an element labeled `name, username - uri` with selected state.
- Copy confirmation announced to the screen reader, not only shown visually.
- Decorative icons hidden from the screen reader.
- Visible, permanent focus ring on the search field (2.5 px) — it indicates that you are typing. In the prototype, a blinking focus ring was a real complaint.
- Respect *Reduce Motion* when scrolling the list.
- Contrast ≥ 4.5:1 for text over icon colors.

## Out of scope for this phase

Autofill, suggestions based on the frontmost app
(`NSWorkspace.frontmostApplication`), real favicons, advanced filters by tag/category.
