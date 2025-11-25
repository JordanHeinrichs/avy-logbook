import { invoke } from "@tauri-apps/api/core";
import type { AvalancheForecast } from "$lib/types/AvalancheForecast";
import type { AvalancheProblem } from "$lib/types/AvalancheProblem";

export const load = async ({ params, url }) => {
  const avy: AvalancheForecast = await invoke("fetch_avy_forecast", {
    tripId: Number(params.tripId),
  });
  return {
    avy: avy,
    wizard: !!url.searchParams.get("wizard"),
    problems: (await invoke("fetch_avy_problems", {
      forecastId: avy.id,
    })) as AvalancheProblem[],
  };
};
