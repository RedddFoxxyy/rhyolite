import type { Tab } from "../types/tab";
import tabsStore from "../stores/tabs.store";
import { ApiProvider } from "./api.service";
import TabService from "./tab.service";
import type { Document } from "../types/document";
import { isValidJSON } from "../helpers/common.helper";
import type { Settings } from "../types/settings";
const apiProvider = new ApiProvider();

const getAllSettings = async (): Promise<Settings> => {
    const settings: Settings = await apiProvider.get_all_settings();
    return settings;
}

export default {
    getAllSettings
}