import { type Writable, writable, get } from 'svelte/store';

interface IContentEditorStates {
    flagToolbarVisibility: boolean;
}

const states: Writable<IContentEditorStates> = writable<IContentEditorStates>({
    flagToolbarVisibility: false,
});

const isToolbarVisible = (): boolean => {
    const { flagToolbarVisibility }: IContentEditorStates = get(states);
    return flagToolbarVisibility;
}

const toggleToolbarVisibility = (): boolean => {
    const { flagToolbarVisibility }: IContentEditorStates = get(states);
    states.update(() => ({
        flagToolbarVisibility: !flagToolbarVisibility,
    }));
    return !flagToolbarVisibility;
}

export default {
    states,
    isToolbarVisible,
    toggleToolbarVisibility,
}