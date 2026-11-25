# QA report - redesign branch

Date: 2026-11-25. No application code was changed during this check. Only this report and the screenshots were added.

## Launch

- Killed the previous `opendeck.exe` and old dev processes. One stale `opendeck.exe` (PID 23504, 44 KB) from an earlier session could not be terminated (access denied); it is not part of this run.
- `deno task tauri dev` started successfully. Vite was ready on `127.0.0.1:5173`, cargo finished in 1m04s, and the window appeared.
- Window identity from UI Automation: `OpenDeck v2.14.2 (03d9874) on x86_64-pc-windows-msvc`.

## Screenshots

All screenshots were captured with `PrintWindow(hwnd, hdc, PW_RENDERFULLCONTENT)` directly from the OpenDeck window at its exact window rectangle. No desktop, taskbar or other program is present in the images.

| View                                   | Path                                     | Size     |
| -------------------------------------- | ---------------------------------------- | -------- |
| Devices, 1280px window                 | `design/screenshots/devices-1280.png`    | 1280x800 |
| Devices, 960px window                  | `design/screenshots/devices-960.png`     | 960x700  |
| Plugins                                | `design/screenshots/plugins-1280.png`    | 1280x800 |
| Photo wall                             | `design/screenshots/photo-wall-1280.png` | 1280x800 |
| Settings                               | `design/screenshots/settings-1280.png`   | 1280x800 |
| Inspector, Appearance selected         | `design/screenshots/inspector-960.png`   | 960x700  |
| Focus ring after keyboard Tab          | `design/screenshots/focus-tab-960.png`   | 960x700  |
| Focus ring after programmatic SetFocus | `design/screenshots/focus-960.png`       | 960x700  |

## What was checked

- **Titlebar at 960px:** UI Automation bounding boxes - `Device` combobox x=580..717, brightness text x=773..836, brightness slider x=843..924, `Photo wall` button x=921..1008. The device chip does not overlap the brightness group (56px gap). The slider and button bounding boxes touch at x=921..924, which is a visual adjacency, not an text overlap.
- **Device grid:** at 960px the grid element spans x=488..672 and the right inspector starts at screen x≈680, so the right column is not clipped. The shell stays `overflow-hidden`, so no horizontal scrollbar.
- **Tile size:** UI Automation reports a 124px layout cell, but CSS zoom reduces it to about 30px at 960px and about 83px at 1280px. Tiles cannot reach 132px in normal windows.
- **Photo wall:** opened via UI Automation. Slider `Columns with screens` value 6 (current device columns), `Gap` value 0. Apply and Reset were not clicked.
- **Plugins:** UI Automation found the chips `All`, `Installed`, `Running`, `Crashed` and the card buttons `Restart`, `Uninstall`, `Settings`.
- **Settings:** section headings `Display`, `Behavior`, `System`, `Config` are present. The original project's star/sponsor footer is absent.
- **Inspector:** a grid cell was clicked with `SendMessage(WM_LBUTTONDOWN/UP)` to the WebView child window (no focus stealing). The inline editor appeared: comboboxes `State`, `Alignment`, `Font`. Switching to the `Action` tab removed those comboboxes and left one Document (plugin property-inspector iframe).
- **Focus ring:** after sending keyboard Tab through `SendMessage`, the focused element was the `Action` button and 106 accent-coloured pixels were found around its rectangle in `focus-tab-960.png`. Programmatic `SetFocus` alone did not show the ring, which matches `:focus-visible` behaviour.
- **Logs:** `C:\Users\NUCOKA\AppData\Local\opendeck\logs\OpenDeck.log` contains plugin registrations and no matches for `panic`, `error`, `failed` or `WARN`. Both plugin logs are 0 bytes. Dev console shows one harmless `[404] GET /favicon.ico`.
- **Cleanup:** the application, WebView2 processes, dev server and both plugin processes (`opendeck-starterpack.exe`, `opendeck-akp153-win.exe`) were terminated. No plugin processes remain.

## Comparison with the mockups

The mockups are 2560x1600 (2x) versions of a 1280x800 window. Programmatic colour sampling: mockup background `(20,21,27)`, app background `(17,17,19)`; mockup sidebar `(20,21,27)`, app sidebar `(23,23,25)`. Structure matches closely: left sidebar, centred device selector chip, grid in a panel, right inspector, plugin cards. Differences intentionally kept: no Backups screen, no fake version/Update, no dials, no Updates filter, no profile tabs.

## Found issues

1. **Tiles cannot reach 132px.** `src/components/DeviceView.svelte:261` scales the whole grid with `zoom: min(1, calc(100cqw / (var(--grid-columns) * 132px + 32px)))`, while the fixed side panels in `src/routes/+page.svelte:122` and `src/routes/+page.svelte:142` leave only ~520px of centre width at 1280px and ~200px at 960px. Result: ~83px tiles at 1280px and ~30px at 960px. Reaching 132px needs roughly a 1584px window.
2. **Pre-existing stale process.** `opendeck.exe` PID 23504 from 2026-11-24 cannot be killed (access denied). It is not a plugin and is not from this run.
3. **Harmless dev-console 404** for `/favicon.ico`; no impact on the UI.

## Not verified

- Visual inspection by eye: images are not rendered in this session. Only geometry, UI Automation and pixel colours were checked.
- Photo wall persistence across an actual application restart: the value was read (6) but not changed, because changing settings was forbidden. WebView2 storage lives under `%LOCALAPPDATA%\opendeck\EBWebView`, not under `%APPDATA%\opendeck`, so the localStorage value was not inspected directly.
- Apply/Reset, drag-and-drop, plugin property-inspector content, hover/pressed states and real WCAG contrast ratios were not exercised, per the task restrictions.

## Follow-up - CSS grid layout check (same day)

After replacing CSS zoom with a CSS Grid, the app was rebuilt and launched again. UI Automation bounding rectangles of the 18 keypad cells were measured at three window widths. Rectangles were filtered to 6 cells per row with names starting `Key A`/`Key B`/`Key C` and width below 200px, excluding the combined row containers.

| Window width | Cells per row | Tile width | Horizontal gap | Vertical gap | Right-edge delta across rows | Result     |
| ------------ | ------------- | ---------- | -------------- | ------------ | ---------------------------- | ---------- |
| 1280         | 6             | 95px       | 13px           | 14px         | 0,0,0,0,0,0                  | no overlap |
| 1100         | 6             | 123px      | 13/14px        | 14px         | 0,0,0,0,0,0                  | no overlap |
| 960          | 6             | 110px      | 14px           | 14px         | 0,0,0,0,0,0                  | no overlap |

Positive gaps and identical column right edges mean tiles do not intersect and the sixth column is aligned. Minimum tile sizes (95px at 1280, 110px at 960) exceed the 72px/56px targets; maximum stays below 132px. `zoom` and tile `transform: scale()` are gone from the device grid.

The action library starts collapsed below 1400px, the sidebar collapses to 64px below 1100px, and the inspector uses `clamp(280px, 24vw, 360px)` with an overlay position below 1100px. No Apply/Reset was clicked and no profile or setting was changed. The app, WebView, dev server and plugin processes were terminated afterwards; the pre-existing zombie `opendeck.exe` PID 23504 remains.
