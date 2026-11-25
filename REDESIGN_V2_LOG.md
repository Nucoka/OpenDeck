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
