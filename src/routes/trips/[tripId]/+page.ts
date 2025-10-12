import { invoke } from "@tauri-apps/api/core";
import type { FullTripDetails } from "$lib/types/FullTripDetails";

export const load = async ({ params }) => {
  return {
    trip: (await invoke("fetch_full_trip", {
      id: Number(params.tripId),
    })) as FullTripDetails,
  };
};
