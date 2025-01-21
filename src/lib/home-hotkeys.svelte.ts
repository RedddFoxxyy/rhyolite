import DocumentService from "../services/document.service";
import TabService from "../services/tab.service";
import {commandPaletteStore} from "../stores/command-palette.svelte";
import {contentEditorStore} from "../stores/content-editor.svelte";

let activeKeys = new Set();

const handleKeyup = (event: KeyboardEvent): void => {
    // remove the keys from the activeKeys set
    activeKeys.delete(event.key)
}
const handleKeydown = (event: KeyboardEvent): void => {
    // check if key is already in the activeKeys set 
    if (!activeKeys.has(event.key)) {
        activeKeys.add(event.key);
    } else return;
    if (event.ctrlKey && event.key === "d") {
         event.preventDefault();
         DocumentService.deleteDocumentTab();
    }
    if (event.ctrlKey && event.key === "c") {
        event.preventDefault();
        TabService.closeTab();
    }
    if (event.ctrlKey && event.key === "n") {
        event.preventDefault();
        DocumentService.addNewDocumentTab();
    }
    if (event.ctrlKey && event.key === "t") {
        event.preventDefault();
        contentEditorStore.toggleToolbarVisibility();
    }
    if (
        (event.ctrlKey && event.key === "Tab") ||
        (event.ctrlKey && event.key === "PageDown")
    ) {
        event.preventDefault();
        TabService.cycleTabs();
    }
    if (event.ctrlKey && event.altKey && event.key === "c") {
        event.preventDefault();
        TabService.closeTab();
    }
    if (event.ctrlKey && event.key === "1") {
        event.preventDefault();
        TabService.gotoTab1();
    }
    if (event.ctrlKey && event.key === "9") {
        event.preventDefault();
        TabService.gotoLastTab();
    }
    if (event.ctrlKey && event.key === "p") {
        event.preventDefault();
        commandPaletteStore.toggleVisibility();
    }
}

window.addEventListener("keydown", handleKeydown)
window.addEventListener("keyup", handleKeyup);
