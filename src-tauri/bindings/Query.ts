export enum SearchActionType {
  Original = "original",
  Next = "next",
}
export interface OriginalSearchAction {
  type: SearchActionType.Original;
}

export interface NextSearchAction {
  type: SearchActionType.Next;
  nextUrl: string;
}

export type SearchAction = OriginalSearchAction | NextSearchAction;
