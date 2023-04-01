export interface StoryBlock {
  index: number;
  top: number;
  left: number;
  connections: Connection[];
  initializing: boolean;
  storyId: string;
  storyContent: string;
}

export interface Connection {
  index: number;
  connectedElementId: number;
  pathLabel: string;
  pathDescription: string;
  startX: number;
  startY: number;
  endX: number;
  endY: number;
  empty: boolean;
  connected: boolean;
  visible: boolean;
}
