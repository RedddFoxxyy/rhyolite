
//TODO: Implement tabType either on the backend
//or on the frontend.
export interface Tab {
  id: string;
  title: string;
  // tabType?: TabType;
  // documentId?: string;
}

export enum TabType {
  Document = "Document",
  Other = "Other",
}
