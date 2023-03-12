import { writable, type Writable } from "svelte/store";

export const activeInputId: Writable<number> = writable(null);
