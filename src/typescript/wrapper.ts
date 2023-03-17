import { invoke } from "@tauri-apps/api";
import type { OpenWorkspaceEvent } from "./events";

export async function updatePath(data: OpenWorkspaceEvent): Promise<void> {
  let path = data.path;
  await invoke("update_path", { newPath: path });
}
