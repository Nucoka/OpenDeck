<script lang="ts">
	import type { ActionInstance } from "$lib/ActionInstance";
	import type { DeviceInfo } from "$lib/DeviceInfo";
	import type { Profile } from "$lib/Profile";

	import { applyAppearance, loadAppearance } from "$lib/appearance";
	import { t } from "$lib/i18n";
	import { initPortBase } from "$lib/ports";
	import { inspectedInstance, inspectedParentAction } from "$lib/propertyInspector";
	import { actionList, deviceSelector } from "$lib/singletons";

	import Diamond from "phosphor-svelte/lib/Diamond";
	import Gear from "phosphor-svelte/lib/Gear";
	import ImageSquare from "phosphor-svelte/lib/ImageSquare";
	import List from "phosphor-svelte/lib/List";
	import Plug from "phosphor-svelte/lib/Plug";
	import SquaresFour from "phosphor-svelte/lib/SquaresFour";
	import UserCircle from "phosphor-svelte/lib/UserCircle";

	import ActionList from "../components/ActionList.svelte";
	import DeviceSelector from "../components/DeviceSelector.svelte";
	import DeviceView from "../components/DeviceView.svelte";
	import InstanceEditor from "../components/InstanceEditor.svelte";
	import NoDevicesDetected from "../components/NoDevicesDetected.svelte";
	import ParentActionView from "../components/ParentActionView.svelte";
	import PhotoWall from "../components/PhotoWall.svelte";
	import PluginManager from "../components/PluginManager.svelte";
	import ProfilePage from "../components/ProfilePage.svelte";
	import PropertyInspectorView from "../components/PropertyInspectorView.svelte";
	import SettingsView from "../components/SettingsView.svelte";

	let devices: { [id: string]: DeviceInfo } = {};
	let selectedDevice: string;
	let selectedProfiles: { [id: string]: Profile } = {};
	type Page = "devices" | "plugins" | "photo-wall" | "settings" | "profile";
	let currentPage: Page = "devices";
	let inspectorTab: "appearance" | "action" = "appearance";
	let actionsCollapsed = typeof window !== "undefined" && window.innerWidth < 1400;

	applyAppearance(loadAppearance());

	$: pageKey = currentPage === "photo-wall" ? "photoWall" : currentPage;
	$: selectedInstance = ((): ActionInstance | null => {
		const context = typeof $inspectedInstance === "string" ? $inspectedInstance : null;
		const profile = selectedProfiles[selectedDevice];
		if (!context || !profile) return null;
		for (const list of [profile.keys, profile.sliders, profile.infobars]) {
			for (const instance of list) {
				if (instance?.context === context) return instance;
				for (const child of instance?.children ?? []) {
					if (child.context === context) return child;
				}
			}
		}
		return null;
	})();

	initPortBase();
</script>

<svelte:window on:dragover={(event) => event.preventDefault()} on:drop={(event) => event.preventDefault()} />

<a
	href="#main-content"
	class="sr-only focus:not-sr-only focus:absolute focus:left-2 focus:top-2 focus:z-50 focus:rounded-xl focus:bg-white focus:px-3 focus:py-2 focus:text-[#0B0B0E]"
>
	{$t("v2.skip")}
</a>

