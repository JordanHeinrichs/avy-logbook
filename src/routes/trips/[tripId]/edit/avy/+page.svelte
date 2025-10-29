<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { AvalancheForecast } from "$lib/types/AvalancheForecast.js";
  import { goto } from "$app/navigation";
  import type { DangerRating } from "$lib/types/DangerRating";
  import type { MacroTrend } from "$lib/types/MacroTrend";
  import type { Confidence } from "$lib/types/Confidence";

  let { params, data } = $props();

  const forecast = $state(data.avy);

  const DANGER_RATINGS: DangerRating[] = [
    "low",
    "moderate",
    "considerable",
    "high",
    "extreme",
    "unknown",
  ];
  const MACRO_TRENDS: MacroTrend[] = [
    "decreasing",
    "steady",
    "meltFreeze",
    "increasing",
    "unknown",
  ];
  const CONFIDENCE_LEVELS: Confidence[] = [
    "high",
    "moderate",
    "low",
    "unknown",
  ];

  async function updateAndNext(_event: Event) {
    const trip: AvalancheForecast = await invoke("edit_avy_forecast", {
      forecast,
    });
    // goto(`/trips/${trip.id}/edit/weather`);
  }

  function getDangerRatingClass(
    rating: DangerRating,
    selectedRating: DangerRating | null
  ): string {
    const base = "btn btn-sm sm:btn-md flex-1";
    const selectedClass = rating === selectedRating ? "" : "btn-outline";

    switch (rating) {
      case "low":
        return `${base} ${selectedClass} btn-success --color-warning`;
      case "moderate":
        return `${base} ${selectedClass} btn-warning`;
      case "considerable":
        return `${base} ${selectedClass} btn-accent`;
      case "high":
        return `${base} ${selectedClass} btn-error`;
      case "extreme":
        return `${base} ${selectedClass} `;
      default:
        return `${base} ${selectedClass} btn-ghost`;
    }
  }

  function getButtonClass<T>(option: T, selected: T | null): string {
    const base = "btn btn-sm sm:btn-md flex-1";
    return option === selected
      ? `${base} btn-primary`
      : `${base} btn-outline btn-primary`;
  }
</script>

<Header title="Avy Forecast" backHref={`/trips/${params.tripId}`} />

<main class="p-4 flex flex-col items-center pt-4">
  <div class="w-full max-w-md space-y-4">
    <div>
      <h2 class="text-2xl font-bold">Avalanche Forecast Information</h2>
      <p class="text-base-content/70 mt-1">
        Get this information from the correct avalanche forecast region.
      </p>
    </div>
  </div>

  <div class="w-full max-w-lg space-y-6">
    <div>
      <h2 class="text-2xl font-bold">Danger Ratings</h2>
      <p class="text-base-content/70 mt-1">
        As specified in the public forecast for your area.
      </p>
    </div>

    <section class="form-control w-full space-y-4">
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
              {rating}
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
              {rating}
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
              {rating}
            </button>
          {/each}
        </div>
      </div>
    </section>

    <div class="divider"></div>

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
            {trend === "meltFreeze" ? "Melt-Freeze" : trend}
          </button>
        {/each}
      </div>
    </section>

    <div class="divider"></div>

    <!-- Confidence Section -->
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
            {level}
          </button>
        {/each}
      </div>
    </section>

    <div class="divider"></div>

    <!-- Comments Section -->
    <section class="form-control w-full">
      <label class="label pb-2" for="textarea">
        <span class="label-text">Comments</span>
      </label>
      <textarea
        class="textarea textarea-bordered h-24"
        placeholder="e.g. Forecast notes, specific concerns..."
        bind:value={forecast.comment}
      ></textarea>
    </section>
  </div>
</main>

<Footer buttonText="Next" on:click={updateAndNext} />
