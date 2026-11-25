<script lang="ts">
	import type { ActionInstance } from "$lib/ActionInstance";
	import type { ActionState } from "$lib/ActionState";
	import type { Context } from "$lib/Context";
	import type { DeviceInfo } from "$lib/DeviceInfo";
	import type { Profile } from "$lib/Profile";

	import { t } from "$lib/i18n";
	import { loadImage, slicePhoto, type PhotoWallMode } from "$lib/photoWall";
	import { renderImage } from "$lib/rendererHelper";

	import Popup from "./Popup.svelte";

	import ImageSquare from "phosphor-svelte/lib/ImageSquare";

	import { invoke } from "@tauri-apps/api/core";
	import { onDestroy, onMount } from "svelte";

	const TILE_DELAY_MS = 120;
	const SCREEN_COLUMNS_KEY = "opendeck.photoWall.screenColumns";
	const GRADIENTS: [string, string][] = [
		["#4F6BFF", "#1FB58A"],
		["#FF5C7A", "#A66BFF"],
		["#1FB58A", "#0B0B0E"],
		["#4F6BFF", "#F5F5F7"],
		["#A66BFF", "#FF5C7A"],
		["#F5C451", "#FF5C7A"],
		["#0B0B0E", "#4F6BFF"],
		["#1FB58A", "#A66BFF"],
	];

	type TileSnapshot = {
		position: number;
		hadInstance: boolean;
		currentState: number;
		states: ActionState[];
	};

	export let device: DeviceInfo;
	export let profile: Profile;
	export let show = false;
	export let inline: boolean = false;

	type RenderParams = {
		image: HTMLImageElement | undefined;
		mode: PhotoWallMode;
		gap: number;
		background: string;
		columns: number;
		rows: number;
		tileWidth: number;
		tileHeight: number;
	};

	let fileInput: HTMLInputElement;
	let previewCanvas: HTMLCanvasElement;
	let image: HTMLImageElement | undefined;
	let mode: PhotoWallMode = "cover";
	let gap = 0;
	let background = "#000000";
	let hideText = true;
	let screenColumns = device.columns;
	let settingsLoaded = false;
	let error = "";
	let limit: number | undefined;
	let applying = false;
	let resetting = false;
	let cancelled = false;
	let progress = 0;
	let total = 0;
	let snapshot: TileSnapshot[] = [];
	let snapshotDevice = "";
	let snapshotProfile = "";
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;

	$: tileWidth = device.key_width || 144;
	$: tileHeight = device.key_height || 144;
	$: columnsWithScreens = Math.max(1, Math.min(screenColumns, device.columns));
	$: params = { image, mode, gap, background, columns: columnsWithScreens, rows: device.rows, tileWidth, tileHeight };
	$: if (params.image && previewCanvas) scheduleRender(params);

	onMount(() => {
		try {
			const stored = Number(localStorage.getItem(SCREEN_COLUMNS_KEY));
			if (Number.isInteger(stored) && stored >= 1 && stored <= device.columns) screenColumns = stored;
		} catch {
			// Ignore unavailable local storage.
		}
		settingsLoaded = true;
	});

	$: if (settingsLoaded) {
		try {
			localStorage.setItem(SCREEN_COLUMNS_KEY, String(screenColumns));
		} catch {
			// Ignore unavailable local storage.
		}
	}

	onDestroy(() => {
		if (debounceTimer !== undefined) clearTimeout(debounceTimer);
	});

	function scheduleRender(params: RenderParams) {
		if (debounceTimer !== undefined) clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			const { image, mode, gap, background, columns, rows, tileWidth, tileHeight } = params;
			if (!image || !previewCanvas) return;

			const { preview } = slicePhoto(image, { columns, rows, tileWidth, tileHeight, mode, gap, background });
			previewCanvas.width = preview.width;
			previewCanvas.height = preview.height;
			const context = previewCanvas.getContext("2d");
			if (!context) return;

			context.drawImage(preview, 0, 0);
			context.strokeStyle = "rgba(255, 255, 255, 0.7)";
			context.lineWidth = 1;
			for (let r = 0; r < rows; r++) {
				for (let c = 0; c < columns; c++) {
					context.strokeRect(c * (tileWidth + gap) + 0.5, r * (tileHeight + gap) + 0.5, tileWidth - 1, tileHeight - 1);
				}
			}
		}, 120);
	}

	function defaultState(): ActionState {
		return {
			image: "actionDefaultImage",
			image_override: null,
			image_scale: 100,
			background_colour: "#000000",
			name: "",
			text: "",
			text_override: null,
			show: true,
			colour: "#FFFFFF",
			stroke_colour: "#000000",
			alignment: "middle",
			family: "Liberation Sans",
			style: "Regular",
			size: 16,
			stroke_size: 3,
			underline: false,
		};
	}

	function buildStates(existing: ActionInstance | null, tile: string): ActionState[] {
		const states = existing ? structuredClone(existing.states) : [defaultState()];
		for (const state of states) {
			state.image_override = tile;
			state.image_scale = 100;
			if (hideText) {
				state.show = false;
				state.text_override = "";
			}
		}
		return states;
	}

	async function apply() {
		if (!image || !profile || applying) return;

		const { tiles } = slicePhoto(image, { columns: columnsWithScreens, rows: device.rows, tileWidth, tileHeight, mode, gap, background });
		total = limit && limit > 0 ? Math.min(limit, tiles.length) : tiles.length;
		error = "";
		snapshot = [];
		snapshotDevice = device.id;
		snapshotProfile = profile.id;
		progress = 0;
		applying = true;
		cancelled = false;

		for (let i = 0; i < total; i++) {
			if (cancelled) break;
			const position = i;
			const existing = profile.keys[position] ?? null;
			snapshot.push({
				position,
				hadInstance: existing != null,
				currentState: existing?.current_state ?? 0,
				states: existing ? structuredClone(existing.states) : [],
			});
			const context: Context = { device: device.id, profile: profile.id, controller: "Keypad", position };
			try {
				await invoke("photo_wall_apply_tile", { context, states: buildStates(existing, tiles[i]), createIfEmpty: existing == null });
				await invoke("update_image", { context, image: tiles[i] });
			} catch (caught) {
				error = caught instanceof Error ? caught.message : String(caught);
				break;
			}
			progress = i + 1;
			if (i + 1 < total) await new Promise<void>((resolve) => setTimeout(resolve, TILE_DELAY_MS));
		}

		applying = false;
		try {
			profile = await invoke<Profile>("get_selected_profile", { device: device.id });
		} catch (caught) {
			error = caught instanceof Error ? caught.message : String(caught);
		}
	}

	function cancel() {
		cancelled = true;
	}

	function actionContext(position: number, index: number): string {
		return `${device.id}.${profile.id}.Keypad.${position}.${index}`;
	}

	async function renderState(state: ActionState): Promise<string> {
		const canvas = document.createElement("canvas");
		canvas.width = 144;
		canvas.height = 144;
		await renderImage(canvas, null, state, state.image, false, false, true, false, false, 0);
		return canvas.toDataURL("image/jpeg", 0.9);
	}

	async function reset() {
		if (!profile || applying || resetting || snapshot.length === 0) return;
		if (device.id !== snapshotDevice || profile.id !== snapshotProfile) {
			error = $t("photo_wall.reset.mismatch");
			return;
		}

		let fresh: Profile;
		try {
			fresh = await invoke<Profile>("get_selected_profile", { device: device.id });
		} catch (caught) {
			error = caught instanceof Error ? caught.message : String(caught);
			return;
		}

		error = "";
		total = snapshot.length;
		progress = 0;
		resetting = true;
		cancelled = false;

		for (let i = 0; i < total; i++) {
			if (cancelled) break;
			const entry = snapshot[i];
			const context: Context = { device: device.id, profile: profile.id, controller: "Keypad", position: entry.position };
			const skip = entry.hadInstance && !fresh.keys[entry.position];
			if (!skip) {
				try {
					if (entry.hadInstance) {
						for (const [index, state] of entry.states.entries()) {
							await invoke("set_state", { context: actionContext(entry.position, index), index, state });
						}
						if (entry.states.length > 0) {
							await invoke("set_current_state", { context: actionContext(entry.position, 0), currentState: entry.currentState });
							const previous = await renderState(entry.states[entry.currentState] ?? entry.states[0]);
							await invoke("update_image", { context, image: previous });
						}
					} else {
						await invoke("remove_instance", { context: actionContext(entry.position, 0) });
						await invoke("update_image", { context, image: null });
					}
				} catch (caught) {
					error = caught instanceof Error ? caught.message : String(caught);
					break;
				}
			}
			progress = i + 1;
			if (i + 1 < total) await new Promise<void>((resolve) => setTimeout(resolve, TILE_DELAY_MS));
		}

		resetting = false;
		try {
			profile = await invoke<Profile>("get_selected_profile", { device: device.id });
		} catch (caught) {
			error = caught instanceof Error ? caught.message : String(caught);
		}
		if (!cancelled && !error) snapshot = [];
	}

	async function useFile(file: File | undefined) {
		if (!file) return;
		try {
			image = await loadImage(file);
			error = "";
		} catch (caught) {
			image = undefined;
			error = caught instanceof Error ? caught.message : String(caught);
		}
	}

	function useGradient(index: number) {
		const canvas = document.createElement("canvas");
		canvas.width = 512;
		canvas.height = 512;
		const context = canvas.getContext("2d");
		if (!context) return;
		const [from, to] = GRADIENTS[index];
		const gradient = context.createLinearGradient(0, 0, 512, 512);
		gradient.addColorStop(0, from);
		gradient.addColorStop(1, to);
		context.fillStyle = gradient;
		context.fillRect(0, 0, 512, 512);

		const preset = new Image();
		preset.onload = () => {
			image = preset;
			error = "";
		};
		preset.src = canvas.toDataURL("image/png");
	}
