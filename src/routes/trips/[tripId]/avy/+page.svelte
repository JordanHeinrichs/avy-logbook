<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import AvyProblemEditor from "$lib/components/AvyProblemEditor.svelte";
  import type { DangerRating } from "$lib/types/DangerRating";
  import type { MacroTrend } from "$lib/types/MacroTrend";
  import type { Confidence } from "$lib/types/Confidence";
  import type { AvalancheForecast } from "$lib/types/AvalancheForecast";

  let { params, data } = $props();

  const forecast = $state(data.avy);
  let problems = $state(data.problems);

  const dangerRating: [DangerRating, string][] = [
    ["low", "Low"],
    ["moderate", "Moderate"],
    ["considerable", "Considerable"],
    ["high", "High"],
    ["extreme", "Extreme"],
    ["unknown", "Unknown"],
  ];

  const dangerRatingElevation: [
    keyof Pick<AvalancheForecast, "forecastAlp" | "forecastTl" | "forecastBtl">,
    string,
  ][] = [
    ["forecastAlp", "Alpine"],
    ["forecastTl", "Treeline"],
    ["forecastBtl", "Below Treeline"],
  ];

  const MACRO_TRENDS: MacroTrend[] = [
    "decreasing",
    "steady",
    "increasing",
    "meltFreeze",
    "unknown",
  ];
  const macroTrendMap: Record<MacroTrend, string> = {
    decreasing: "Decreasing",
    steady: "Steady",
    meltFreeze: "Melt-Freeze",
    increasing: "Increasing",
    unknown: "Unknown",
  };

  const CONFIDENCE_LEVELS: Confidence[] = [
    "high",
    "moderate",
    "low",
    "unknown",
  ];
  const confidenceMap: Record<Confidence, string> = {
    high: "High",
    moderate: "Moderate",
    low: "Low",
    unknown: "Unknown",
  };

  async function updateAndNext(_event: Event) {
    await invoke("edit_avy_forecast", {
      forecast,
    });

    if (data.wizard) {
      await goto(`/trips/${params.tripId}/weather/new?plan=true&wizard=true`);
    } else {
      await goto(`/trips/${params.tripId}`);
    }
  }

  function getDangerRatingClass(
    rating: DangerRating,
    selectedRating: DangerRating | null | undefined
  ): string {
    const base = "btn btn-sm flex-0";
    const selectedClass = rating === selectedRating ? "" : "btn-outline";

    switch (rating) {
      case "low":
        return `${base} ${selectedClass} btn-low`;
      case "moderate":
        return `${base} ${selectedClass} btn-moderate`;
      case "considerable":
        return `${base} ${selectedClass} btn-considerable`;
      case "high":
        return `${base} ${selectedClass} btn-high`;
      case "extreme":
        return `${base} ${selectedClass} btn-extreme`;
      default:
        return `${base} ${selectedClass} btn-unknown`;
    }
  }

  function getButtonClass<T>(
    option: T,
    selected: T | null | undefined
  ): string {
    const base = "btn btn-sm flex-0 whitespace-nowrap";
    return option === selected
      ? `${base} btn-primary`
      : `${base} btn-outline btn-primary`;
  }
</script>

<Header title="Avalanche Forecast" backHref={`/trips/${params.tripId}`} />

<main class="container mx-auto p-4 flex flex-col overflow-y-auto">
  <div role="alert" class="alert">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      class="stroke-info h-6 w-6 shrink-0"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
      ></path>
    </svg>
    <span>Get this information from the correct avalanche forecast region.</span
    >
  </div>

  <fieldset
    class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4"
  >
    <legend class="fieldset-legend">Danger Ratings</legend>
    {#each dangerRatingElevation as [key, label]}
      <div>
        <label class="label" for="div">
          <span class="label-text text-lg">{label}</span>
        </label>
        <div class="flex w-full flex-wrap gap-1">
          {#each dangerRating as [rating, ratingLabel]}
            <button
              class={getDangerRatingClass(rating, forecast[key])}
              onclick={() => (forecast[key] = rating)}
            >
              {ratingLabel}
            </button>
          {/each}
        </div>
      </div>
    {/each}
  </fieldset>

  <fieldset
    class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4"
  >
    <legend class="fieldset-legend">Macro Trend</legend>
    <div class="flex w-full flex-wrap gap-1">
      {#each MACRO_TRENDS as trend}
        <button
          class={getButtonClass(trend, forecast.macroTrends)}
          onclick={() => (forecast.macroTrends = trend)}
        >
          {macroTrendMap[trend]}
        </button>
      {/each}
    </div>
  </fieldset>

  <fieldset
    class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4"
  >
    <legend class="fieldset-legend">Confidence</legend>
    <div class="flex w-full flex-wrap gap-1">
      {#each CONFIDENCE_LEVELS as level}
        <button
          class={getButtonClass(level, forecast.confidence)}
          onclick={() => (forecast.confidence = level)}
        >
          {confidenceMap[level]}
        </button>
      {/each}
    </div>
  </fieldset>

  <fieldset
    class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4"
  >
    <legend class="fieldset-legend">Avalanche Problems</legend>
    <AvyProblemEditor forecastId={forecast.id} bind:problems />
  </fieldset>

  <fieldset
    class="fieldset bg-base-200 border-base-300 rounded-box w-full border p-4"
  >
    <legend class="fieldset-legend">Comments</legend>
    <textarea
      class="textarea w-full"
      placeholder="e.g. Forecast notes, specific concerns..."
      bind:value={forecast.comment}
    ></textarea>
  </fieldset>
</main>

<Footer buttonText={data.wizard ? "Next" : "Save"} on:click={updateAndNext} />
