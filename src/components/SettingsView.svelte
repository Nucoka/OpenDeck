<script lang="ts">
	import ClockClockwise from "phosphor-svelte/lib/ClockClockwise";
	import ClockCounterClockwise from "phosphor-svelte/lib/ClockCounterClockwise";
	import Gear from "phosphor-svelte/lib/Gear";
	import Scroll from "phosphor-svelte/lib/Scroll";
	import Popup from "./Popup.svelte";
	import Tooltip from "./Tooltip.svelte";

	import { ACCENTS, loadAppearance, saveAppearance, type Accent } from "$lib/appearance";
	import { t } from "$lib/i18n";
	import { settings } from "$lib/settings";
	import { PRODUCT_NAME } from "$lib/singletons";

	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { message } from "@tauri-apps/plugin-dialog";

	let showPopup: boolean;
	export let inline: boolean = false;
	let buildInfo: string;
	let appearance = loadAppearance();
	(async () => (buildInfo = await invoke("get_build_info")))();

	function setAccent(accent: Accent) {
		appearance = { ...appearance, accent };
		saveAppearance(appearance);
	}

	function setBlur(value: number) {
		appearance = { ...appearance, blur: value };
		saveAppearance(appearance);
	}

	function setMotion(value: boolean) {
		appearance = { ...appearance, motion: value };
		saveAppearance(appearance);
	}

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

