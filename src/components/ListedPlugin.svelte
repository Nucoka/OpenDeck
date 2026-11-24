<script lang="ts">
	import { t } from "$lib/i18n";

	export let icon: string;
	export let name: string;
	export let subtitle: string;
	export let hidden: boolean = false;
	export let disconnected: boolean = false;
	export let action: () => void;
	export let actionLabel: string = "";
	export let secondaryAction: (() => void) | undefined = undefined;
	export let secondaryActionLabel: string = "";
	export let status: string = "running";
	export let exitCode: number | null = null;
</script>

<div class="flex flex-row items-center m-2 p-2 bg-neutral-700 hover:bg-neutral-600 transition-colors border border-neutral-600 rounded-lg" class:hidden>
	<img src={icon} class="w-24 h-24 rounded-lg" class:opacity-75={disconnected} alt={name} loading="lazy" />
	<div class="ml-4 mr-2 text-neutral-300 wrap-anywhere" class:opacity-75={disconnected}>
		<p class="flex flex-row items-center font-semibold">
			{name}
			<span
				class="ml-2 px-1 text-xs rounded border border-neutral-500"
				class:text-green-500={status === "running"}
				class:text-yellow-500={status === "crashed" || status === "restarting"}
				class:text-red-500={status === "failed"}
				title={$t(`plugin_manager.status.${status}`)}
			>
				{$t(`plugin_manager.status.${status}`)}{#if (status === "crashed" || status === "failed") && exitCode !== null}
					({exitCode}){/if}
			</span>
		</p>
		<slot name="subtitle">{subtitle}</slot>
	</div>

	<div class="flex flex-col items-end ml-auto mr-4">
		<slot name="controls" />
		{#if secondaryAction}
			<button on:click={secondaryAction} aria-label={secondaryActionLabel}>
				<slot name="secondary" />
			</button>
		{/if}
		<button on:click={action} aria-label={actionLabel}>
			<slot />
		</button>
	</div>
</div>
