<script lang="ts">
  import CommandPaletteStore from "../stores/command-palette.store";
  import DocumentService, {
    runDummyCommand,
  } from "../services/document.service";
  import TabService from "../services/tab.service";
  import { onDestroy } from "svelte";
  import ContentEditorStore from "../stores/content-editor.store";
  import { listen } from "@tauri-apps/api/event";

  let selectedIndex: number = $state(-1);
  let searchText: string = $state("");

  interface Command {
    name: string;
    shortcut: string;
    action: () => void;
  }

  $effect(() => {
    const unlisten = listen("dummy-event", (...args) => {
      console.log("dummy-event fired with ", args);
    });
    return () => unlisten.then((s) => s());
  });

  let commands: Command[] = [
    {
      name: "Delete Tab",
      shortcut: "Ctrl + D",
      action: () => {
        DocumentService.deleteDocumentTab();
        CommandPaletteStore.toggleVisibility();
      },
    },
    {
      name: "Execute dummy command",
      shortcut: "Ctrl + D",
      action: () => {
        runDummyCommand({ cmd: "dummy", payload: { hi: 2 } });
      },
    },
    {
      name: "Close Tab",
      shortcut: "Ctrl + C",
      action: () => {
        TabService.closeTab();
        CommandPaletteStore.toggleVisibility();
      },
    },
    {
      name: "New Tab",
      shortcut: "Ctrl + N",
      action: () => {
        DocumentService.addNewDocumentTab();
        CommandPaletteStore.toggleVisibility();
      },
    },
    {
      name: "Next Tab",
      shortcut: "Ctrl + Tab or Ctrl + pgDown",
      action: () => {
        TabService.cycleTabs();
        CommandPaletteStore.toggleVisibility();
      },
    },
    {
      name: "Go to First Tab",
      shortcut: "Ctrl + 1",
      action: () => {
        TabService.gotoTab1();
        CommandPaletteStore.toggleVisibility();
      },
    },
    {
      name: "Go to Last Tab",
      shortcut: "Ctrl + 9",
      action: () => {
        TabService.gotoLastTab();
        CommandPaletteStore.toggleVisibility();
      },
    },
    {
      name: "Toggle ToolBar",
      shortcut: "Ctrl + T",
      action: () => {
        ContentEditorStore.toggleToolbarVisibility();
        CommandPaletteStore.toggleVisibility();
      },
    },
  ];

  function handleKeydown(event: KeyboardEvent) {
    if (!CommandPaletteStore.isVisible()) return;
    console.log(event.key, event.shiftKey, event);
    if (
      event.key === "ArrowDown" ||
      (event.code === "Tab" && !event.shiftKey)
    ) {
      event.preventDefault();
      if (selectedIndex === -1) {
        selectedIndex = 0;
      } else {
        selectedIndex = (selectedIndex + 1) % commands.length;
      }
    }
    if (event.key === "ArrowUp" || (event.code === "Tab" && event.shiftKey)) {
      event.preventDefault();
      if (selectedIndex === -1) {
        selectedIndex = commands.length - 1;
      } else {
        selectedIndex = (selectedIndex - 1 + commands.length) % commands.length;
      }
    }
    if (event.key === "Enter") {
      event.preventDefault();
      if (selectedIndex >= 0 && selectedIndex < commands.length) {
        commands[selectedIndex].action();
      }
    }
    if (event.key === "Escape") {
      event.preventDefault();
      CommandPaletteStore.toggleVisibility();
    }
  }
  $effect(() => {
    if (selectedIndex !== -1)
      document
        .querySelector(
          `#command-palette-options > button:nth-child(${selectedIndex + 1})`,
        )
        ?.scrollIntoView({ behavior: "smooth", block: "nearest" });
  });

  function handleWheel(event: WheelEvent) {
    if (!CommandPaletteStore.isVisible()) return;

    event.preventDefault();
    if (event.deltaY > 0) {
      // Scrolling down
      selectedIndex = (selectedIndex + 1) % commands.length;
    } else {
      // Scrolling up
      selectedIndex = (selectedIndex - 1 + commands.length) % commands.length;
    }
  }

  // Reset selected index when command palette is closed
  $effect(() => {
    if (!CommandPaletteStore.isVisible()) {
      selectedIndex = -1;
      searchText = "";
    }
  });

  let flagVisibility = $state(false);
  $effect(() => {
    if (flagVisibility) {
      (
        document.querySelector("#commandPaletteTextarea") as HTMLTextAreaElement
      ).focus();
    }
  });
  const unsubscribeStates = CommandPaletteStore.states.subscribe((value) => {
    flagVisibility = value.flagCommandPaletteVisibility;
  });
  onDestroy(unsubscribeStates); // Clean up
</script>

<!-- <svelte:window on:keydown={handleKeydown} /> -->

{#if flagVisibility}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="fixed top-0 left-0 w-full h-full bg-black/60 z-20"
    tabindex="-1"
    aria-modal="true"
    role="dialog"
    onclick={(e) => {
      if (e.target === e.currentTarget) CommandPaletteStore.toggleVisibility();
    }}
  >
    <div
      class="fixed top-[40%] left-1/2 flex flex-col bg-crust rounded-lg p-3 z-[60] w-min-[200px] w-[50%] h-fit min-h-[100px] max-h-[400px] gap-2 -translate-x-1/2 -translate-y-1/2 overflow-hidden"
    >
      <div class="relative basis-[42px] w-full shrink-0 mb-2 overflow-hidden">
        <textarea
          id="commandPaletteTextarea"
          class="w-full h-full overflow-hidden resize-none pr-8 p-2 cursor-text text-text bg-surface0 text-left box-border border-2 hover:border-subtext0 outline-none rounded transition-all duration-200 border-overlay0 focus:border-subtext0 focus:outline-none focus:ring-0"
          placeholder="Select a Command"
          bind:value={searchText}
          onkeydown={handleKeydown}
        ></textarea>
        <button
          class="absolute right-2 top-1/2 -translate-y-1/2 bg-transparent text-text opacity-70 hover:opacity-100 transition-opacity duration-200"
          onclick={() => CommandPaletteStore.toggleVisibility()}>✕</button
        >
      </div>
      <div
        id="command-palette-options"
        class="flex overflow-y-auto flex-col gap-[0.5px]"
      >
        <div class="mx-2">
          {#each commands as command, index}
            <button
              type="button"
              class="flex px-4 justify-between items-center p-1 hover:bg-surface0 cursor-pointer w-full h-[34px] text-left text-text border-none shadow-none rounded transition-colors duration-200"
              class:bg-surface0={selectedIndex === index}
              onclick={() => {
                command.action();
              }}
              onmouseenter={() => (selectedIndex = index)}
            >
              <span>{command.name}</span>
              <span class="shortcut">{command.shortcut}</span>
            </button>
          {/each}
        </div>
      </div>
    </div>
  </div>
{/if}
