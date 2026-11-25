<script lang="ts">
	import type { DeviceInfo } from "$lib/DeviceInfo";
	import type { Profile } from "$lib/Profile";

	import Cube from "phosphor-svelte/lib/Cube";
	import Plus from "phosphor-svelte/lib/Plus";
	import ProfileManager from "./ProfileManager.svelte";

	import { t } from "$lib/i18n";
	import { profileManager } from "$lib/singletons";

	import { invoke } from "@tauri-apps/api/core";

	export let device: DeviceInfo;
	export let profile: Profile;

	let profiles: string[] = [];

	$: if (device?.id) void loadProfiles(device.id);
	$: keyCount = profile ? [...profile.keys, ...profile.sliders, ...profile.infobars].filter((instance) => instance != null).length : 0;
	$: pluginCount = profile
		? new Set([...profile.keys, ...profile.sliders, ...profile.infobars].filter((instance) => instance != null).map((instance) => instance!.action.plugin)).size
		: 0;

	async function loadProfiles(deviceId: string) {
		profiles = await invoke("get_profiles", { device: deviceId });
	}

	async function switchProfile(id: string) {
		if (!$profileManager) return;
		await $profileManager.setProfile(id);
		await $profileManager.setProfile(id);
		await loadProfiles(device.id);
	}

	async function createProfile() {
		let name = `New profile ${profiles.length + 1}`;
		let suffix = 2;
		while (profiles.includes(name)) {
			name = `New profile ${profiles.length + suffix}`;
			suffix += 1;
		}
		await switchProfile(name);
	}
</script>

<div class="flex flex-col gap-4">
	<section class="settings-card flex flex-row items-center gap-4">
		<div class="flex size-14 shrink-0 items-center justify-center rounded-[18px] border border-app-border bg-app-elevated text-app-accent">
			<Cube size={26} />
		</div>
		<div class="min-w-0 flex-1">
			<div class="font-display truncate text-lg font-semibold">{device.name}</div>
			<div class="truncate text-xs text-app-muted">{$t("v2.profile.connected")} · {device.id}</div>
		</div>
		<div class="flex shrink-0 flex-row gap-4">
			<div class="text-center">
				<div class="font-mono text-lg text-app-text">{keyCount}</div>
				<div class="text-xs text-app-muted">{$t("v2.profile.keysSet")}</div>
			</div>
			<div class="text-center">
				<div class="font-mono text-lg text-app-text">{pluginCount}</div>
				<div class="text-xs text-app-muted">{$t("v2.profile.pluginsInstalled")}</div>
			</div>
			<div class="text-center">
				<div class="font-mono text-lg text-app-text">{profiles.length}</div>
				<div class="text-xs text-app-muted">{$t("v2.profile.profiles")}</div>
			</div>
		</div>
	</section>

	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
		{#if profiles.length === 0}
			<div class="empty-state">{$t("v2.profile.empty")}</div>
		{/if}
		{#each profiles as id}
			<button class="profile-card" class:profile-card-active={profile?.id === id} on:click={() => void switchProfile(id)}>
				<div class="profile-preview" style={`--cols: ${device.columns}; --rows: ${device.rows}`}>
					{#each { length: device.rows * device.columns } as _, i}
						<span class:profile-key-set={Boolean(profile?.keys[i])}></span>
					{/each}
				</div>
				<div class="truncate text-sm font-semibold text-app-text">{id}</div>
				<div class="text-xs text-app-muted">{profile?.id === id ? $t("v2.profile.active") : $t("v2.profile.clickToSwitch")}</div>
			</button>
		{/each}

		<button class="profile-card profile-card-new" on:click={() => void createProfile()}>
			<Plus size={28} class="text-app-accent" />
			<div class="text-sm font-semibold text-app-text">{$t("v2.profile.newProfile")}</div>
		</button>
	</div>

	<section class="settings-card">
		<h3 class="v2-section-title mt-0">{$t("v2.profile.manage")}</h3>
		<ProfileManager bind:device bind:profile bind:this={$profileManager} />
	</section>
</div>
