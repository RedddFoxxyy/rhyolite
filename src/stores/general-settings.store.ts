import { type Writable, writable, get } from 'svelte/store';

interface IGeneralSettingsStates {
    flagGeneralSettingsVisibility: boolean;
}

const states: Writable<IGeneralSettingsStates> = writable<IGeneralSettingsStates>({
    flagGeneralSettingsVisibility: false,
});

const isVisible = (): boolean => {
    const { flagGeneralSettingsVisibility }: IGeneralSettingsStates = get(states);
    return flagGeneralSettingsVisibility;
}

const toggleVisibility = (): boolean => {
    const { flagGeneralSettingsVisibility }: IGeneralSettingsStates = get(states);
    states.update(() => ({
        flagGeneralSettingsVisibility: !flagGeneralSettingsVisibility,
    }));
    return !flagGeneralSettingsVisibility;
}

export default {
    states,
    isVisible,
    toggleVisibility,
}