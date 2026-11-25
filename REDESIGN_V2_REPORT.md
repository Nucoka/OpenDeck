# Redesign v2 report

Branch: `redesign-v2`. Previous design preserved in tag `redesign-v1`. No Rust code, handlers, stores or Tauri command signatures were changed. The application was not launched, `%appdata%` was not touched, and Photo wall Apply/Reset were not pressed.

## What was done

1. **Tokens, fonts, shell, pages.** New dark palette (`bg #0B0B0E`, sidebar `#111113`, card `#141416`, border `#26262B`, text `#F2F2F4`, muted `#8B8B94`), configurable accent, 12/18/20px radii, 4px spacing scale, static accent glow, glass blur on sidebar/dialogs. Sidebar with five pages; Plugins, Photo wall and Settings are now pages in the main area instead of popups. Profile selection moved off the header.
2. **Devices + Actions.** Dynamic CSS Grid from `device.rows/columns` with 8px gaps, no zoom or scale; empty keys show a dashed border and plus; selected keys get a 2px accent ring and glow; footer shows device/profile and key count. Inspector tabs Key / Action with Preview, Title, Image, Choose file and the plugin-icon hint. 270px Actions drawer with search, category chips from real categories, compact rows and a drag hint; clicking an action does not assign it.
3. **Plugins.** Installed / Store tabs, search, Install from file. Compact rows with 36px icons, status badges with exit codes, Auto-restart, Restart and Uninstall (non-builtin only). Store rows have no status badge.
4. **Photo wall.** Dropzone, preview card with grid lines for screen columns only, selected-image thumbnail, eight gradient presets feeding `slicePhoto`. Right panel: Mode, Columns with screens (mono value), Gap, Fit background, Hide labels, Apply to first N keys (empty = All), reset warning. Reset (secondary) and Apply to deck (primary).
5. **Settings.** Two-column cards with name/description/control rows. General, Behavior, Appearance, Data and Advanced sections. All existing options remain, including numeric Sleep and platform-conditional separate Wine prefixes. Accent, Glass blur and Background motion persist in `localStorage`. No Light theme.
6. **Profile.** Device card with real id and counters (keys set, unique plugins, profiles), profile cards with cheap coloured mini-grids, click-to-switch through the existing ProfileManager flow, New profile card, and a Manage profiles block with the existing rename/delete/application-profile popup.
7. **Polish.** Accent focus ring on all interactive elements, skip link, `aria-current` on sidebar items, unified empty states, hover/pressed/disabled states.

## Rolled back

Nothing was rolled back: every step passed `deno task check` on the first or second attempt. No step was reverted.

## Differences from the mockups and why

- No `design-v2/` directory or `tokens.md` exists in the workspace, so the textual specification was used as the source of truth.
- No Inter or JetBrains Mono font files exist in `static/fonts`. Per the rules, no new dependencies and no runtime network fonts: the UI uses the system/Inter fallback stack, headings use bundled Archivo Black, monospaced values use Courier Prime with a system mono fallback.
- No Windows titlebar redesign: the system titlebar was left untouched as required.
- Plugin enable/disable and Updates filter were not added: no such feature or data exists.
- Profile mini-previews use coloured blocks from action presence, not captured images, to stay cheap.
- No firmware, serial number, fake version or invented metrics were added.

## Not verified visually

No GUI was launched. The following were checked in source only and need human eyes:

- overall similarity to the mockups, exact spacing and typography;
- actual blur strength and radial glow intensity;
- focus order and screen-reader labels;
- real WCAG contrast measurements;
- drag-and-drop of actions onto keys after the new drawer styling;
- Photo wall Apply/Reset behaviour after the page conversion;
- plugin status updates and auto-restart toggles in the new row layout;
- profile switching and application-profile rules in the new page.

## Look at this first

1. Open each of the five sidebar pages and check that nothing overlaps at 1280 and 960 window widths.
2. Devices: drag an action from the Actions drawer onto a key, then open Key and Action inspector tabs.
3. Photo wall: choose an image, try a gradient preset, verify preview lines only cover screen columns, then Apply/Reset.
4. Plugins: check status badge, Auto-restart, Restart, Uninstall, Install from file and the Store tab.
5. Settings: change accent, blur and motion, restart the app and confirm they persist.
6. Profile: switch profiles and open the Manage profiles popup.
