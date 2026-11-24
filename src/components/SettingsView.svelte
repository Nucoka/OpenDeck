<script lang="ts">
	import ClockClockwise from "phosphor-svelte/lib/ClockClockwise";
	import ClockCounterClockwise from "phosphor-svelte/lib/ClockCounterClockwise";
	import Gear from "phosphor-svelte/lib/Gear";
	import Heart from "phosphor-svelte/lib/Heart";
	import Scroll from "phosphor-svelte/lib/Scroll";
	import Star from "phosphor-svelte/lib/Star";
	import Popup from "./Popup.svelte";
	import Tooltip from "./Tooltip.svelte";

	import { t } from "$lib/i18n";
	import { settings } from "$lib/settings";
	import { PRODUCT_NAME } from "$lib/singletons";

	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { message } from "@tauri-apps/plugin-dialog";

	let showPopup: boolean;
	let buildInfo: string;
	(async () => (buildInfo = await invoke("get_build_info")))();

	listen("device_brightness", ({ payload }: { payload: { action: string; value: number } }) => {
		if (!$settings) return;
		let value = $settings.brightness;
		switch (payload.action) {
			case "increase":
				value += payload.value;
				break;
			case "decrease":
				value -= payload.value;
				break;
			default:
				value = payload.value;
				break;
		}
		$settings.brightness = Math.max(0, Math.min(100, value));
	});

	async function backupConfig() {
		await message($t("settings.backup_config.prompt"), { title: $t("settings.backup_config.title"), buttons: { ok: $t("dialog.ok") } });
		if (await invoke("backup_config_directory")) {
			await message($t("settings.backup_config.success.prompt"), { title: $t("settings.backup_config.success.title"), buttons: { ok: $t("dialog.ok") } });
		}
	}

	async function restoreConfig() {
		await message($t("settings.restore_config.prompt"), { title: $t("settings.restore_config.title"), buttons: { ok: $t("dialog.ok") } });
		await invoke("restore_config_directory");
	}
</script>

<button class="side-control" on:click={() => (showPopup = true)}>
	{$t("settings.button")}
</button>

<svelte:window
	on:keydown={(event) => {
		if (event.key == "Escape") showPopup = false;
	}}
/>

