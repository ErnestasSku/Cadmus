export interface FoundLinkEvent {
  target: HTMLElement;
  link: number;
}

export interface UpdateConnectionLinesEvent {
  storyElementId: number;
  top: number;
  left: number;
}

export interface OpenWorkspaceEvent {
  new: boolean;
  path: string;
}
