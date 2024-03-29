import { invoke } from "@tauri-apps/api";
import type { OpenWorkspaceEvent } from "./events";
import type { StoryBlock } from "./interfaces";

export async function updatePath(data: OpenWorkspaceEvent): Promise<void> {
  let path = data.path;
  await invoke("update_path", { newPath: path });
}

export async function saveFile(storyBlocks: StoryBlock[]): Promise<void> {
  await invoke("save_file", { storyBlocks: storyBlocks });
}

export async function loadFile(storyPath: string): Promise<StoryBlock[]> {
  try {
    return await invoke("load_file", { storyPath: storyPath });
  } catch (error) {
    return Promise.reject(error);
  }
}
