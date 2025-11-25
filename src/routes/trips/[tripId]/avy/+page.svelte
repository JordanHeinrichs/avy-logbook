<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import AvyProblemEditor from "$lib/components/AvyProblemEditor.svelte";
  import type { DangerRating } from "$lib/types/DangerRating";
  import type { MacroTrend } from "$lib/types/MacroTrend";
  import type { Confidence } from "$lib/types/Confidence";

  let { params, data } = $props();

  const forecast = $state(data.avy);
  let problems = $state(data.problems);

  // --- Enums & Display Maps (camelCase) ---
  const DANGER_RATINGS: DangerRating[] = [
    "low",
    "moderate",
    "considerable",
    "high",
    "extreme",
    "unknown",
  ];
  const dangerRatingMap: Record<DangerRating, string> = {
    low: "Low",
    moderate: "Moderate",
    considerable: "Considerable",
    high: "High",
    extreme: "Extreme",
    unknown: "Unknown",
  };

  const MACRO_TRENDS: MacroTrend[] = [
    "decreasing",
    "steady",
    "meltFreeze",
    "increasing",
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
    const base = "btn btn-sm sm:btn-md flex-1";
    const selectedClass = rating === selectedRating ? "" : "btn-outline";

    switch (rating) {
      case "low":
        return `${base} ${selectedClass} btn-success`;
      case "moderate":
        return `${base} ${selectedClass} btn-warning`;
      case "considerable":
        return `${base} ${selectedClass} btn-error`;
      case "high":
        return `${base} ${selectedClass} btn-error`;
      case "extreme":
        return `${base} ${selectedClass} btn-neutral`;
      default:
        return `${base} ${selectedClass} btn-ghost`;
    }
  }

  function getButtonClass<T>(
    option: T,
    selected: T | null | undefined
  ): string {
    const base = "btn btn-sm sm:btn-md flex-1";
    return option === selected
      ? `${base} btn-primary`
      : `${base} btn-outline btn-primary`;
  }
</script>

<Header title="Avy Forecast" backHref={`/trips/${params.tripId}`} />

<main class="container mx-auto p-4 flex flex-col items-center pb-24">
  <div class="card w-full max-w-lg bg-base-200 shadow-xl">
    <div class="card-body">
      <h2 class="card-title text-2xl font-bold">Avalanche Forecast</h2>
      <p class="text-base-content/70 -mt-2 mb-4">
        Get this information from the correct avalanche forecast region.
      </p>

      <section class="form-control w-full space-y-4">
        <h3 class="text-xl font-bold">Danger Ratings</h3>

        <div>
          <label class="label" for="div">
            <span class="label-text text-lg">Alpine</span>
          </label>
          <div class="btn-group w-full">
            {#each DANGER_RATINGS as rating}
              <button
                class={getDangerRatingClass(rating, forecast.forecastAlp)}
                onclick={() => (forecast.forecastAlp = rating)}
              >
                {dangerRatingMap[rating]}
              </button>
            {/each}
          </div>
        </div>
        <div>
          <label class="label" for="div">
            <span class="label-text text-lg">Treeline</span>
          </label>
          <div class="btn-group w-full">
            {#each DANGER_RATINGS as rating}
              <button
                class={getDangerRatingClass(rating, forecast.forecastTl)}
                onclick={() => (forecast.forecastTl = rating)}
              >
                {dangerRatingMap[rating]}
              </button>
            {/each}
          </div>
        </div>
        <div>
          <label class="label" for="div">
            <span class="label-text text-lg">Below Treeline</span>
          </label>
          <div class="btn-group w-full">
            {#each DANGER_RATINGS as rating}
              <button
                class={getDangerRatingClass(rating, forecast.forecastBtl)}
                onclick={() => (forecast.forecastBtl = rating)}
              >
                {dangerRatingMap[rating]}
              </button>
            {/each}
          </div>
        </div>
      </section>

      <div class="divider my-4"></div>

      <section class="form-control w-full">
        <label class="label" for="div">
          <span class="label-text text-xl font-bold">Macro Trend</span>
        </label>
        <div class="btn-group w-full">
          {#each MACRO_TRENDS as trend}
            <button
              class={getButtonClass(trend, forecast.macroTrends)}
              onclick={() => (forecast.macroTrends = trend)}
            >
              {macroTrendMap[trend]}
            </button>
          {/each}
        </div>
      </section>

      <div class="divider my-4"></div>

      <section class="form-control w-full">
        <label class="label" for="div">
          <span class="label-text text-xl font-bold">Confidence</span>
        </label>
        <div class="btn-group w-full">
          {#each CONFIDENCE_LEVELS as level}
            <button
              class={getButtonClass(level, forecast.confidence)}
              onclick={() => (forecast.confidence = level)}
            >
              {confidenceMap[level]}
            </button>
          {/each}
        </div>
      </section>

      <div class="divider my-4"></div>

      <section class="form-control w-full">
        <h3 class="text-xl font-bold mb-4">Avalanche Problems</h3>
        <AvyProblemEditor forecastId={forecast.id} bind:problems />
      </section>

      <div class="divider my-4"></div>

      <section class="form-control w-full">
        <label class="label pb-2" for="textarea">
          <span class="label-text text-xl font-bold">Comments</span>
        </label>
        <textarea
          class="textarea textarea-bordered h-24"
          placeholder="e.g. Forecast notes, specific concerns..."
          bind:value={forecast.comment}
        ></textarea>
      </section>
    </div>
  </div>
</main>

<Footer buttonText={data.wizard ? "Next" : "Back"} on:click={updateAndNext} />
