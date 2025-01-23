<script lang="ts">
  import {
    ChevronRight,
    SlidersHorizontal,
    Palette,
    Keyboard,
    Info,
  } from "lucide-svelte";
  import {settingsMenuStore} from "../stores/settings-menu.svelte";
  import {themeStore} from "../stores/theme.svelte";
  import type { Theme } from "../types/theme";

  let showThemeOptions = $state(false);
  let self: HTMLElement | null = $state(null);
  let originalTheme: Theme | undefined;

  const layout = {
    position: { top: 150, left: 44, bottom: 15 },
    dimensions: { width: 200, height: 200 },
  };

  const menuButtons = [
    {
      label: "General Settings",
      icon: SlidersHorizontal,
      onClick: () => console.log("Opening General Settings...")
    },
    {
      label: "Theme",
      icon: Palette,
      onClick: () => (showThemeOptions = !showThemeOptions),
      hasSubmenu: true,
    },
    {
      label: "Keyboard Shortcuts",
      icon: Keyboard,
      onClick: () => console.log("Opening Keyboard Shortcuts...")
    },
    {
      label: "About",
      icon: Info,
      onClick: () => console.log("Opening About...")
    }
  ];

  const handleCloseEvent = (e: MouseEvent | KeyboardEvent) => {
    if (
      (e instanceof MouseEvent && !self?.contains(e.target as Node)) ||
      (e instanceof KeyboardEvent && e.key === "Escape")
    ) {
      e.stopPropagation();
      settingsMenuStore.toggleVisibility();
    }
  };

  // Store the original theme when opening the menu
  const storeOriginalTheme = () => {
    const theme = themeStore.getCurrentTheme();
    // this is a workaround
    // currently svelte 5 seems to have a bug (sveltejs/svelte issue #13562) where a structuredClone does not actually work properly
    // so I've had to do it manually
    originalTheme = {
      name: theme.name,
      colors: structuredClone(theme.colors),
      colorscheme: theme.colorscheme,
    };
  };

  // Preview theme on hover
  const previewTheme = (theme: Theme) => {
    console.log(originalTheme);
    themeStore.updateCurrentTheme(theme);
    console.log(theme);
  };

  // Restore original theme when mouse leaves
  const resetTheme = () => {
    if (originalTheme) {
      themeStore.updateCurrentTheme(originalTheme);
    }
  };

  $effect(() => {
    if (settingsMenuStore.isVisible()) {
      document.addEventListener("click", handleCloseEvent);
      document.addEventListener("keydown", handleCloseEvent);
      storeOriginalTheme(); // Store original theme when opening menu
    } else {
      document.removeEventListener("click", handleCloseEvent);
      document.removeEventListener("keydown", handleCloseEvent);
      showThemeOptions = false;
      resetTheme(); // Reset to original theme when closing without selecting
    }

    return () => {
      document.removeEventListener("click", handleCloseEvent);
      document.removeEventListener("keydown", handleCloseEvent);
      resetTheme(); // Ensure theme is reset if component is destroyed while previewing
    }
  })

  // Apply theme and close menu
  const changeTheme = (theme: Theme) => {
    themeStore.updateCurrentTheme(theme);
    originalTheme = theme; // Update original theme to the new selection
    settingsMenuStore.toggleVisibility();
  };
</script>

{#if settingsMenuStore.isVisible()}
  {@const settingsVisible = settingsMenuStore.isVisible()}
  <div
    bind:this={self}
    class="absolute rounded-lg p-1 pt-[6px] z-50 transition-all duration-300 transform bg-base shadow-xl"
    class:translate-y-0={settingsVisible}
    class:opacity-100={settingsVisible}
    class:translate-y-5={!settingsVisible}
    class:opacity-0={!settingsVisible}
    style="bottom: {layout.position.bottom}px; left: {layout.position
      .left}px; width: {layout.dimensions.width}px;"
  >
    {#each menuButtons as { label, icon: Icon, onClick, hasSubmenu }}
      <button
        class="w-full p-1 rounded-lg text-left text-text cursor-pointer transition-all duration-300 text-sm hover:bg-surface1 focus:bg-surface1 flex flex-row justify-between items-center"
        onclick={onClick}
      >
        <div class="flex flex-row gap-1.5 items-center">
          <Icon class="w-4 h-4" />
          {label}
        </div>
        {#if hasSubmenu}
          <ChevronRight class="w-4 h-4" />
        {/if}
      </button>
    {/each}

    {#if showThemeOptions}
      <div
        role="menu"
        tabindex="0"
        class="absolute left-full rounded-lg p-1 bottom-[58%] mt-8 ml-1 w-max bg-base shadow-xl"
        style="width: {layout.dimensions.width}px;"
        onmouseleave={resetTheme}
      >
        {#each themeStore.getThemes() as theme}
          <button
            class="w-full p-1 rounded-lg text-left text-text bg-transparent cursor-pointer transition-all duration-300 text-sm hover:bg-surface1 focus:bg-surface1"
            onmouseenter={() => previewTheme(theme)}
            onclick={() => changeTheme(theme)}
          >
            {theme.name}
          </button>
        {/each}
      </div>
    {/if}
  </div>
{/if}
