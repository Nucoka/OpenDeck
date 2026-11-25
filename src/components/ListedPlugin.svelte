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
	export let status: string = "";
	export let exitCode: number | null = null;
	export let description: string = "";
	export let actionVisible: boolean = true;
	export let compact: boolean = false;
</script>

<div class="m-2 flex flex-col rounded-[18px] border border-app-border bg-app-card p-3 hover:border-app-accent" class:hidden class:compact-row={compact}>
	<div class="flex items-start gap-3">
		<img src={icon} class="size-12 rounded-lg" class:opacity-75={disconnected} alt={name} loading="lazy" />
		<div class="min-w-0 flex-1">
			<p class="flex flex-wrap items-center gap-2 font-semibold text-app-text">
				<span class="truncate">{name}</span>
				{#if status}
					<span
						class="rounded border border-app-border px-1 text-xs"
						class:text-app-success={status === "running"}
						class:text-app-warning={status === "crashed" || status === "restarting"}
						class:text-app-danger={status === "failed"}
						title={$t(`plugin_manager.status.${status}`)}
					>
						{$t(`plugin_manager.status.${status}`)}{#if (status === "crashed" || status === "failed") && exitCode !== null}
							({exitCode}){/if}
					</span>
				{/if}
			</p>
			<div class="mt-0.5 text-xs text-app-muted" class:opacity-75={disconnected}>
				<slot name="subtitle">{subtitle}</slot>
			</div>
			{#if description}
				<p class="mt-1 line-clamp-2 text-xs text-app-muted">{description}</p>
			{/if}
		</div>
	</div>

	<div class="mt-3 flex flex-row flex-wrap items-center gap-2 border-t border-app-border pt-3">
		<slot name="controls" />
		{#if secondaryAction}
			<button
				class="flex flex-row items-center gap-1 rounded-lg border border-app-border bg-app-elevated px-2 py-1 text-xs text-app-text hover:text-app-accent"
				on:click={secondaryAction}
				aria-label={secondaryActionLabel}
			>
				<slot name="secondary" />
			</button>
		{/if}
		{#if actionVisible}
			<button
				class="flex flex-row items-center gap-1 rounded-lg border border-app-border bg-app-elevated px-2 py-1 text-xs text-app-text hover:text-app-accent"
				on:click={action}
				aria-label={actionLabel}
				title={actionLabel}
			>
				<slot />
			</button>
		{/if}
	</div>
</div>

<style>
	.compact-row {
		flex-direction: row;
		align-items: center;
		gap: 12px;
	}

	.compact-row > div:first-child {
		flex: 1 1 auto;
	}

	.compact-row img {
		width: 36px;
		height: 36px;
		border-radius: 10px;
	}

	.compact-row > div:last-child {
		flex-direction: row;
		align-items: center;
		margin-top: 0;
		border-top: 0;
		padding-top: 0;
	}
</style>
