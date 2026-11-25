<script lang="ts">
	import ArrowClockwise from "phosphor-svelte/lib/ArrowClockwise";
	import ArrowSquareOut from "phosphor-svelte/lib/ArrowSquareOut";
	import CloudArrowDown from "phosphor-svelte/lib/CloudArrowDown";
	import FileArrowUp from "phosphor-svelte/lib/FileArrowUp";
	import Gear from "phosphor-svelte/lib/Gear";
	import MagnifyingGlass from "phosphor-svelte/lib/MagnifyingGlass";
	import Robot from "phosphor-svelte/lib/Robot";
	import Trash from "phosphor-svelte/lib/Trash";
	import WarningCircle from "phosphor-svelte/lib/WarningCircle";
	import ListedPlugin from "./ListedPlugin.svelte";
	import PluginDetails from "./PluginDetails.svelte";
	import Popup from "./Popup.svelte";
	import Tooltip from "./Tooltip.svelte";

	import { t } from "$lib/i18n";
	import { getWebserverUrl } from "$lib/ports";
	import { localisations, settings } from "$lib/settings";
	import { actionList, deviceSelector, PRODUCT_NAME } from "$lib/singletons";

	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
	import { ask, message, open } from "@tauri-apps/plugin-dialog";
	import { onDestroy } from "svelte";

	// @ts-expect-error
	const fetch = window.fetchNative ?? window.fetch;

	type PluginStatus = {
		id: string;
		status: string;
		attempts: number;
		exit_code: number | null;
	};

	let showPopup: boolean;
	export let inline: boolean = false;
	let statuses: { [id: string]: PluginStatus } = {};
	let unlistenStatus: (() => void) | undefined;

	async function listenStatus() {
		if (unlistenStatus) return;
		unlistenStatus = await listen<PluginStatus>("plugin_status", ({ payload }) => {
			statuses = { ...statuses, [payload.id]: payload };
		});
	}

	function stopStatus() {
		unlistenStatus?.();
		unlistenStatus = undefined;
	}

	$: if (showPopup) void listenStatus();
	else stopStatus();

	onDestroy(stopStatus);

	setInterval(async () => {
		if (showPopup) {
			installed = await invoke("list_plugins");
			const merged = { ...statuses };
			for (const plugin of installed) {
				if (!merged[plugin.id] && plugin.status) {
					merged[plugin.id] = { id: plugin.id, status: plugin.status, attempts: 0, exit_code: null };
				}
			}
			statuses = merged;
		}
	}, 1e3);

	async function installPlugin(name: string, url: string | null, file: string | null, fallback_id: string | null) {
		if (
			!file &&
			!(await ask($t("plugin_manager.install.prompt"), {
				title: $t("plugin_manager.install.title", { name }),
				okLabel: $t("dialog.yes"),
				cancelLabel: $t("dialog.no"),
			}))
		)
			return;
		try {
			await invoke("install_plugin", { url, file, fallback_id });
			message($t("plugin_manager.install.success", { name }), {
				title: $t("plugin_manager.install.success.title", { name }),
				buttons: { ok: $t("dialog.ok") },
			});
			$actionList?.reload();
			installed = await invoke("list_plugins");
		} catch (error: any) {
			message(error, { title: $t("plugin_manager.install.error", { name }), buttons: { ok: $t("dialog.ok") } });
		}
	}

	let choices: any[] | undefined;
	let choice: number;
	let finishChoice = (_: unknown) => {};
	let cancelChoice = () => {};
	async function chooseAsset(assets: any[]): Promise<any> {
		choices = assets;
		try {
			await new Promise((resolve, reject) => {
				finishChoice = resolve;
				cancelChoice = reject;
			});
		} catch (e) {
			throw e;
		} finally {
			choices = undefined;
			finishChoice = (_: unknown) => {};
			cancelChoice = () => {};
		}
		return assets[choice];
	}

	let openDetailsView: string | null = null;
	type GitHubPlugin = {
		name: string;
		author: string;
		repository: string;
		download_url: string | undefined;
		flags?: {
			ai_assisted?: boolean;
		};
	};
	async function installPluginGitHub(id: string, plugin: GitHubPlugin) {
		if (plugin.download_url) {
			await installPlugin(plugin.name, plugin.download_url, null, id);
			return;
		}

		let endpoint = new URL(plugin.repository);
		endpoint.hostname = "api." + endpoint.hostname;
		endpoint.pathname = "/repos" + endpoint.pathname + "/releases";

		let res;
		try {
			res = await (await fetch(endpoint)).json();
		} catch (error: any) {
			message(error, { title: $t("plugin_manager.install.error", { name: plugin.name }), buttons: { ok: $t("dialog.ok") } });
			return;
		}

		let release = res[0];
		if (release.prerelease && res.find((r: any) => !r.prerelease)) release = res.find((r: any) => !r.prerelease);

		let assets = [];
		for (const asset of release.assets) {
			if (asset.name.toLowerCase().endsWith(".streamdeckplugin") || asset.name.toLowerCase().endsWith(".zip")) {
				assets.push(asset);
			}
		}
		let selected;
		if (assets.length == 1) selected = assets[0];
		else {
			try {
				selected = await chooseAsset(assets);
			} catch {
				return;
			}
		}

		await installPlugin(plugin.name, selected.browser_download_url, null, id);
	}

	async function installPluginElgato(plugin: any) {
		await installPlugin(plugin.name, `https://plugins.amankhanna.me/rezipped/${plugin.id}.zip`, null, plugin.id);
	}

	async function installPluginFile() {
		const path = await open({ multiple: false, directory: false });
		if (!path) return;
		await installPlugin(path.split(/[\/\\]/).at(-1) ?? path, null, path, null);
	}

	async function removePlugin(plugin: any) {
		if (
			!(await ask($t("plugin_manager.remove.prompt", { name: plugin.name }), {
				title: $t("plugin_manager.remove.title", { name: plugin.name }),
				okLabel: $t("dialog.yes"),
				cancelLabel: $t("dialog.no"),
			}))
		)
			return;
		try {
			await invoke("remove_plugin", { id: plugin.id });
			message($t("plugin_manager.remove.success", { name: plugin.name }), {
				title: $t("plugin_manager.remove.success.title", { name: plugin.name }),
				buttons: { ok: $t("dialog.ok") },
			});
			$actionList?.reload();
			$deviceSelector?.reloadProfiles();
			installed = await invoke("list_plugins");
		} catch (error: any) {
			message(error, { title: $t("plugin_manager.remove.error", { name: plugin.name }), buttons: { ok: $t("dialog.ok") } });
		}
	}

	async function isUpdateAvailable(plugin: any): Promise<string | false> {
		const id = plugin.id.endsWith(".sdPlugin") ? plugin.id.slice(0, -9) : plugin.id;
		const cataloguePlugin = plugins[id];
		if (!cataloguePlugin || cataloguePlugin.download_url) return false;

		try {
			const endpoint = new URL(cataloguePlugin.repository);
			endpoint.hostname = "api." + endpoint.hostname;
			endpoint.pathname = "/repos" + endpoint.pathname + "/releases/latest";

			const res = await fetch(endpoint);
			if (!res.ok) return false;
			const release = await res.json();

			const normalizeVersion = (v: string) => v.replace(/^v/, "").replace(/^(\d+\.\d+\.\d+)\.\d+$/, "$1");
			if (normalizeVersion(release.tag_name) != normalizeVersion(plugin.version)) {
				return release.tag_name.replace(/^v/, "");
			} else {
				return false;
			}
		} catch (error) {
			console.warn("Failed to check for plugin update:", error);
			return false;
		}
	}

	let installed: any[] = [];
	(async () => (installed = await invoke("list_plugins")))();

	async function restartPlugin(id: string) {
		await invoke("restart_plugin", { id });
		installed = await invoke("list_plugins");
	}

	async function setAutoRestart(id: string, enabled: boolean) {
		await invoke("set_plugin_auto_restart", { id, enabled });
		installed = await invoke("list_plugins");
	}

	let plugins: { [id: string]: GitHubPlugin };
	(async () => (plugins = await (await fetch("https://openactionapi.github.io/plugins/catalogue.json")).json()))();

	let showArchive: boolean = false;
	let archivePlugins: any[] | null = null;

	let availableUpdates: { [id: string]: string | false } = {};
	let checkedPlugins = new Set<string>();
	$: if (showPopup) {
		for (const plugin of installed) {
			if (!checkedPlugins.has(plugin.id)) {
				checkedPlugins.add(plugin.id);
				isUpdateAvailable(plugin).then((version) => (availableUpdates = { ...availableUpdates, [plugin.id]: version }));
			}
		}
	}

	let pluginVersions: { [id: string]: string } = {};
	$: for (const plugin of installed) {
		if (pluginVersions[plugin.id] != plugin.version) {
			checkedPlugins.delete(plugin.id);
			delete availableUpdates[plugin.id];
			availableUpdates = availableUpdates;
			pluginVersions[plugin.id] = plugin.version;
		}
	}

	let query: string = "";

	type PluginFilter = "all" | "installed" | "running" | "crashed";
	const filters: PluginFilter[] = ["all", "installed", "running", "crashed"];
	let pluginFilter: PluginFilter = "all";

	$: filteredInstalled = installed
		.filter((plugin) => {
			const lower = query.toLowerCase().trim();
			if (lower && !`${plugin.name} ${plugin.id}`.toLowerCase().includes(lower)) return false;
			const status = statuses[plugin.id]?.status ?? plugin.status ?? "running";
			if (pluginFilter === "installed") return !plugin.builtin;
			if (pluginFilter === "running") return status === "running";
			if (pluginFilter === "crashed") return status === "crashed" || status === "failed";
			return true;
		})
		.sort((a, b) => (a.builtin && !b.builtin ? -1 : b.builtin && !a.builtin ? 1 : a.id.localeCompare(b.id)));

	onOpenUrl((urls: string[]) => {
		if (!urls[0].includes("installPlugin/")) return;
		let id = urls[0].split("installPlugin/")[1];
		if (!plugins[id]) return;
		installPluginGitHub(id, plugins[id]);
	});
