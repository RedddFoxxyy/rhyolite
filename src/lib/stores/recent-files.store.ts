import { type Writable, writable, get } from "svelte/store";

interface IFilesMenuStates {
  flagFilesMenuVisibility: boolean;
}

const states: Writable<IFilesMenuStates> = writable<IFilesMenuStates>({
  flagFilesMenuVisibility: false,
});

const isVisible = (): boolean => {
  const { flagFilesMenuVisibility }: IFilesMenuStates = get(states);
  return flagFilesMenuVisibility;
};

const toggleVisibility = (): boolean => {
  const { flagFilesMenuVisibility }: IFilesMenuStates = get(states);
  states.update((currentState) => ({
    flagFilesMenuVisibility: !currentState.flagFilesMenuVisibility,
  }));
  return !flagFilesMenuVisibility;
};

export default {
  states,
  isVisible,
  toggleVisibility,
};