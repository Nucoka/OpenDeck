<script lang="ts">
	import type { ActionInstance } from "$lib/ActionInstance";
	import type { DeviceInfo } from "$lib/DeviceInfo";
	import type { Profile } from "$lib/Profile";

	import { t } from "$lib/i18n";
	import { initPortBase } from "$lib/ports";
	import { inspectedInstance, inspectedParentAction } from "$lib/propertyInspector";
	import { settings } from "$lib/settings";
	import { actionList, deviceSelector, profileManager } from "$lib/singletons";

	import Diamond from "phosphor-svelte/lib/Diamond";
	import Gear from "phosphor-svelte/lib/Gear";
	import ImageSquare from "phosphor-svelte/lib/ImageSquare";
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
	import ProfileManager from "../components/ProfileManager.svelte";
	import PropertyInspectorView from "../components/PropertyInspectorView.svelte";
	import SettingsView from "../components/SettingsView.svelte";

	let devices: { [id: string]: DeviceInfo } = {};
	let selectedDevice: string;
	let selectedProfiles: { [id: string]: Profile } = {};
	let showPhotoWall = false;
	let inspectorTab: "appearance" | "action" = "appearance";

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

<div class="flex h-screen flex-row bg-app-bg text-app-text">
	<aside class="flex w-44 shrink-0 flex-col border-r border-app-border bg-app-panel" class:hidden={$inspectedParentAction}>
		<div class="flex h-14 items-center gap-2 border-b border-app-border px-4">
			<Diamond size={18} weight="fill" class="text-app-accent" />
			<span class="text-sm font-semibold tracking-wide">OpenDeck</span>
		</div>

		<nav class="flex flex-col gap-1 p-2" aria-label={$t("settings.button")}>
			<div class="side-row side-row-active" aria-current="page">
				<SquaresFour size={20} />
				<span>{$t("device_selector.device")}</span>
			</div>
			<div class="side-row">
				<Plug size={20} />
				<PluginManager />
			</div>
			<div class="side-row">
				<UserCircle size={20} />
				<span>{$t("profile_manager.label")}</span>
			</div>
			<div class="side-row">
				<Gear size={20} />
				<SettingsView />
			</div>
		</nav>
	</aside>

	<div class="flex min-w-0 grow flex-col">
		<header class="grid h-14 shrink-0 grid-cols-3 items-center border-b border-app-border bg-app-bg px-6" class:hidden={$inspectedParentAction}>
			<div class="flex items-center gap-2">
				{#key selectedDevice}
					{#if selectedDevice && devices[selectedDevice]}
						<ProfileManager bind:device={devices[selectedDevice]} bind:profile={selectedProfiles[selectedDevice]} bind:this={$profileManager} />
					{/if}
				{/key}
			</div>

			<div class="flex items-center justify-center gap-2">
				<span
					class="size-2 rounded-full"
					class:bg-app-success={Boolean(selectedDevice && devices[selectedDevice])}
					class:bg-app-danger={!selectedDevice || !devices[selectedDevice]}
					aria-hidden="true"
				/>
				<DeviceSelector bind:devices bind:value={selectedDevice} bind:selectedProfiles bind:this={$deviceSelector} />
			</div>

			<div class="flex items-center justify-end gap-4">
				{#if $settings}
					<label class="flex items-center gap-2 text-xs text-app-muted" for="header-brightness">
						{$t("settings.brightness")}
						<input id="header-brightness" type="range" min="0" max="100" bind:value={$settings.brightness} class="w-28 accent-app-accent" />
					</label>
				{/if}
				{#if selectedDevice && devices[selectedDevice] && selectedProfiles[selectedDevice]}
					<button class="header-button" on:click={() => (showPhotoWall = true)}>
						<ImageSquare size={16} />
						{$t("photo_wall.button")}
					</button>
					<PhotoWall bind:show={showPhotoWall} bind:profile={selectedProfiles[selectedDevice]} device={devices[selectedDevice]} />
				{/if}
			</div>
		</header>

		<main class="flex min-h-0 grow">
			<aside class="w-56 shrink-0 overflow-hidden border-r border-app-border bg-app-panel">
				<ActionList bind:this={$actionList} />
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

			<aside class="flex w-[360px] shrink-0 flex-col border-l border-app-border bg-app-panel">
				<div class="flex h-10 shrink-0 items-center gap-1 border-b border-app-border px-3">
					<button
						class="rounded-lg px-3 py-1 text-xs text-app-muted hover:text-app-text"
						class:bg-app-accent-soft={inspectorTab === "appearance"}
						class:text-app-accent={inspectorTab === "appearance"}
						on:click={() => (inspectorTab = "appearance")}
					>
						{$t("inspector.appearance")}
					</button>
					<button
						class="rounded-lg px-3 py-1 text-xs text-app-muted hover:text-app-text"
						class:bg-app-accent-soft={inspectorTab === "action"}
						class:text-app-accent={inspectorTab === "action"}
						on:click={() => (inspectorTab = "action")}
					>
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
		</main>
	</div>
</div>
