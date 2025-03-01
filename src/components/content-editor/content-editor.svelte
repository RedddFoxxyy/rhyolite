<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import type { ChainedCommands } from "@tiptap/core";
  import { Extension } from "@tiptap/core";
  import Bold from "@tiptap/extension-bold";
  import CharacterCount from "@tiptap/extension-character-count";
  import { Color } from "@tiptap/extension-color";
  import Document from "@tiptap/extension-document";
  import FontFamily from "@tiptap/extension-font-family";
  import Highlight from "@tiptap/extension-highlight";
  import Image from "@tiptap/extension-image";
  import Link from "@tiptap/extension-link";
  import TextAlign from "@tiptap/extension-text-align";
  import TextStyle from "@tiptap/extension-text-style";
  import Typography from "@tiptap/extension-typography";
  import Underline from "@tiptap/extension-underline";
  import YouTube from "@tiptap/extension-youtube";
  import StarterKit from "@tiptap/starter-kit";
  import { Dropdown, DropdownItem, Tooltip } from "flowbite-svelte";
  import { onDestroy, onMount } from "svelte";
  import { createEditor, Editor, EditorContent } from "svelte-tiptap";
  import type { Readable } from "svelte/store";
  import CodeBlockLowlight from "@tiptap/extension-code-block-lowlight";
  import ListItem from "@tiptap/extension-list-item";
  import TaskItem from "@tiptap/extension-task-item";
  import TaskList from "@tiptap/extension-task-list";
  import Heading from "@tiptap/extension-heading";
  import Paragraph from "@tiptap/extension-paragraph";
  import { ChevronDownOutline, ChevronUpOutline } from "flowbite-svelte-icons";
  import { all, createLowlight } from "lowlight";
  import ContentEditorStore from "../../stores/content-editor.store";
  import documentservice from "../../services/document.service";
  // import ToolbarButton from "./components/toolbar-button.svelte";
  import {
    CustomHeader,
    CustomParagraph,
    CustomLineBreak,
    TabIndent,
  } from "./components/custom-extentions";

  let editor = $state() as Readable<Editor>;
  const lowlight = createLowlight(all);

  interface ContentEditorProps {
    content: any;
    onchange: (content: Editor) => void;
    class: string;
  }
  const {
    content,
    onchange,
    class: classProp = "",
  }: ContentEditorProps = $props();

  const setupEditor = () => {
    const FontSizeTextStyle = TextStyle.extend({
      addAttributes() {
        return {
          fontSize: {
            default: null,
            renderHTML: (attributes) => {
              if (!attributes.fontSize) {
                return {};
              }
              return { style: `font-size: ${attributes.fontSize}` };
            },
          },
        };
      },
    });

    editor = createEditor({
      extensions: [
        StarterKit.configure({
          bold: {
            HTMLAttributes: { class: "text-text font-bold" },
          },
          heading: false,
          blockquote: {
            HTMLAttributes: {
              class:
                "border-l-2 border-subtext2 m-1 p-4 w-fit text-text text-base bg-base/60 font-normal not-italic leading-none before:content-none after:content-none",
            },
          },
          paragraph: false,
          code: {
            HTMLAttributes: {
              class: "bg-surface0 text-text font-normal",
            },
          },
        }),
        CustomParagraph,
        CustomHeader,
        // CustomLineBreak,
        TabIndent,
        TextStyle,
        Color,
        FontSizeTextStyle,
        FontFamily,
        Highlight,
        Underline,
        Typography,
        Document,
        TaskList,
        TaskItem,
        ListItem,
        TaskItem.configure({
          HTMLAttributes: {
            class: "flex gap-2 leading-none", // Added leading-none and changed to items-start
          },
          nested: true,
        }),

        TaskList.configure({
          HTMLAttributes: {
            class: "not-prose pl-2",
          },
        }),
        Link.configure({
          openOnClick: false,
          autolink: true,
          defaultProtocol: "https",
        }),
        TextAlign.configure({
          types: ["heading", "paragraph"],
        }),
        CharacterCount.configure({
          textCounter: (text) => [...new Intl.Segmenter().segment(text)].length,
          wordCounter: (text) =>
            text.split(/\s+/).filter((word) => word !== "").length,
        }),
        CodeBlockLowlight.configure({
          lowlight,
          HTMLAttributes: {
            class: "bg-mantle text-text rounded-lg p-4", // Added bg-surface0
          },
        }),
        Image,
        YouTube,
      ],
      content,
      editorProps: {
        attributes: {
          class:
            "format text-text text-base focus:outline-none format-blue max-w-none leading-1",
        },
      },
      // onTransaction: ({ editor }) => {
      //   // force re-render so `editor.isActive` works as expected
      //   editor = editor;
      // },
      onCreate: ({ editor }) => {
        // Update counts when editor is initialized
        onchange(editor as Editor);
      },
      onUpdate: ({ editor }) => {
        onchange(editor as Editor);
      },
    });
  };

  onMount(() => {
    setupEditor();
    const docContentlisten = listen<any>("current_editor_content", (event) => {
      let documentContent = event.payload;
      console.log("Setting editor content from event");
      $editor.commands.clearContent();
      $editor.commands.setContent(documentContent);
    });
    return () => {
      docContentlisten.then((unsub) => unsub());
    };
  });

  const toggleMark = (mark: string) => {
    const focused: ChainedCommands = $editor.chain().focus();
    if (typeof (focused as any)[mark] === "function") {
      (focused as any)[mark]().run();
    }
  };

  const toggleLink = () => {
    const url: string | null = window.prompt("Enter image URL:", "https://");
    if (url) $editor.chain().focus().toggleLink({ href: url }).run();
  };

  const addImage = () => {
    const url = window.prompt("Enter image URL:", "https://");
    if (url) {
      $editor.chain().focus().setImage({ src: url }).run();
    }
  };

  const addYoutubeVideo = () => {
    const url = window.prompt(
      "Enter YouTube URL:",
      "https://www.youtube.com/watch?v=",
    );
    if (url) {
      $editor.commands.setYoutubeVideo({
        src: url,
        width: 640,
        height: 480,
      });
    }
  };

  let textSizeDropdownOpen = $state(false);
  const setTextSize = (event: Event) => {
    const fontSize: string | null = (
      event.target as HTMLInputElement
    ).getAttribute("data-text-size");
    // Apply the selected font size via pixels using the TipTap editor chain
    if (fontSize)
      $editor.chain().focus().setMark("textStyle", { fontSize }).run();
    //close dropdown after selection font size
    textSizeDropdownOpen = false;
  };

  let colorDropdownOpen = $state(false);
  const onInputColor = (event: Event) => {
    const selectedColor = (event.target as HTMLInputElement).value;
    // Apply the selected color to the selected text
    $editor.chain().focus().setColor(selectedColor).run();
    colorDropdownOpen = false;
  };
  const onClickColorHex = (event: Event) => {
    const selectedColor: string | null = (
      event.target as HTMLInputElement
    ).getAttribute("data-hex-color");
    // Apply the selected color to the selected text
    if (selectedColor) $editor.chain().focus().setColor(selectedColor).run();
    colorDropdownOpen = false;
  };

  const resetColor = () => {
    $editor.commands.unsetColor();
  };

  let fontFamilyDropdownOpen = $state(false);
  const onSelectFontFamily = (event: Event) => {
    const fontFamily: string | null = (
      event.target as HTMLInputElement
    ).getAttribute("data-font-family");

    // Apply the selected font size via pixels using the TipTap editor chain
    if (fontFamily) $editor.chain().focus().setFontFamily(fontFamily).run();

    // Hide the dropdown after selection
    fontFamilyDropdownOpen = false;
  };

  const setTextAlign = (align: "left" | "right" | "center") => {
    $editor.chain().focus().setTextAlign(align).run();
  };

  let typographyDropdownOpen = $state(false);
  const onSelectHeading = (event: Event) => {
    const headingLevel: string | null = (
      event.target as HTMLInputElement
    ).getAttribute("data-heading-level");
    if (headingLevel) {
      const level = parseInt(headingLevel) as 1 | 2 | 3 | 4 | 5 | 6;
      $editor.chain().focus().toggleHeading({ level }).run();
    }
    typographyDropdownOpen = false;
  };
  const onSelectParagraph = () => {
    $editor.chain().focus().setParagraph().run();
    typographyDropdownOpen = false;
  };

  let flagToolbarVisibility = $state(false);
  const unsubscribeStates = ContentEditorStore.states.subscribe((value) => {
    flagToolbarVisibility = value.flagToolbarVisibility;
  });

  onDestroy(unsubscribeStates); // Clean up
</script>

<div class={`flex flex-col h-full w-full text-text ${classProp}`}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="grow px-4 py-2 rounded-b-lg"
    onclick={() => $editor.commands.focus()}
    onfocus={() => $editor.commands.focus()}
    role="textbox"
    tabindex="-1"
  >
    <EditorContent class="cursor-text h-full text-text" editor={$editor} />
  </div>
</div>
