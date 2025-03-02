export interface Settings {
    general: GeneralSettings;
    appearance: AppearanceSettings;
    shortcuts: ShortcutsSettings;
    sync: SyncSettings;
  }
  
  interface GeneralSettings {
    theme: string;
    language: string;
    checkbox_test: boolean;
  }
  
  interface AppearanceSettings {
    theme: string;
    app_font: string;
    app_font_size: string;
    app_font_weight: string;
    editor_font: string;
    editor_font_size: string;
    editor_font_weight: string;
  }
  
  interface ShortcutsSettings {
    theme: string;
    checkbox_test: boolean;
  }
  
  interface SyncSettings {}
  
  
  // Optional: Default settings object
  const defaultSettings: Settings = {
    general: {
      theme: "Light",
      language: "English",
      checkbox_test: false
    },
    appearance: {
      theme: "Light",
      app_font: "Roboto",
      app_font_size: "Medium",
      app_font_weight: "Regular",
      editor_font: "Courier New",
      editor_font_size: "Medium",
      editor_font_weight: "Regular"
    },
    shortcuts: {
      theme: "Light",
      checkbox_test: false
    },
    sync: {}
  };