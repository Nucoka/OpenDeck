# Redesign v2 log

Branch: `redesign-v2`. Old design preserved in tag `redesign-v1`.

Decisions:

- No `design-v2/` directory or `tokens.md` exists in the workspace, so the textual specification in the task is the source of truth.
- No Inter or JetBrains Mono font files exist in `static/fonts`. Per the rules, no runtime network fonts and no new dependencies: UI uses the system/Inter fallback stack, headings use bundled Archivo Black (closest geometric display face), monospaced values use Courier Prime with a system mono fallback. This is a documented deviation, not a missing feature.
- Appearance settings (accent, glass blur, background motion) are stored in `localStorage` under `opendeck.v2.appearance` via `src/lib/appearance.ts`. No Rust or app settings-store changes.
- Popups become real pages through an `inline` prop on `Popup`, `PluginManager`, `SettingsView` and `PhotoWall`; their internal logic and invoke calls are unchanged.
- The Windows system titlebar is untouched.

## Step 1 - tokens, fonts, shell, pages

Done: new colour tokens (`app-bg`, `app-panel`, `app-card`, `app-border`, `app-text`, `app-muted`, configurable accent), font stacks, body text 13px, static radial accent glow, glass blur on sidebar and dialogs, 190px sidebar with a light active pill, five sidebar pages (Devices, Plugins, Photo wall, Settings, Profile), and page headings with subtitles. Plugins, Photo wall and Settings now render as pages in the main area; Profile is a placeholder until step 6. Profile selection was removed from the header. When the Actions panel is open on Devices, the sidebar collapses to 64px icons.

Not verified: no GUI run, so visual similarity to the mockups, contrast and blur appearance were not observed.

Symbol check: the required grep matches existing `translations/ko.json` content and language option labels in `SettingsView`. Those are localization data, not new text added by the redesign; no Chinese/Arabic/Korean characters were added to source comments or UI strings by this step.

## Step 2 - Devices and Actions panel

Done: grid gap is 8px with a 20px card, empty keypad cells show a dashed border and plus, selected keys get a 2px accent ring plus glow, and a footer shows `device · profile` and `Key N of M`. Inspector tabs are named Key / Action; the Key tab now has Preview, Title and Image sections with a Choose file button and the plugin-icon hint. The Actions panel is a 270px drawer with search, category chips built from real category names without duplicates, compact rows and the drag hint. Clicking an action does not assign it; only the existing drag-and-drop handler assigns actions to keys.

Not verified: no GUI run, so panel behaviour, focus and visual spacing were not observed.

## Step 3 - Plugins page

Done: the page now has Installed / Store tabs, a single search field and an Install from file button in the header. Installed plugins render as compact rows with 36px icons, status badges with exit codes, Auto-restart toggles, Restart and Uninstall (non-builtin only). Store plugins use the same row style without status badges. Existing restart, auto-restart, uninstall, file install, catalogue and property-inspector calls are unchanged.

Not verified: no GUI run, so row wrapping and tab switching were not observed.

## Step 4 - Photo wall page

Done: two-column page with dropzone, preview card, selected-image thumbnail and eight gradient presets. Right panel holds Mode, Columns with screens (mono value), Gap, background for Fit, Hide labels, and an always-visible "Apply to first N keys" field where empty means All. Footer has Reset (secondary) and Apply to deck (primary). Gradient presets feed a canvas-generated image into the existing `slicePhoto` path; Apply/Reset logic and invoke calls are unchanged.

Not verified: no GUI run, and Apply/Reset were intentionally not clicked.

## Step 5 - Settings page

Done: two-column card layout with rows (name + description left, control right). Left column: General and Behavior; right column: Appearance, Data and Advanced. All existing app settings remain, including the numeric Sleep value and the platform-conditional separate Wine prefixes, which was kept in Advanced. Accent, Glass blur and Background motion are stored in `localStorage` via `appearance.ts`; no Light theme was added. Data actions keep their existing backup, restore and folder-opening calls.

Not verified: no GUI run, so control alignment and localStorage persistence were not observed.

## Step 6 - Profile page

Done: device summary card with real name, id, key count, unique plugin count and profile count; profile cards with a cheap coloured mini-grid, active state, and click-to-switch through the existing `ProfileManager.setProfile` flow; a New profile card using the same switching/creation path; and a Manage profiles block reusing the existing ProfileManager popup for rename, delete and application profiles. Mini-previews use action presence only, not images. No firmware, serial number or fake metrics were added. No Tauri command signatures or handlers were changed; the page calls the same existing commands through the existing component API.

Not verified: no GUI run, so switching behaviour and card layout were not observed.
