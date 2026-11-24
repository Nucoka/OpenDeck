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

## Step 5 - pending
