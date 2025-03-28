<script>
	import "../app.css";
	import { attachHotkeyListener } from "$lib/hotkeys.svelte";

	attachHotkeyListener();
</script>

<slot />
