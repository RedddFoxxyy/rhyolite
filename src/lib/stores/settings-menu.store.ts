import { writable, get } from 'svelte/store';

interface ISettingsMenuState {
  settingsMenuVisible: boolean;
}

const settingsMenuState = writable<ISettingsMenuState>({
  settingsMenuVisible: false,
});

const isSettingsMenuVisible = (): boolean => {
  const { settingsMenuVisible }: ISettingsMenuState = get(settingsMenuState);
  return settingsMenuVisible;
};

const toggleSettingsMenu = (): void => {
  settingsMenuState.update(state => ({
    settingsMenuVisible: !state.settingsMenuVisible,
  }));
};

export default {
  subscribe: settingsMenuState.subscribe,
  settingsMenuState,
  isSettingsMenuVisible,
  toggleSettingsMenu,
};