<Popup show={showPopup} label={$t("settings.button")}>
	<svelte:fragment slot="header">
		<button class="mr-2 my-1 float-right text-xl text-app-text" on:click={() => (showPopup = false)} aria-label={$t("settings.close")}>✕</button>
		<h2 class="m-2 font-semibold text-xl text-app-text">{$t("settings.button")}</h2>
	</svelte:fragment>

	{#if $settings}
		<div class="flex flex-row items-center mx-2 mb-2 space-x-2">
			<label for="settings-language" class="text-app-muted">{$t("settings.language")}</label>
			<div class="select-wrapper">
				<select bind:value={$settings.language} class="w-auto pr-10!" id="settings-language">
					<option value="de">Deutsch</option>
					<option value="en">English</option>
					<option value="es">Español</option>
					<option value="fr">Français</option>
					<option value="pt_BR">Português (Brasil)</option>
					<option value="sv">Svenska</option>
					<option value="uk">Українська</option>
					<option value="zh_CN">中文</option>
					<option value="ja">日本語</option>
					<option value="ko">한국어</option>
				</select>
			</div>
			<Tooltip>
				{$t("settings.language.tooltip", { PRODUCT_NAME })}
			</Tooltip>
		</div>

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-brightness" class="text-app-muted">{$t("settings.brightness")}</label>
			<input type="range" min="0" max="100" bind:value={$settings.brightness} id="settings-brightness" />
		</div>

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-sleep_timeout_minutes" class="text-app-muted">{$t("settings.sleep_timeout_minutes")}</label>
			<input
				type="number"
				min="0"
				bind:value={$settings.sleep_timeout_minutes}
				class="w-12 px-1 text-app-text border border-app-border rounded-lg"
				id="settings-sleep_timeout_minutes"
			/>
			<span class="text-app-muted">{$t("settings.sleep_timeout_minutes.minutes")}</span>
			<Tooltip>{$t("settings.sleep_timeout_minutes.tooltip")}</Tooltip>
		</div>

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-sleep_when_computer_locked" class="text-app-muted">{$t("settings.sleep_when_computer_locked")}</label>
			<input type="checkbox" bind:checked={$settings.sleep_when_computer_locked} id="settings-sleep_when_computer_locked" />
			<Tooltip>{$t("settings.sleep_when_computer_locked.tooltip")}</Tooltip>
		</div>

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-rotation" class="text-app-muted">{$t("settings.rotation")}</label>
			<input type="range" min="0" max="270" step="90" bind:value={$settings.rotation} id="settings-rotation" />
		</div>

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-background" class="text-app-muted">{$t("settings.background")}</label>
			<input type="checkbox" bind:checked={$settings.background} id="settings-background" />
			<Tooltip>{$t("settings.background.tooltip", { PRODUCT_NAME })}</Tooltip>
		</div>

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-autolaunch" class="text-app-muted">{$t("settings.autolaunch")}</label>
			<input type="checkbox" bind:checked={$settings.autolaunch} id="settings-autolaunch" />
			<Tooltip>
				{$t("settings.autolaunch.tooltip.1", { PRODUCT_NAME })}
				{#if buildInfo?.split("</summary>")[0]?.includes("linux")}
					<br />
					{$t("settings.autolaunch.tooltip.2", { PRODUCT_NAME })}
				{/if}
			</Tooltip>
		</div>

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-updatecheck" class="text-app-muted">{$t("settings.updatecheck")}</label>
			<input type="checkbox" bind:checked={$settings.updatecheck} id="settings-updatecheck" />
		</div>

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-statistics" class="text-app-muted">{$t("settings.statistics")}</label>
			<input type="checkbox" bind:checked={$settings.statistics} id="settings-statistics" />
		</div>

		{#if !buildInfo?.split("</summary>")[0]?.includes("windows")}
			<div class="flex flex-row items-center m-2 space-x-2">
				<label for="settings-separatewine" class="text-app-muted">{$t("settings.separatewine")}</label>
				<input type="checkbox" bind:checked={$settings.separatewine} id="settings-separatewine" />
				<Tooltip>
					{$t("settings.separatewine.tooltip", { PRODUCT_NAME })}
				</Tooltip>
			</div>
		{/if}

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-developer" class="text-app-muted">{$t("settings.developer")}</label>
			<input type="checkbox" bind:checked={$settings.developer} id="settings-developer" />
			<Tooltip>
				{$t("settings.developer.tooltip")}
			</Tooltip>
		</div>

		<div class="flex flex-row items-center m-2 space-x-2">
			<label for="settings-disableelgato" class="text-app-muted">{$t("settings.disableelgato")}</label>
			<input type="checkbox" bind:checked={$settings.disableelgato} id="settings-disableelgato" />
			<Tooltip>{$t("settings.disableelgato.tooltip")}</Tooltip>
		</div>
	{/if}

	<div class="ml-2">
		<div class="flex flex-row my-3 space-x-2">
			<button
				class="flex flex-row items-center px-2 py-1 text-sm text-app-text bg-app-elevated hover:bg-app-border transition-colors border border-app-border rounded-lg"
				on:click={() => backupConfig()}
			>
				<ClockCounterClockwise class="mr-1" />
				{$t("settings.backup_config.button")}
			</button>
			<button
				class="flex flex-row items-center px-2 py-1 text-sm text-app-text bg-app-elevated hover:bg-app-border transition-colors border border-app-border rounded-lg"
				on:click={() => restoreConfig()}
			>
				<ClockClockwise class="mr-1" />
				{$t("settings.restore_config.button")}
			</button>
			<button
				class="flex flex-row items-center px-2 py-1 text-sm text-app-text bg-app-elevated hover:bg-app-border transition-colors border border-app-border rounded-lg"
				on:click={() => invoke("open_config_directory")}
			>
				<Gear class="mr-1" />
				{$t("settings.open_config")}
			</button>
			<button
				class="flex flex-row items-center px-2 py-1 text-sm text-app-text bg-app-elevated hover:bg-app-border transition-colors border border-app-border rounded-lg"
				on:click={() => invoke("open_log_directory")}
			>
				<Scroll class="mr-1" />
				{$t("settings.open_logs")}
			</button>
		</div>

		<span class="text-xs text-app-muted">
			{@html buildInfo}
		</span>
	</div>

	<svelte:fragment slot="footer">
		<div class="flex flex-row items-center mt-4 text-sm text-app-muted">
			<span class="mr-1">
				{$t("settings.footer.1")}
				<button on:click={() => invoke("open_url", { url: "https://github.com/nekename/OpenDeck" })} class="underline">{$t("settings.footer.2")}</button>
			</span>
			<Star weight="fill" fill="yellow" />
			<span class="mx-1">
				{$t("settings.footer.3")}
				<button on:click={() => invoke("open_url", { url: "https://github.com/sponsors/nekename" })} class="underline">{$t("settings.footer.4")}</button>
			</span>
			<Heart weight="fill" fill="fuchsia" />
			<span class="ml-1">{$t("settings.footer.5")}</span>
		</div>
	</svelte:fragment>
</Popup>
