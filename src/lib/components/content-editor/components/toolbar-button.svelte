<script lang="ts">
	import { Tooltip } from "flowbite-svelte";
	import type { Snippet } from "svelte";

	type Props = {
		onclick: any;
		tooltipText: string;
		children: Snippet;
	};

	let { onclick, tooltipText, children }: Props = $props();
</script>

<button type="button" {onclick} class="custom-btn-toolbar">
	{@render children()}
	<span class="sr-only">{tooltipText}</span>
</button>
<Tooltip>
	{tooltipText}
</Tooltip>
