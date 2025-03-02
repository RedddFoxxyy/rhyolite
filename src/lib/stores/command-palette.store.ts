import { type Writable, writable, get } from 'svelte/store';

interface ICommandPaletteStates {
    flagCommandPaletteVisibility: boolean;
}

const states: Writable<ICommandPaletteStates> = writable<ICommandPaletteStates>({
    flagCommandPaletteVisibility: false,
});

const isVisible = (): boolean => {
    const { flagCommandPaletteVisibility }: ICommandPaletteStates = get(states);
    return flagCommandPaletteVisibility;
}

const toggleVisibility = (): boolean => {
    const { flagCommandPaletteVisibility }: ICommandPaletteStates = get(states);
    states.update(() => ({
        flagCommandPaletteVisibility: !flagCommandPaletteVisibility,
    }));
    return !flagCommandPaletteVisibility;
}

export default {
    states,
    isVisible,
    toggleVisibility,
}