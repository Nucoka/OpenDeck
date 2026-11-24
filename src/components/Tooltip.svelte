<script>
	import Question from "phosphor-svelte/lib/Question";

	let show = false;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="inline-block cursor-help" on:mouseenter={() => (show = true)} on:mouseleave={() => (show = false)}>
	<Question size="24" class="text-app-muted" />
	{#if show}
		<div class="absolute mt-2 p-2 w-96 text-xs text-app-text bg-app-elevated border border-app-border rounded-lg">
			<slot />
		</div>
	{/if}
</div>
