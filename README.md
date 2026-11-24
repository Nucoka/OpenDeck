# OpenDeck (fork)

This is a fork of [nekename/OpenDeck](https://github.com/nekename/OpenDeck).

Differences from upstream:

- **Photo wall**: one image spread across the button grid of the current device, with a preview, Cover/Fit modes, a gap setting, and reset.

This fork is built from source only; no release artifacts are published here.

![Main menu](.github/readme/mainmenu.png)
[More screenshots](#showcase)

> [!NOTE]
> Screenshots are still from the original project and will be updated.

OpenDeck is a desktop application for using stream controller devices like the Elgato Stream Deck on Linux, Windows, and macOS. OpenDeck supports plugins made for the original Stream Deck SDK, allowing many plugins made for the Elgato software ecosystem to be used, or the [OpenAction](https://openaction.amankhanna.me/) API.

Only Elgato hardware is officially supported, but plugins are available for support for other hardware vendors.

> [!TIP]
> No Stream Deck in front of you? Use OpenDeck with [Tacto](https://tacto.live/) to turn any smartphone into one!

Special thanks to [nekename](https://github.com/nekename) for creating OpenDeck, and to the developers of [Tauri](https://github.com/tauri-apps/tauri), the [elgato-streamdeck](https://github.com/OpenActionAPI/rust-elgato-streamdeck) Rust library, and [Phosphor Icons](https://phosphoricons.com/).

### Why use OpenDeck?

- **Stream Deck plugins**: OpenDeck supports the majority of the Stream Deck plugins that users of the Elgato ecosystem are already familiar with, unlike other third-party softwares which are much more limited (e.g. streamdeck-ui, StreamController, Boatswain etc).
- **Cross-platform**: OpenDeck supports Linux alongside Windows and macOS. macOS users also benefit from switching from the first-party Elgato software as OpenDeck can run plugins only built for Windows on Linux and macOS thanks to Wine. Additionally, profile files are easily moveable between platforms with no changes to them necessary.
- **Feature-packed**: From Multi Actions and Toggle Actions to switching profiles when you switch apps and brightness control, OpenDeck has all the features you'd expect from stream controller software.
- **Open source**: OpenDeck source code is licensed under the GNU General Public License, allowing anyone to view it and improve it for feature, stability, privacy or security reasons. [Most plugins are open-source, too.](https://marketplace.rivul.us/)
- **Written in Rust**: The Rust programming language, which OpenDeck is built with alongside TypeScript, is known for its performance, safety and resulting code quality.

## Photo wall

Open the photo wall from the button in the top bar next to Settings. Choose an image or drop it onto the picker, then adjust the mode, gap, background colour (used by Fit), and label visibility. The preview shows how the image will be cut into the current device's button grid, with outlines marking individual buttons.

Press Apply to write each piece to its button in order; progress is shown in the dialog. Cancel stops after the current button. Empty buttons receive a no-op tile action so the image stays continuous. Reset removes those tiles and restores the previous images, settings, and active states.

Reset works only until the application is restarted: the previous state is kept in memory only. After a restart, restore the profile manually from a backup.

## Support

### How do I...?

To edit an action's settings, left-click on it to display its _property inspector_. To remove an action, right-click on it and choose "Delete" from the context menu.

To edit an action's appearance, right-click on it and select "Edit" from the context menu. You can then customise the image and text for each of its states. Left-click on the image to choose an image from your filesystem or right-click on the image to reset it to the plugin-provided default.

To select another device, or to switch profiles, use the dropdowns in the top right corner. You can organise profiles into folders by prefixing the profile name with the folder name and a forward slash. You can also configure automatically switching to a profile when a specific application's window is active.

To change other options, open Settings. From here, you can also view information about your version of OpenDeck or open the configuration and log directories. To add or remove plugins, visit the Plugins tab.

### Troubleshooting

- Ensure you are running the latest version of OpenDeck, as well as recent versions of related software (e.g. Spotify or OBS).
- Check the [FAQ](https://github.com/nekename/OpenDeck/wiki/0.-FAQ) and [GitHub Issues](https://github.com/nekename/OpenDeck/issues) to see if there's a fix for your problem already.
- Check the OpenDeck log file for any important messages. This file should be included with any support request.
  - You can also run OpenDeck from the terminal to see the logs directly if it's easier than finding the log file or if the log file is empty or missing details.
  - For issues with plugins, you can also check the plugin's logs (in the same folder, sometimes as well as a file named `plugin.log` or similar in the plugin's own folder).
  - The log directory can be opened from the settings page of OpenDeck, or alternatively located manually at the paths below:
    - Linux: `~/.local/share/opendeck/logs/`
    - Flatpak: `~/.var/app/me.amankhanna.opendeck/data/opendeck/logs/`
    - Windows: `%appdata%\opendeck\logs\`
    - macOS: `~/Library/Logs/opendeck/`
- When trying to run compiled plugins built for Windows on Linux or macOS, please ensure you have the latest version of Wine (and Wine Mono) installed on your system.
- If your device isn't showing up, ensure you have the correct permissions to access it (e.g. on Linux, installing udev subsystem rules and restarting your system), and that you have restarted OpenDeck since connecting it.

### Support forums

- [Discord](https://discord.gg/26Nf8rHvaj)
- [Matrix](https://matrix.to/#/#opendeck:matrix.org)
- [GitHub Issues](https://github.com/nekename/OpenDeck/issues)

## Building from source

> [!TIP]
> The development guide for agents present in [AGENTS.md](AGENTS.md) also serves as a useful introduction to the codebase for humans.

You'll need to ensure that all of the [prerequisites for building a Tauri application](https://tauri.app/start/prerequisites) are satisfied to build OpenDeck, as well as making sure that [Deno](https://deno.com/) is installed. On Linux, you'll also need `libudev` and `libdbus` installed for your distribution. After running `deno install`, you can use `deno task tauri dev` and `deno task tauri build` to work with OpenDeck.

Before each commit, please ensure that all of the following are completed:

1. Rust code has been linted using `cargo clippy` and it discovers no violations
2. Rust code has been formatted using `cargo fmt`
3. TypeScript code has been checked using `deno check` and linted using `deno lint` and they discover no violations
4. Svelte code has been linted using `deno task check` and it discovers no violations
5. Frontend code has been formatted using `deno task format`

When submitting contributions, please adhere to the [Conventional Commits specification](https://conventionalcommits.org/) for commit messages. You will also need to [sign your commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits). Feel free to reach out on the support channels above for guidance when contributing!

## License

OpenDeck is licensed under the GNU General Public License version 3.0 or later. For more details, see the LICENSE.md file.

## Showcase

![Main menu](.github/readme/mainmenu.png)
![Multi action](.github/readme/multiaction.png)
![Plugins](.github/readme/plugins.png)
![Profiles](.github/readme/profiles.png)
