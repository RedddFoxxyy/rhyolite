export interface Theme {
  name: string;
  colorscheme: "light" | "dark";
  colors: {
    text: string;
    subtext2: string;
    subtext1: string;
    subtext0: string;
    overlay2: string;
    overlay1: string;
    overlay0: string;
    surface2: string;
    surface1: string;
    surface0: string;
    base: string;
    crust: string;
    mantle: string;
  };
}
