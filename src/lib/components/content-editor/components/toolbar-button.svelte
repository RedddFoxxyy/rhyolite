<script lang="ts">
	import { Tooltip } from "flowbite-svelte";

	export let onclick: () => void;
	export let tooltipText: string;
</script>

<button type="button" {onclick} class="custom-btn-toolbar">
	<slot></slot>
	<span class="sr-only">{tooltipText}</span>
</button>
<Tooltip>
	{tooltipText}
</Tooltip>
