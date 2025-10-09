import { invoke } from "@tauri-apps/api/core";
import type { Trip } from "$lib/types/Trip";

export const load = async () => {
  return { trips: (await invoke("trip_list")) as Trip[] };
};
