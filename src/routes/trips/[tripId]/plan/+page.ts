import { invoke } from "@tauri-apps/api/core";
import type { TripPlan } from "$lib/types/TripPlan";

export const load = async ({ params }) => {
  try {
    const plan: TripPlan = await invoke("fetch_plan", {
      tripId: Number(params.tripId),
    });
    return {
      plan,
    };
  } catch (err) {
    return {
      plan: {
        tripId: Number(params.tripId),
        areasToAvoid: null,
        decisionPointsComment: null,
        decisionPointsConsidered: false,
        planLeftWithSomeone: false,
      },
    };
  }
};
