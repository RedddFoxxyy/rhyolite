import type { Theme } from "../../types/theme";

export const CrimsonNocturne: Theme = {
  name: "Crimson Nocturne",
  colorscheme: "dark",
  colors: {
    text: "#FADCD5", // lightest: used for main text
    subtext2: "#FADCD5", // using the same light tone for high–contrast subtext
    subtext1: "#765D67", // a lighter medium for secondary text
    subtext0: "#6D3C52", // a medium–dark tone for less prominent text
    overlay2: "#765D67", // matching subtext1 for overlays
    overlay1: "#6D3C52", // similar to subtext0 for tertiary elements
    overlay0: "#4B2138", // dark overlay for contrast
    surface2: "#765D67", // a mid–tone for slightly raised surfaces
    surface1: "#6D3C52", // consistent with your chosen surface0 tone
    surface0: "#6D3C52", // as you already set
    base: "#4B2138", // dark base background
    crust: "#1B0C1A", // darkest color for borders/accents
    mantle: "#2D222F", // very dark, for a deep backdrop
  },
};
