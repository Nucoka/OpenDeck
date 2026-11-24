# Redesign report

Fork of OpenDeck, branch `redesign`. All work was done without launching the GUI and without changing Rust code, handlers, stores or Tauri invocations, except where a step explicitly required new local UI state.

## What was done

1. **Photo wall columns with screens.** Added a session-local `Columns with screens` control (1..device.columns, default device.columns). Slicing, preview and tile count use it; tile positions stay `r * columnsWithScreens + c`, so only leading physical columns are written. Reset is unchanged.
2. **Adaptive device grid.** Replaced column-count zoom thresholds with CSS container queries: `zoom: min(1, calc(100cqw / (var(--grid-columns) * 132px + 32px)))`. Grid shell uses `overflow-hidden`; no horizontal scrollbar or clipped right column.
3. **Titlebar and sidebar.** Device selector moved to the centre of the titlebar with a real-data green/red connection dot. Sidebar contains only existing surfaces: Devices label, Plugins popup, Profiles select (moved to header), Settings popup.
4. **Devices workspace.** Three-column main area: action library left (existing `ActionList`), device stage centre, 360px inspector right. Header holds profile select, brightness and Photo wall. Inspector has `Appearance` (existing `InstanceEditor` in new inline mode) and `Action` (existing `PropertyInspectorView` iframes) tabs, plus an empty state. No Advanced tab.
5. **Plugins screen.** Cards with icon, name, author, mono version, localised description, status badge with exit code, Restart / Settings-or-logs / Uninstall, Auto-restart, search and All/Installed/Running/Crashed chips.
6. **Design tokens.** `app-*` colour tokens, `--font-sans` (Inter with installed fallbacks) and `--font-mono` (JetBrains Mono with installed fallbacks). Dialogs use 12px radius, inline panels 8px, spacing on a 4px scale. Settings, Profiles, popups, context menus and tooltips follow the tokens.
7. **Accessibility.** 2px accent `:focus-visible` ring, disabled/input hover/active states, shared `.empty-state` component class.
8. **Performance.** No blur or backdrop-filter exists in the codebase. Removed cheap-but-unnecessary colour transitions from long action/plugin lists and reduced the floating editor shadow.

## Rolled back

Nothing was rolled back: every step passed `deno task check` on the first attempt. Obsolete fade-mask CSS was removed when horizontal scrolling was eliminated. Pre-existing Chinese/Japanese locale option labels in `SettingsView.svelte` were intentionally left untouched; they are language names, not redesign content.

## Differences from the mockups and why

- **No Backups screen**, pages `1-5`, dummy dial row, fake version, `Update` button or `key_id` - none exist in the real app.
- **No Updates filter**, Disable/Enable buttons, `Open plugin folder` button or invented error text - no corresponding data or commands.
- **No Advanced tab** - no real data to show there.
- **No gradient/image background modes** - profile state only has a solid background colour.
- **Grid is 6x3 with 5 screen columns** from the real device, not the mockup's 5x3. Photo wall defaults to the real screen column count.
- **Action list stays visible** on the left instead of the mockup's hidden strip, because it is a real part of the app and must keep working.
- **Inter / JetBrains Mono are not bundled** (no new dependencies allowed). They are declared first with installed fallbacks; the webview will use the fallback families unless the fonts are present on the system.

## Not verified visually

No GUI was launched, so nothing below was observed in the actual Tauri webview:

- CSS container-query `zoom` with length division inside `calc()` in the Tauri webview.
- Centring of the device selector in the titlebar with unequal left/right header content.
- 360px inspector: inline `InstanceEditor` width, iframe property inspector height, tab switching keeping PI sessions alive.
- Focus ring visibility, hover/pressed/disabled states and AA contrast in the real renderer.
- Photo wall slider, preview outlines and Apply/Reset on hardware.
- Plugin status colours, auto-restart toggle and restart button in the redesigned cards.
- Drag-and-drop of actions onto keys and the plugin property inspector iframe after a tab switch.

## Check this first with eyes

1. Launch the app with Mirabox HSV293S connected; confirm all 5 screen columns fit without horizontal scroll and the 6th column of screenless buttons is not clipped.
2. Open Photo wall: default `Columns with screens` should be 5, preview should show 5x3 tiles, Apply should touch only those 15 positions, Reset should restore them.
3. Select a button: Appearance tab should show the editor, Action tab should show the plugin property inspector; switch tabs a few times.
4. Open Plugins: status badge, exit code, Auto-restart and Restart must work as before.
5. Check keyboard focus rings on the device selector, filter chips and inspector tabs.
