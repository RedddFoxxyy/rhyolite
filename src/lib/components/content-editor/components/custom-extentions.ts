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

const CustomHeader = Heading.configure({
  levels: [1, 2, 3, 4, 5, 6],
}).extend({
  renderHTML({ node, HTMLAttributes }) {
    const level = node.attrs.level as 1 | 2 | 3 | 4 | 5 | 6;
    const sizes = {
      1: "text-4xl text-text font-extrabold my-3",
      2: "text-3xl text-text font-extrabold my-3",
      3: "text-2xl text-text font-extrabold my-3",
      4: "text-xl text-text font-extrabold my-3",
      5: "text-lg text-text font-extrabold my-3",
      6: "text-sm text-text font-extrabold my-3",
    };

    const className = `${HTMLAttributes.class || ""} ${sizes[level]}`.trim();

    return [`h${level}`, { ...HTMLAttributes, class: className }, 0];
  },
});

const CustomParagraph = Paragraph.extend({
  renderHTML({ HTMLAttributes }) {
    return ["div", { ...HTMLAttributes, class: "editor-line" }, 0];
  },
});

const CustomLineBreak = Extension.create({
  addKeyboardShortcuts() {
    return {
      Enter: () => {
        const { state, dispatch } = this.editor.view;

        // Create a new div with editor-line class instead of paragraph
        const node = state.schema.nodes.paragraph.create(
          {
            class: "editor-line",
          },
          state.selection.content().content,
        );

        const tr = state.tr.replaceSelectionWith(node).scrollIntoView();
        dispatch(tr);

        return true;
      },
      "Shift-Enter": () => {
        // Default paragraph behavior
        return false;
      },
    };
  },
});

const TabIndent = Extension.create({
  addKeyboardShortcuts() {
    return {
      Tab: () => {
        const { state, dispatch } = this.editor.view;
        const { from, to } = state.selection;
        const indent = "    "; // Four spaces

        // Insert four spaces at the current selection
        dispatch(state.tr.insertText(indent, from, to));
        return true; // Prevents the default behavior
      },
      "Shift-Tab": () => {
        const { state, dispatch } = this.editor.view;
        const { from, to } = state.selection;
        const indent = "    "; // Four spaces
        const startOfLine = state.doc.resolve(from).start();

        // Check if the selection starts with four spaces
        if (state.doc.textBetween(startOfLine, from).startsWith(indent)) {
          // Remove four spaces before the current selection
          dispatch(state.tr.delete(startOfLine, startOfLine + indent.length));
        }
        return true; // Prevents the default behavior
      },
    };
  },
});

export { CustomHeader, CustomParagraph, CustomLineBreak, TabIndent };
