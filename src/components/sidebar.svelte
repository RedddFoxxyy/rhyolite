<script lang="ts">
  import CommandIcon from "$lib/static/command_palette.svg.svelte";
  import RecentFilesIcon from "$lib/static/recent_files.svg.svelte";
  import { getContext } from "svelte";
  import {commandPaletteStore} from "../stores/command-palette.svelte";
  import {recentFilesStore} from "../stores/recent-files.svelte";
  import SettingsIcon from "$lib/static/settings.svg.svelte";
  import {settingsMenuStore} from "../stores/settings-menu.svelte";
  import SettingsMenu from "./settingsmenu.svelte";
  const editor: any = getContext("editor");
</script>

<main class="h-full basis-[40px] grow-0 shrink-0 border-r-2 border-r-surface0">
  <div class="flex flex-col justify-between h-full">
    <div class="flex flex-col mt-[10px] py-[10px] px-[2px] gap-3">
      <button
        class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-gray-200/10 transition-all duration-200 focus:outline-none focus:ring-0"
        id="CommandPalette_button"
        onclick={(e) => {
          e.stopPropagation();
          commandPaletteStore.toggleVisibility();
        }}
        aria-label="Open Command Palette"
        title="Open Command Palette"
      >
        <CommandIcon />
      </button>
      <button
        class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-text/10 transition-all duration-200 focus:outline-none focus:ring-0"
        id="Files_Menu_button"
        onclick={() => recentFilesStore.toggleVisibility()}
        aria-label="Open Files Menu"
        title="Open Files Menu"
      >
        <RecentFilesIcon />
      </button>
    </div>
    <div class="flex flex-col mb-[5px] py-[10px] px-[2px] gap-3">
      <button
        class="flex justify-center rounded-lg items-center h-5 px-0.5 py-4 w-full cursor-pointer hover:bg-text/10 transition-all duration-200 focus:outline-none focus:ring-0"
        id="Settings_button"
        onclick={(e) => {
          e.stopPropagation();
          settingsMenuStore.toggleVisibility();
        }}
        aria-label="Open Settings Menu"
        title="Open Settings Menu"
      >
        <SettingsIcon />
      </button>
      <SettingsMenu />
    </div>
  </div>
</main>
