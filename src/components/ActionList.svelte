<script lang="ts">
	import type { Action } from "$lib/Action";

	import MagnifyingGlass from "phosphor-svelte/lib/MagnifyingGlass";

	import { t } from "$lib/i18n";
	import { getWebserverUrl } from "$lib/ports";
	import { copiedItem } from "$lib/propertyInspector";
	import { localisations } from "$lib/settings";
	import { PRODUCT_NAME } from "$lib/singletons";

	import { invoke } from "@tauri-apps/api/core";

	let categories: { [name: string]: { icon?: string; actions: Action[] } } = {};
	let plugins: any[] = [];
	export async function reload() {
		categories = await invoke("get_categories");
		plugins = await invoke("list_plugins");
	}
	reload();

	let query: string = "";
	let activeCategory: string | null = null;
	let filteredCategories: [string, { icon?: string; actions: Action[] }][] = [];
	$: categoryChips = Array.from(new Set(Object.keys(categories)));
	$: {
		let lowerCaseQuery = query.toLowerCase().trim();
		filteredCategories = Object.entries(categories)
			.sort((a, b) => (a[0] == PRODUCT_NAME ? -1 : b[0] == PRODUCT_NAME ? 1 : a[0].localeCompare(b[0])))
			.map(([categoryName, { icon, actions }]): [string, { icon?: string; actions: Action[] }] => {
				if (!categoryName.toLowerCase().includes(lowerCaseQuery)) {
					actions = actions.filter((action) => action.name.toLowerCase().includes(lowerCaseQuery));
				}
				return [categoryName, { icon, actions }];
			})
			.filter(([name, { actions }]) => actions.length > 0 && (!activeCategory || name === activeCategory));
	}

	function handleListKeydown(event: KeyboardEvent) {
		if (!["ArrowUp", "ArrowDown", "Home", "End"].includes(event.key)) return;
		const list = event.currentTarget as HTMLElement;
		const items = Array.from(list.querySelectorAll("[role='option']"));
		const currentIndex = items.indexOf(event.target as Element);
		if (currentIndex == -1) return;

		event.preventDefault();

		let newIndex = currentIndex;
		switch (event.key) {
			case "ArrowDown":
				newIndex = Math.min(currentIndex + 1, items.length - 1);
				break;
			case "ArrowUp":
				newIndex = Math.max(currentIndex - 1, 0);
				break;
			case "Home":
				newIndex = 0;
				break;
			case "End":
				newIndex = items.length - 1;
				break;
		}

		if (newIndex == currentIndex) return;
		(items[currentIndex] as HTMLElement).tabIndex = -1;
		(items[newIndex] as HTMLElement).tabIndex = 0;
		(items[newIndex] as HTMLElement).focus();
	}

	function handleListFocusin(event: FocusEvent) {
		const list = event.currentTarget as HTMLElement;
		const items = Array.from(list.querySelectorAll("[role='option']"));
		const index = items.indexOf(event.target as Element);
		if (index == -1) return;
		for (let i = 0; i < items.length; i++) {
			(items[i] as HTMLElement).tabIndex = i == index ? 0 : -1;
		}
	}
</script>

<div class="flex h-full w-full flex-col bg-app-card">
	<div class="m-3 flex flex-row items-center rounded-xl border border-app-border bg-app-elevated">
		<MagnifyingGlass size={14} class="ml-3 mr-1 text-app-muted" />
		<input
			bind:value={query}
			class="w-full bg-transparent p-2 text-xs text-app-text"
			placeholder={$t("action_list.search_placeholder")}
			type="search"
			spellcheck="false"
		/>
	</div>

	<div class="mx-3 flex flex-row flex-wrap gap-2">
		<button class="action-chip" class:action-chip-active={activeCategory === null} on:click={() => (activeCategory = null)}>
			{$t("v2.actions.all")}
		</button>
		{#each categoryChips as category}
			<button class="action-chip" class:action-chip-active={activeCategory === category} on:click={() => (activeCategory = category)}>
				{category}
			</button>
		{/each}
	</div>

	<span id="action-list-hint" class="sr-only">{$t("action_list.hint")}</span>
	<div class="mt-3 grow overflow-auto select-none">
		{#if filteredCategories.length === 0}
			<div class="empty-state">{$t("v2.actions.empty")}</div>
		{/if}
		{#each filteredCategories as [categoryName, { actions }]}
			<div class="px-3 py-2 text-xs font-semibold uppercase tracking-wide text-app-muted">{categoryName}</div>
			<div
				role="listbox"
				aria-label={categoryName}
				aria-describedby="action-list-hint"
				tabindex="-1"
				on:keydown={handleListKeydown}
				on:focusin={handleListFocusin}
			>
				{#each actions as action, i}
					<div
						class="mx-2 mb-1 flex flex-row items-center gap-3 rounded-xl border border-transparent p-2 hover:border-app-border hover:bg-app-elevated cursor-grab active:cursor-grabbing"
						draggable="true"
						title={$localisations?.[action.plugin]?.[action.uuid]?.Tooltip ?? action.tooltip}
						role="option"
						aria-selected="false"
						tabindex={i == 0 ? 0 : -1}
						aria-label={$localisations?.[action.plugin]?.[action.uuid]?.Name ?? action.name}
						on:dragstart={(event) => {
							if (!event.dataTransfer) return;
							event.dataTransfer.effectAllowed = "copy";
							event.dataTransfer.setData("action", JSON.stringify(action));
						}}
						on:keydown={(event) => {
							if ((event.ctrlKey || event.metaKey) && event.key == "c") {
								copiedItem.set({ type: "action", action });
							}
						}}
					>
						<img
							src={!action.icon.startsWith("opendeck/") ? getWebserverUrl(action.icon) : action.icon.replace("opendeck", "")}
							alt=""
							class="size-8 shrink-0 rounded-[10px] border border-app-border pointer-events-none"
						/>
						<div class="min-w-0 flex-1">
							<div class="truncate text-xs text-app-text">{$localisations?.[action.plugin]?.[action.uuid]?.Name ?? action.name}</div>
							<div class="truncate text-[11px] text-app-muted">{categoryName}</div>
						</div>
					</div>
				{/each}
			</div>
		{/each}
	</div>

	<div class="shrink-0 border-t border-app-border p-3 text-xs text-app-muted">{$t("v2.actions.hint")}</div>
</div>

<style>
	.action-chip {
		border: 1px solid var(--color-app-border);
		border-radius: 999px;
		padding: 4px 10px;
		font-size: 11px;
		color: var(--color-app-muted);
	}

	.action-chip-active {
		background: #f5f5f7;
		color: #0b0b0e;
	}
</style>
