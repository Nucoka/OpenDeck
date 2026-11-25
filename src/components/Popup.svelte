<script lang="ts">
	import { onDestroy, tick } from "svelte";

	export let show = false;
	export let label = "";
	export let inline = false;

	let popupEl: HTMLDivElement;
	let previousFocus: HTMLElement | null = null;

	$: if (show && !inline) {
		previousFocus = document.activeElement as HTMLElement | null;
		tick().then(() => popupEl?.focus());
	} else if (previousFocus) {
		previousFocus.focus();
		previousFocus = null;
	}

	onDestroy(() => previousFocus?.focus());
</script>

{#if inline || show}
	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<div
		bind:this={popupEl}
		class={inline
			? "glass flex min-h-full w-full flex-col overflow-visible rounded-[20px] border border-app-border bg-app-card/90"
			: "glass absolute left-0 top-0 m-2 h-[calc(100%-1rem)] w-[calc(100%-1rem)] overflow-auto rounded-[20px] border border-app-border bg-app-card/90 p-4 z-30"}
		class:flex={!inline && ($$slots.header || $$slots.footer)}
		class:flex-col={!inline && ($$slots.header || $$slots.footer)}
		role={inline ? "region" : "dialog"}
		tabindex={inline ? undefined : -1}
		aria-label={label}
	>
		{#if $$slots.header}
			<div class="shrink-0">
				<slot name="header" />
			</div>
		{/if}
		<div class="min-h-0 flex-1" class:overflow-auto={!inline && ($$slots.header || $$slots.footer)}>
			<slot />
		</div>
		{#if $$slots.footer}
			<div class="shrink-0">
				<slot name="footer" />
			</div>
		{/if}
	</div>
{/if}
