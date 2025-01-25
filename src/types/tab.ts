export interface Tab {
    id: string;
    title: string;
    tabType?: TabType;
    documentId?: string;
    /** a workaround to mark when a tab is being deleted */
    deleting?: boolean
}

export enum TabType {
    Document = "Document",
    Other = "Other",
}
