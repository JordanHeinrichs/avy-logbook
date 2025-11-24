import { invoke } from "@tauri-apps/api/core";
import type { Weather } from "$lib/types/Weather.js";

export const load = async ({ params }) => {
  return {
    weather: (await invoke("fetch_weather", {
      id: Number(params.weatherId),
    })) as Weather,
  };
};