{#if !inline}
	<button class="side-control" on:click={() => (showPopup = true)}>
		{$t("settings.button")}
	</button>
{/if}

<svelte:window
	on:keydown={(event) => {
		if (event.key == "Escape") showPopup = false;
	}}
/>

<Popup show={inline || showPopup} {inline} label={$t("settings.button")}>
	<svelte:fragment slot="header">
		{#if !inline}
			<button class="mr-2 my-1 float-right text-xl text-app-text" on:click={() => (showPopup = false)} aria-label={$t("settings.close")}>✕</button>
		{/if}
	</svelte:fragment>

	{#if $settings}
		<div class="m-3 grid grid-cols-1 gap-4 lg:grid-cols-2">
			<div class="flex flex-col gap-4">
				<section class="settings-card">
					<h3 class="v2-section-title mt-0">{$t("v2.settings.general")}</h3>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.language")}</div>
							<div class="settings-desc">{$t("v2.settings.language.desc")}</div>
						</div>
						<div class="flex shrink-0 items-center gap-2">
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
							<Tooltip>{$t("settings.language.tooltip", { PRODUCT_NAME })}</Tooltip>
						</div>
					</div>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.sleep_timeout_minutes")}</div>
							<div class="settings-desc">{$t("v2.settings.sleep.desc")}</div>
						</div>
						<div class="flex shrink-0 items-center gap-2">
							<input
								type="number"
								min="0"
								bind:value={$settings.sleep_timeout_minutes}
								class="w-16 rounded-xl border border-app-border bg-app-elevated px-2 py-1 text-xs font-mono text-app-text"
								id="settings-sleep_timeout_minutes"
							/>
							<span class="text-xs text-app-muted">{$t("settings.sleep_timeout_minutes.minutes")}</span>
							<Tooltip>{$t("settings.sleep_timeout_minutes.tooltip")}</Tooltip>
						</div>
					</div>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.brightness")}</div>
							<div class="settings-desc">{$t("v2.settings.brightness.desc")}</div>
						</div>
						<input type="range" min="0" max="100" bind:value={$settings.brightness} id="settings-brightness" class="w-40" />
					</div>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.autolaunch")}</div>
							<div class="settings-desc">{$t("v2.settings.autolaunch.desc")}</div>
						</div>
						<div class="flex shrink-0 items-center gap-2">
							<input type="checkbox" bind:checked={$settings.autolaunch} id="settings-autolaunch" />
							<Tooltip>
								{$t("settings.autolaunch.tooltip.1", { PRODUCT_NAME })}
								{#if buildInfo?.split("</summary>")[0]?.includes("linux")}
									<br />
									{$t("settings.autolaunch.tooltip.2", { PRODUCT_NAME })}
								{/if}
							</Tooltip>
						</div>
					</div>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.updatecheck")}</div>
							<div class="settings-desc">{$t("v2.settings.updatecheck.desc")}</div>
						</div>
						<input type="checkbox" bind:checked={$settings.updatecheck} id="settings-updatecheck" />
					</div>
				</section>

				<section class="settings-card">
					<h3 class="v2-section-title mt-0">{$t("settings.section.behavior")}</h3>
					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.sleep_when_computer_locked")}</div>
							<div class="settings-desc">{$t("v2.settings.sleepLocked.desc")}</div>
						</div>
						<div class="flex shrink-0 items-center gap-2">
							<input type="checkbox" bind:checked={$settings.sleep_when_computer_locked} id="settings-sleep_when_computer_locked" />
							<Tooltip>{$t("settings.sleep_when_computer_locked.tooltip")}</Tooltip>
						</div>
					</div>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.background")}</div>
							<div class="settings-desc">{$t("v2.settings.background.desc")}</div>
						</div>
						<div class="flex shrink-0 items-center gap-2">
							<input type="checkbox" bind:checked={$settings.background} id="settings-background" />
							<Tooltip>{$t("settings.background.tooltip", { PRODUCT_NAME })}</Tooltip>
						</div>
					</div>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.rotation")}</div>
							<div class="settings-desc">{$t("v2.settings.rotation.desc")}</div>
						</div>
						<input type="range" min="0" max="270" step="90" bind:value={$settings.rotation} id="settings-rotation" class="w-40" />
					</div>
				</section>
			</div>

			<div class="flex flex-col gap-4">
				<section class="settings-card">
					<h3 class="v2-section-title mt-0">{$t("v2.settings.appearance")}</h3>
					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("v2.settings.accent.desc")}</div>
						</div>
						<div class="flex shrink-0 flex-row gap-2">
							{#each ACCENTS as accent}
								<button
									class="accent-swatch"
									class:accent-swatch-active={appearance.accent === accent}
									style:background={accent}
									on:click={() => setAccent(accent)}
									aria-label={accent}
									title={accent}
								></button>
							{/each}
						</div>
					</div>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("v2.settings.general")}</div>
							<div class="settings-desc">{$t("v2.settings.blur.desc")}</div>
						</div>
						<div class="flex shrink-0 items-center gap-2">
							<input
								type="range"
								min="0"
								max="24"
								value={appearance.blur}
								on:change={(event) => setBlur(Number(event.currentTarget.value))}
								class="w-40"
								aria-label={$t("v2.settings.blur.desc")}
							/>
							<span class="w-8 text-xs font-mono text-app-text">{appearance.blur}</span>
						</div>
					</div>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("v2.settings.motion.desc")}</div>
						</div>
						<input
							type="checkbox"
							checked={appearance.motion}
							on:change={(event) => setMotion(event.currentTarget.checked)}
							aria-label={$t("v2.settings.motion.desc")}
						/>
					</div>
				</section>

				<section class="settings-card">
					<h3 class="v2-section-title mt-0">{$t("settings.section.config")}</h3>
					<div class="settings-desc mb-2">{$t("v2.settings.data.desc")}</div>
					<div class="flex flex-row flex-wrap gap-2">
						<button class="secondary-button" on:click={() => backupConfig()}><ClockCounterClockwise size={16} />{$t("settings.backup_config.button")}</button>
						<button class="secondary-button" on:click={() => restoreConfig()}><ClockClockwise size={16} />{$t("settings.restore_config.button")}</button>
						<button class="secondary-button" on:click={() => invoke("open_config_directory")}><Gear size={16} />{$t("settings.open_config")}</button>
						<button class="secondary-button" on:click={() => invoke("open_log_directory")}><Scroll size={16} />{$t("settings.open_logs")}</button>
					</div>
					<span class="mt-3 block text-xs text-app-muted">{@html buildInfo}</span>
				</section>

				<section class="settings-card">
					<h3 class="v2-section-title mt-0">{$t("settings.section.advanced")}</h3>
					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.statistics")}</div>
							<div class="settings-desc">{$t("v2.settings.statistics.desc")}</div>
						</div>
						<input type="checkbox" bind:checked={$settings.statistics} id="settings-statistics" />
					</div>

					{#if !buildInfo?.split("</summary>")[0]?.includes("windows")}
						<div class="settings-row">
							<div class="min-w-0">
								<div>{$t("settings.separatewine")}</div>
							</div>
							<div class="flex shrink-0 items-center gap-2">
								<input type="checkbox" bind:checked={$settings.separatewine} id="settings-separatewine" />
								<Tooltip>{$t("settings.separatewine.tooltip", { PRODUCT_NAME })}</Tooltip>
							</div>
						</div>
					{/if}

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.developer")}</div>
							<div class="settings-desc">{$t("v2.settings.developer.desc")}</div>
						</div>
						<div class="flex shrink-0 items-center gap-2">
							<input type="checkbox" bind:checked={$settings.developer} id="settings-developer" />
							<Tooltip>{$t("settings.developer.tooltip")}</Tooltip>
						</div>
					</div>

					<div class="settings-row">
						<div class="min-w-0">
							<div>{$t("settings.disableelgato")}</div>
							<div class="settings-desc">{$t("v2.settings.disableElgato.desc")}</div>
						</div>
						<div class="flex shrink-0 items-center gap-2">
							<input type="checkbox" bind:checked={$settings.disableelgato} id="settings-disableelgato" />
							<Tooltip>{$t("settings.disableelgato.tooltip")}</Tooltip>
						</div>
					</div>
				</section>
			</div>
		</div>
	{/if}
</Popup>