<div class="flex h-screen flex-row bg-app-bg text-app-text">
	<aside
		class="app-sidebar glass flex shrink-0 flex-col border-r border-app-border bg-app-panel"
		class:compact={currentPage === "devices" && !actionsCollapsed}
		class:hidden={$inspectedParentAction}
	>
		<div class="flex h-14 items-center gap-2 border-b border-app-border px-4">
			<Diamond size={18} weight="fill" class="shrink-0 text-app-accent" />
			<span class="brand-label text-sm font-semibold tracking-wide">OpenDeck</span>
		</div>

		<nav class="flex flex-col gap-1 p-2" aria-label={$t("nav.devices")}>
			<button
				class="side-row"
				class:side-row-active={currentPage === "devices"}
				aria-current={currentPage === "devices" ? "page" : undefined}
				on:click={() => (currentPage = "devices")}
			>
				<SquaresFour size={20} />
				<span>{$t("nav.devices")}</span>
			</button>
			<button
				class="side-row"
				class:side-row-active={currentPage === "plugins"}
				aria-current={currentPage === "plugins" ? "page" : undefined}
				on:click={() => (currentPage = "plugins")}
			>
				<Plug size={20} />
				<span>{$t("plugin_manager.button")}</span>
			</button>
			<button
				class="side-row"
				class:side-row-active={currentPage === "photo-wall"}
				aria-current={currentPage === "photo-wall" ? "page" : undefined}
				on:click={() => (currentPage = "photo-wall")}
			>
				<ImageSquare size={20} />
				<span>{$t("photo_wall.button")}</span>
			</button>
			<button
				class="side-row"
				class:side-row-active={currentPage === "settings"}
				aria-current={currentPage === "settings" ? "page" : undefined}
				on:click={() => (currentPage = "settings")}
			>
				<Gear size={20} />
				<span>{$t("settings.button")}</span>
			</button>
			<button
				class="side-row"
				class:side-row-active={currentPage === "profile"}
				aria-current={currentPage === "profile" ? "page" : undefined}
				on:click={() => (currentPage = "profile")}
			>
				<UserCircle size={20} />
				<span>{$t("profile_manager.label")}</span>
			</button>
		</nav>
	</aside>

	<div class="flex min-w-0 grow flex-col">
		<header class="flex h-16 shrink-0 items-center justify-between gap-4 border-b border-app-border px-6" class:hidden={$inspectedParentAction}>
			<div class="min-w-0">
				<h1 class="font-display truncate text-2xl font-semibold">{$t(`v2.${pageKey}.title`)}</h1>
				<p class="truncate text-xs text-app-muted">{$t(`v2.${pageKey}.subtitle`)}</p>
			</div>

			<div class="flex shrink-0 items-center gap-3">
				<div class="device-chip flex min-w-[120px] max-w-[220px] items-center justify-center gap-2">
					<span
						class="size-2 shrink-0 rounded-full"
						class:bg-app-success={Boolean(selectedDevice && devices[selectedDevice])}
						class:bg-app-danger={!selectedDevice || !devices[selectedDevice]}
						aria-hidden="true"
					/>
					<DeviceSelector bind:devices bind:value={selectedDevice} bind:selectedProfiles bind:this={$deviceSelector} />
				</div>
				{#if currentPage === "devices"}
					<button class="primary-button" on:click={() => (actionsCollapsed = !actionsCollapsed)}>
						+ {$t("action_list.button")}
					</button>
				{/if}
			</div>
		</header>

		<main id="main-content" class="page-area relative min-h-0 grow overflow-auto p-5">
			{#if currentPage === "devices"}
				<div class="flex h-full min-h-0 gap-4">
					<aside
						class="action-rail flex shrink-0 flex-col overflow-hidden rounded-[20px] border border-app-border bg-app-card"
						class:collapsed={actionsCollapsed}
					>
						<button
							class="flex h-10 shrink-0 items-center justify-center border-b border-app-border text-app-muted hover:text-app-accent"
							on:click={() => (actionsCollapsed = !actionsCollapsed)}
							aria-label={$t("action_list.toggle")}
							title={$t("action_list.toggle")}
						>
							<List size={18} />
						</button>
						<div class="min-w-0 flex-1" class:hidden={actionsCollapsed}>
							<ActionList bind:this={$actionList} />
						</div>
					</aside>

					<div class="flex min-w-0 grow flex-col">
						{#if Object.keys(devices).length > 0 && selectedProfiles}
							{#if $inspectedParentAction}
								<ParentActionView bind:profile={selectedProfiles[selectedDevice]} />
							{/if}

							{#each Object.entries(devices) as [id, device]}
								{#if device && selectedProfiles[id]}
									<DeviceView bind:device bind:profile={selectedProfiles[id]} bind:selectedDevice />
								{/if}
							{/each}
						{:else}
							<NoDevicesDetected />
						{/if}
					</div>

					<aside class="inspector-panel flex shrink-0 flex-col rounded-[20px] border border-app-border bg-app-card">
						<div class="flex h-10 shrink-0 items-center gap-1 border-b border-app-border px-3">
							<button class="tab-button" class:tab-active={inspectorTab === "appearance"} on:click={() => (inspectorTab = "appearance")}>
								{$t("inspector.key")}
							</button>
							<button class="tab-button" class:tab-active={inspectorTab === "action"} on:click={() => (inspectorTab = "action")}>
								{$t("inspector.action")}
							</button>
						</div>

						{#if selectedProfiles[selectedDevice]}
							<div class="min-h-0 grow overflow-auto p-3" class:hidden={!selectedInstance || inspectorTab !== "action"}>
								<PropertyInspectorView bind:device={devices[selectedDevice]} bind:profile={selectedProfiles[selectedDevice]} />
							</div>
						{/if}

						{#if selectedInstance && inspectorTab === "appearance"}
							<div class="min-h-0 grow overflow-auto p-3">
								<InstanceEditor instance={selectedInstance} showEditor={true} inline />
							</div>
						{:else if !selectedInstance}
							<div class="empty-state">{$t("inspector.empty")}</div>
						{/if}
					</aside>
				</div>
			{:else if currentPage === "plugins"}
				<div class="page-panel h-full">
					<PluginManager inline />
				</div>
			{:else if currentPage === "photo-wall"}
				{#if selectedDevice && devices[selectedDevice] && selectedProfiles[selectedDevice]}
					<div class="page-panel h-full">
						<PhotoWall inline show={true} profile={selectedProfiles[selectedDevice]} device={devices[selectedDevice]} />
					</div>
				{:else}
					<div class="empty-state">{$t("v2.photoWall.noDevice")}</div>
				{/if}
			{:else if currentPage === "settings"}
				<div class="page-panel h-full">
					<SettingsView inline />
				</div>
			{:else if selectedDevice && devices[selectedDevice] && selectedProfiles[selectedDevice]}
				<ProfilePage bind:device={devices[selectedDevice]} bind:profile={selectedProfiles[selectedDevice]} />
			{:else}
				<div class="empty-state">{$t("v2.profile.placeholder")}</div>
			{/if}
		</main>
	</div>
</div>
