import { invoke } from "@tauri-apps/api/core";
import type { Trip } from "$lib/utils/types";

export const load = async () => {
  return { trips: (await invoke("trip_list")) as Trip[] };
};
