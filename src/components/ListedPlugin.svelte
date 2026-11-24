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
	export let description: string = "";
</script>

<div class="m-2 flex flex-col rounded-xl border border-app-border bg-app-panel p-3 hover:border-app-accent" class:hidden>
	<div class="flex items-start gap-3">
		<img src={icon} class="size-12 rounded-lg" class:opacity-75={disconnected} alt={name} loading="lazy" />
		<div class="min-w-0 flex-1">
			<p class="flex flex-wrap items-center gap-2 font-semibold text-app-text">
				<span class="truncate">{name}</span>
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
		<button
			class="flex flex-row items-center gap-1 rounded-lg border border-app-border bg-app-elevated px-2 py-1 text-xs text-app-text hover:text-app-accent"
			on:click={action}
			aria-label={actionLabel}
		>
			<slot />
		</button>
	</div>
</div>
