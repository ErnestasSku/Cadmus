export interface FoundLinkEvent {
  target: HTMLElement;
  link: number;
}

export interface UpdateConnectionLinesEvent {
  storyElementId: number;
  top: number;
  left: number;
}
