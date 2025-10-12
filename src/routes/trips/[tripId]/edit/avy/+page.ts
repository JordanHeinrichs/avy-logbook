import { invoke } from "@tauri-apps/api/core";
import type { AvalancheForecast } from "$lib/types/AvalancheForecast";

export const load = async ({ params }) => {
  return {
    avy: (await invoke("fetch_avy_forecast", {
      tripId: Number(params.tripId),
    })) as AvalancheForecast,
  };
};
