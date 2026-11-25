# Redesign log

Journal for the autonomous redesign work. Each step records what was done and what was not verified.

Decisions:

- Branch `redesign` was created from the current `profile-export` HEAD because no `redesign` branch existed. `main` was not touched.
- Fonts: no dependencies added. `Inter` / `JetBrains Mono` are declared as preferred families with installed fallbacks (`Open Sans` / ui-monospace) in the design tokens.
- The application is not launched for interaction. Only static checks (`deno task check`) and source review are used.
- Device under design target: Mirabox HSV293S, 6 columns x 3 rows, only the first 5 columns have screens. Photo wall targets a configurable number of leading columns.

## Step 1 - Photo wall columns with screens

Done: added a session-local `Columns with screens` control (1..device.columns, default device.columns). Slicing, preview outlines and tile count use that number; tile positions remain `r * columnsWithScreens + c`, so only leading physical columns of the device are written. Reset keeps its previous snapshot-based behavior. Dev limit max follows the reduced tile count.

Not verified: no GUI run, no visual check of the new slider and no hardware apply/reset test.

Symbol check: `git grep` matched only pre-existing language option labels in `SettingsView.svelte` (`中文`, `日本語`). They are intentional locale names, not redesign additions; left unchanged.

## Step 2 - adaptive device grid

Done: replaced the column-count zoom thresholds with a CSS container query. The grid shell establishes `container-type: inline-size`; the inner scale uses `zoom: min(1, calc(100cqw / (var(--grid-columns) * 132px + 32px)))`. The CSS variable comes from `device.columns`, so any column count scales to the available width. `overflow-hidden` stays on the shell, so no horizontal scrollbar and no clipped right column.

Not verified: no GUI run, so actual browser support for length division inside `calc()` in the Tauri webview was not observed. If the webview rejects it, the grid stays at zoom 1 and scrolls internally; fallback would be a resize observer (not added because it is logic, not markup/style).

## Step 3 - titlebar device selector and sidebar navigation

Done: device selection moved from the sidebar into the centre column of a three-column titlebar. A status dot uses real registry data: green when `selectedDevice` resolves in `devices`, red otherwise. The sidebar keeps only the existing surfaces (Devices label, Plugins popup, Profiles select, Settings popup); no Backups screen or fake pages were added. All DeviceSelector bindings and Popup triggers are unchanged.

Not verified: no GUI run, so centring with unequal left/right column content was not measured visually.

## Step 4 - Devices workspace layout

Done: main area is now three columns - a narrow action library (existing `ActionList`, still bound to the same singleton), the device stage, and a 360px inspector. Header left holds the existing profile select, centre the device selector, right brightness and Photo wall. Inspector has local `Appearance / Action` tabs: Appearance renders the existing `InstanceEditor` in a new `inline` layout mode (floating popup mode unchanged for right-click Edit), Action hosts the existing `PropertyInspectorView` iframes. The iframe container stays mounted and is only hidden while the Appearance tab is active, so plugin PI sessions survive tab switches. Empty state shows a hint. No Advanced tab: no real data exists for it.

Not verified: no GUI run; selected-instance lookup handles full ActionContext strings from `$inspectedInstance` but empty-slot selections store a Context object and intentionally show the empty state. The inspector may render both the Key's floating editor and the inline editor if a user right-clicks a selected button; both remain functional.

## Step 5 - Plugins screen token pass

Done: plugin cards already matched the mockup structure from the earlier redesign pass (icon, name, author, mono version, description from localisations, status badge with exit code, Restart / Settings-or-logs / Uninstall, Auto-restart, search and All/Installed/Running/Crashed chips). This step replaces remaining hard-coded yellow shades with `app-warning` / `app-warning-soft` tokens and adds `aria-pressed` to the filter chips. No new data field was invented: Updates, Disable/Enable, Open plugin folder and a fake error line remain absent.

Not verified: no GUI run; card wrapping and badge contrast were checked in source only.

## Step 6 - tokens across remaining screens and dialogs

Done: added `--font-sans` (Inter with installed fallbacks) and `--font-mono` (JetBrains Mono with installed fallbacks) tokens; body and numeric/version labels use them. Converted remaining hard-coded neutral/blue/indigo/red/green/yellow utility classes in components and global select/readme styles to `app-*` tokens. Dialogs use 12px radius (`rounded-xl`), inline panels 8px (`rounded-lg`), spacing stays on the 4px scale. Popups, Settings, Profiles, PluginDetails, tooltips, context menus, action library and Photo wall all follow the same tokens.

Not verified: no GUI run; no visual contrast or typography measurement. Inter / JetBrains Mono are not bundled (no dependencies allowed), so the browser will use the fallback families unless the user has them installed.

## Step 7 - accessibility and polish

Done: global `:focus-visible` 2px accent outline with 2px offset for buttons, selects, inputs, textareas, role=button and focusable elements; disabled controls are dimmed and non-interactive; active buttons get a 1px press shift; input/select/textarea hover border uses the accent token. Added a shared `.empty-state` class and applied it to the no-devices screen and the inspector empty state. Colour tokens were chosen for AA contrast on the dark background.

Not verified: no GUI run, so focus ring visibility, actual contrast ratios and hover/pressed feel were not measured in the Tauri webview.

## Step 8 - safe performance pass

Done: source review found no `blur()` or `backdrop-filter` anywhere, so the "blur only on sidebar and modals" rule is already satisfied. Removed `transition-colors` from the high-count action list rows and plugin cards to avoid per-hover paint work on long lists, and reduced the floating editor modal shadow from `shadow-2xl` to `shadow-lg`. Device grid zoom is a pure CSS calculation that only recalculates on container resize; no JS resize logic was added.

Not verified: no GUI run, no profiling, no measurement of repaint counts.

## Step 9 - final report

Done: created `REDESIGN_REPORT.md` with step summary, no rollbacks, mockup differences, the full list of things not visually verified, and a five-item manual check list. Pushed branch `redesign` to origin without force.

Not verified: report content was not rendered as Markdown in the app; no GUI run at any step.

## Visual-fix 1 - titlebar overflow

Done: device chip has a 120px minimum and 220px maximum width, the device select uses `truncate` with ellipsis, and all three header columns are `min-w-0` with reduced gaps and a narrower brightness slider for 960px windows. Not verified in GUI.

## Visual-fix 2 - key tile size

Done: removed the 112/144 inner scale from `Key`, so a normal key now occupies the full 132px cell before CSS zoom. The grid shell still centres the panel and the container-query zoom shrinks the whole grid when space is tight; tiles grow back to 132px when the container allows. `size` still controls canvas resolution and the border margin; ParentActionView keeps its explicit 3/4 scale. Not verified in GUI; no horizontal scrollbar because the shell stays `overflow-hidden`.

## Visual-fix 3 - Photo wall column persistence

Done: `Columns with screens` is stored in `localStorage` under `opendeck.photoWall.screenColumns`, restored on mount with clamping to the current device, and written back on change. No Rust or settings-store changes. Not verified across an actual app restart; storage errors are swallowed on purpose.
