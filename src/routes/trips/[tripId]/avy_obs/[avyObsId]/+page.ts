import { invoke } from "@tauri-apps/api/core";
import type { AvyObservation } from "$lib/types/AvyObservation.js";

export const load = async ({ params }) => {
  return {
    avyObs: (await invoke("fetch_avy_observation", {
      id: Number(params.avyObsId),
    })) as AvyObservation,
  };
};
