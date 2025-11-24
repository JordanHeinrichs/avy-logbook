import { invoke } from "@tauri-apps/api/core";
import type { AvalancheForecast } from "$lib/types/AvalancheForecast";

export const load = async ({ params, url }) => {
  return {
    avy: (await invoke("fetch_avy_forecast", {
      tripId: Number(params.tripId),
    })) as AvalancheForecast,
    wizard: !!url.searchParams.get("wizard"),
  };
};
