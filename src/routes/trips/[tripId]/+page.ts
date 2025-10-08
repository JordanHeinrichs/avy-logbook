import { invoke } from "@tauri-apps/api/core";
import type { Trip } from "$lib/utils/types";

export const load = async ({ params }) => {
  return {
    trip: (await invoke("fetch_trip", { id: Number(params.tripId) })) as Trip,
  };
};