</script>

<svelte:window
	on:keydown={(event) => {
		if (event.key == "Escape") show = false;
	}}
/>

<Popup show={inline || show} {inline} label={$t("photo_wall.title")}>
	<svelte:fragment slot="header">
		<button class="mr-2 my-1 float-right text-xl text-app-text" on:click={() => (show = false)} aria-label={$t("photo_wall.close")}>✕</button>
		<h2 class="m-2 font-semibold text-xl text-app-text">{$t("photo_wall.title")}</h2>
	</svelte:fragment>

	<div class="m-3 grid grid-cols-1 gap-4 lg:grid-cols-[minmax(0,1fr)_320px]">
		<div class="flex min-w-0 flex-col gap-3">
			<button
				class="photo-dropzone flex flex-row items-center justify-center gap-2 rounded-[20px] border border-dashed border-app-border bg-app-card p-6 text-xs text-app-muted hover:border-app-accent"
				on:click={() => fileInput.click()}
				on:dragover|preventDefault
				on:drop|preventDefault={(event) => useFile(event.dataTransfer?.files?.[0])}
			>
				<ImageSquare size={20} />
				{$t("photo_wall.chooseImage")}
			</button>
			<input
				bind:this={fileInput}
				type="file"
				accept="image/*"
				class="hidden"
				on:change={() => {
					void useFile(fileInput.files?.[0]);
					fileInput.value = "";
				}}
			/>
			{#if error}
				<p class="text-sm text-app-danger">{$t("photo_wall.error")}: {error}</p>
			{/if}

			<div class="rounded-[20px] border border-app-border bg-app-card p-4">
				<h3 class="v2-section-title mt-0">{$t("photo_wall.preview")}</h3>
				<canvas bind:this={previewCanvas} class="max-w-full rounded-xl border border-app-border" aria-label={$t("photo_wall.preview")} />
			</div>

			<div class="flex flex-row flex-wrap gap-2">
				{#if image}
					<img src={image.src} alt="" class="size-10 rounded-[10px] border border-app-border object-cover" />
				{/if}
				{#each GRADIENTS as _, i}
					<button
						class="size-10 rounded-[10px] border border-app-border"
						style:background={`linear-gradient(135deg, ${GRADIENTS[i][0]}, ${GRADIENTS[i][1]})`}
						on:click={() => useGradient(i)}
						aria-label={`Preset ${i + 1}`}
						title={`Preset ${i + 1}`}
					></button>
				{/each}
			</div>
		</div>

		<div class="flex flex-col gap-4 rounded-[20px] border border-app-border bg-app-card p-4">
			<div class="flex flex-row items-center gap-2">
				<span class="text-app-muted">{$t("photo_wall.mode")}:</span>
				<button class="tab-button" class:tab-active={mode === "cover"} on:click={() => (mode = "cover")}>{$t("photo_wall.mode.cover")}</button>
				<button class="tab-button" class:tab-active={mode === "fit"} on:click={() => (mode = "fit")}>{$t("photo_wall.mode.fit")}</button>
			</div>

			<div class="flex flex-row items-center gap-2">
				<label for="photo-wall-columns" class="flex-1 text-app-muted">{$t("photo_wall.screen_columns")}</label>
				<input type="range" min="1" max={device.columns} bind:value={screenColumns} id="photo-wall-columns" />
				<span class="w-8 text-xs font-mono text-app-text">{columnsWithScreens}</span>
			</div>

			<div class="flex flex-row items-center gap-2">
				<label for="photo-wall-gap" class="flex-1 text-app-muted">{$t("photo_wall.gap")}</label>
				<input type="range" min="0" max="80" bind:value={gap} id="photo-wall-gap" />
				<span class="w-8 text-xs font-mono text-app-text">{gap}</span>
			</div>

			{#if mode === "fit"}
				<div class="flex flex-row items-center gap-2">
					<label for="photo-wall-background" class="flex-1 text-app-muted">{$t("photo_wall.background")}</label>
					<input type="color" bind:value={background} class="size-8 rounded-lg bg-app-elevated" id="photo-wall-background" />
				</div>
			{/if}

			<label class="flex flex-row items-center gap-2 text-app-muted" for="photo-wall-hide-text">
				<input type="checkbox" bind:checked={hideText} id="photo-wall-hide-text" />
				{$t("photo_wall.hide_text")}
			</label>

			<div class="flex flex-row items-center gap-2">
				<label for="photo-wall-limit" class="flex-1 text-app-muted">{$t("photo_wall.applyLimit")}</label>
				<input
					type="number"
					min="1"
					max={device.rows * columnsWithScreens}
					bind:value={limit}
					placeholder={$t("photo_wall.all")}
					class="w-20 rounded-xl border border-app-border bg-app-elevated px-2 py-1 text-xs font-mono text-app-text"
					id="photo-wall-limit"
				/>
			</div>

			<p class="text-xs text-app-muted">{$t("photo_wall.reset.warning")}</p>
		</div>
	</div>

	<svelte:fragment slot="footer">
		<div class="m-2 flex flex-row items-center justify-end gap-2">
			{#if applying || resetting}
				<span class="text-sm font-mono text-app-text">{$t("photo_wall.progress", { n: progress, total })}</span>
				<button class="secondary-button" on:click={cancel} disabled={cancelled}>{$t("photo_wall.cancel")}</button>
			{/if}
			<button class="secondary-button" on:click={() => void reset()} disabled={snapshot.length === 0 || applying || resetting}>
				{$t("photo_wall.reset")}
			</button>
			<button class="primary-button" on:click={() => void apply()} disabled={!image || !profile || applying || resetting}>
				{$t("photo_wall.applyDeck")}
			</button>
		</div>
	</svelte:fragment>
</Popup>