</script>

{#if !inline}
	<button class="side-control" on:click={() => (showPopup = true)}>
		{$t("plugin_manager.button")}
	</button>
{/if}

<svelte:window
	on:keydown={(event) => {
		if (event.key == "Escape") {
			if (choices) cancelChoice();
			else if (openDetailsView) openDetailsView = null;
			else showPopup = false;
		}
	}}
/>

<Popup show={inline || showPopup} {inline} label={$t("plugin_manager.title")}>
	<svelte:fragment slot="header">
		<button class="mr-2 my-1 float-right text-xl text-app-text" on:click={() => (showPopup = false)} aria-label={$t("settings.close")}>✕</button>
		<h2 class="m-2 font-semibold text-xl text-app-text">{$t("plugin_manager.title")}</h2>
	</svelte:fragment>

	<h2 class="mx-2 mb-2 mt-4 text-lg text-app-muted">{$t("plugin_manager.installed")}</h2>
	<div class="mx-2 mb-3 flex flex-row items-center gap-2 rounded-lg border border-app-border bg-app-panel px-2 py-2">
		<MagnifyingGlass size={16} class="text-app-muted" />
		<input
			bind:value={query}
			class="w-full bg-transparent p-1 text-sm text-app-text placeholder:text-app-muted"
			placeholder={$t("plugin_manager.search")}
			aria-label={$t("plugin_manager.search")}
			type="search"
			spellcheck="false"
		/>
	</div>
	<div class="mx-2 mb-3 flex flex-row flex-wrap gap-2">
		{#each filters as filter}
			<button
				class="rounded-full border border-app-border bg-app-elevated px-3 py-1 text-xs text-app-muted transition-colors"
				class:border-app-accent={pluginFilter === filter}
				class:text-app-accent={pluginFilter === filter}
				on:click={() => (pluginFilter = filter)}
				aria-pressed={pluginFilter === filter}
			>
				{$t(`plugin_manager.filter.${filter}`)}
			</button>
		{/each}
	</div>
	<div class="grid grid-cols-1 gap-1 md:grid-cols-2 lg:grid-cols-3">
		{#each filteredInstalled as plugin}
			<ListedPlugin
				icon={getWebserverUrl(plugin.icon)}
				name={$localisations && $localisations[plugin.id] && $localisations[plugin.id].Name ? $localisations[plugin.id].Name : plugin.name}
				subtitle={plugin.version}
				description={$localisations?.[plugin.id]?.Tooltip ?? ""}
				disconnected={!plugin.registered}
				action={() => {
					if ($settings?.developer) invoke("reload_plugin", { id: plugin.id });
					else removePlugin(plugin);
				}}
				actionLabel={$settings?.developer ? $t("plugin_manager.reload") : $t("plugin_manager.uninstall")}
				secondaryAction={!plugin.registered
					? () => invoke("open_log_directory")
					: plugin.has_settings_interface
						? () => invoke("show_settings_interface", { plugin: plugin.id })
						: undefined}
				secondaryActionLabel={!plugin.registered ? $t("plugin_manager.view_logs") : $t("plugin_manager.plugin_settings")}
				status={statuses[plugin.id]?.status ?? plugin.status ?? "running"}
				exitCode={statuses[plugin.id]?.exit_code ?? null}
				actionVisible={$settings?.developer || !plugin.builtin}
			>
				<svelte:fragment slot="subtitle">
					<span class="font-mono">{plugin.version}</span>
					{#if plugin.author}<span class="text-app-muted"> · {plugin.author}</span>{/if}
					{#if availableUpdates[plugin.id]}
						(<span class="text-app-warning">
							{$t("plugin_manager.available")}
							<button class="font-semibold underline" on:click={() => (openDetailsView = plugin.id.endsWith(".sdPlugin") ? plugin.id.slice(0, -9) : plugin.id)}>
								{availableUpdates[plugin.id]}
							</button></span
						>)
					{/if}
				</svelte:fragment>

				<svelte:fragment slot="secondary">
					{#if !plugin.registered}
						<WarningCircle size={16} class="text-app-warning" />
					{:else if plugin.has_settings_interface}
						<Gear size={16} class="text-app-success" />
					{/if}
				</svelte:fragment>

				<svelte:fragment slot="controls">
					<button
						class="flex flex-row items-center justify-center gap-1 rounded-lg border border-app-border bg-app-elevated px-2 py-1 text-xs text-app-text hover:text-app-accent"
						on:click={() => void restartPlugin(plugin.id)}
						aria-label={$t("plugin_manager.restart")}
						title={$t("plugin_manager.restart")}
					>
						<ArrowClockwise size={14} />
						{$t("plugin_manager.restart")}
					</button>
					<label class="flex flex-row items-center space-x-1 text-xs text-app-muted" title={$t("plugin_manager.auto_restart")}>
						<input type="checkbox" checked={plugin.auto_restart} on:change={(event) => void setAutoRestart(plugin.id, event.currentTarget.checked)} />
						{$t("plugin_manager.auto_restart")}
					</label>
				</svelte:fragment>

				{#if $settings?.developer}
					<ArrowClockwise size={16} class="text-app-muted" />
				{:else if !plugin.builtin}
					<Trash size={16} class="text-app-danger" />
				{/if}
			</ListedPlugin>
		{/each}
	</div>

	<div class="flex flex-row justify-between items-center mx-2 mt-6 mb-2">
		<h2 class="text-lg text-app-muted">{$t("plugin_manager.store")}</h2>
		<button
			class="flex flex-row items-center mt-2 px-1 py-0.5 text-sm text-app-text bg-app-panel hover:bg-app-border transition-colors border border-app-border rounded-lg"
			on:click={installPluginFile}
		>
			<FileArrowUp />
			<span class="ml-1">{$t("plugin_manager.install_from_file")}</span>
		</button>
	</div>

	<div class="flex flex-row items-center mx-2 my-4 p-3 space-x-2 bg-app-warning/10 border-l-4 border-app-warning rounded">
		<WarningCircle size="20" class="mt-0.5 text-app-warning" />
		<div class="text-sm text-app-warning-soft">
			{$t("plugin_manager.warning", { PRODUCT_NAME })}
		</div>
	</div>

	{#if !plugins}
		<h2 class="mx-2 mt-6 mb-2 text-md text-app-muted">{$t("plugin_manager.loading.open_source")}</h2>
	{:else}
		<div class="flex flex-row items-center ml-2 mt-6 mb-2 space-x-2">
			<h2 class="font-semibold text-md text-app-muted">{$t("plugin_manager.open_source")}</h2>
			<Tooltip>{$t("plugin_manager.open_source.tooltip")}</Tooltip>
		</div>
		<div class="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
			{#each Object.entries(plugins) as [id, plugin]}
				<ListedPlugin
					icon="https://openactionapi.github.io/plugins/icons/{id}.png"
					name={plugin.name}
					subtitle={plugin.author}
					hidden={!plugin.name.toLowerCase().includes(query.toLowerCase())}
					action={() => (openDetailsView = id)}
					actionLabel={$t("plugin_manager.view_details")}
				>
					<svelte:fragment slot="subtitle">
						<span>
							{plugin.author}
							{#if plugin.flags?.ai_assisted}
								<Robot size="1em" class="inline align-text-bottom text-app-warning" />
							{/if}
						</span>
					</svelte:fragment>

					<ArrowSquareOut size="24" class="text-app-muted" />
				</ListedPlugin>
			{/each}
		</div>
	{/if}

	<div class="flex flex-row items-center mt-6 mb-2">
		<h2 class="mx-2 font-semibold text-md text-app-muted">{$t("plugin_manager.elgato")}</h2>
		<Tooltip>{$t("plugin_manager.elgato.tooltip")}</Tooltip>
	</div>
	{#if !showArchive}
		<button
			class="ml-2 mt-2 mb-2 px-2 py-1 text-sm text-app-text bg-app-panel hover:bg-app-border transition-colors border border-app-border rounded-lg"
			on:click={async () => {
				showArchive = true;
				archivePlugins = await (await fetch("https://plugins.amankhanna.me/catalogue.json")).json();
			}}
		>
			{$t("plugin_manager.elgato.load")}
		</button>
	{:else if !archivePlugins}
		<h2 class="mx-2 mt-4 mb-2 text-md text-app-muted">{$t("plugin_manager.loading.elgato")}</h2>
	{:else}
		<div class="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
			{#each archivePlugins as plugin}
				<ListedPlugin
					icon="https://plugins.amankhanna.me/icons/{plugin.id}.png"
					name={plugin.name}
					subtitle={plugin.author}
					hidden={!plugin.name.toLowerCase().includes(query.toLowerCase())}
					action={() => installPluginElgato(plugin)}
					actionLabel={$t("plugin_details.install")}
				>
					<CloudArrowDown size="24" class="text-app-muted" />
				</ListedPlugin>
			{/each}
		</div>
	{/if}

	{#if "Tacto Connect".toLowerCase().includes(query.toLowerCase())}
		<div class="flex flex-row items-center mt-6 mb-2">
			<h2 class="mx-2 font-semibold text-md text-app-muted">Tacto</h2>
			<Tooltip>{$t("plugin_manager.tacto.tooltip")}</Tooltip>
		</div>
		<div class="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
			<ListedPlugin
				icon="https://tacto.live/icon-192.png"
				name="Tacto Connect"
				subtitle="Rivulus"
				action={() => {
					installPluginGitHub("us.rivul.tacto", {
						name: "Tacto Connect",
						author: "Rivulus",
						repository: "https://github.com/RivulusLive/tacto-desktop",
						download_url: undefined,
					});
				}}
				actionLabel={$t("plugin_details.install")}
				secondaryAction={() => window.open("https://tacto.live")}
				secondaryActionLabel={$t("plugin_manager.visit_website")}
			>
				<svelte:fragment slot="secondary">
					<ArrowSquareOut size="24" class="text-app-muted" />
				</svelte:fragment>

				<CloudArrowDown size="24" class="mt-2 text-app-muted" />
			</ListedPlugin>
		</div>
	{/if}
</Popup>

{#if openDetailsView}
	<PluginDetails
		id={openDetailsView}
		details={plugins[openDetailsView]}
		install={() => {
			// @ts-expect-error
			installPluginGitHub(openDetailsView, plugins[openDetailsView]);
		}}
		close={() => (openDetailsView = null)}
	/>
{/if}

{#if choices}
	<div
		class="fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 mt-2 p-2 w-96 text-xs text-app-text bg-app-panel border border-app-border rounded-lg z-40"
	>
		<h3 class="mb-2 font-semibold text-lg text-center">{$t("plugin_manager.choose_asset")}</h3>
		<div class="select-wrapper">
			<select class="w-full bg-app-elevated!" bind:value={choice} aria-label={$t("plugin_manager.choose_asset.label")}>
				{#each choices as choice, i}
					<option value={i}>{choice.name}</option>
				{/each}
			</select>
		</div>
		<button
			class="mt-2 p-1 w-full text-sm text-app-text bg-app-elevated hover:bg-app-panel transition-colors border border-app-border rounded-lg"
			on:click={finishChoice}
		>
			{$t("plugin_details.install")}
		</button>
	</div>
{/if}
