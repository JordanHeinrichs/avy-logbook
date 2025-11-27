<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import type { NewWeather } from "$lib/types/NewWeather";
  import WeatherForm from "$lib/components/WeatherForm.svelte";

  let { params, data } = $props();

  const isPlan = data.isPlan;

  // Initialize state
  let weather = $state<NewWeather>({
    tripId: data.tripId,
    accumulation: null,
    comment: null,
    observationTime: data.isPlan
      ? null
      : new Date().toLocaleTimeString([], {
          hour: "2-digit",
          minute: "2-digit",
          hour12: false,
        }),
    precipitation: null,
    solarRadiation: null,
    windDirection: null,
    windSpeed: null,
  });

  async function updateAndNext() {
    if (data.isPlan) {
      weather.observationTime = null;
    }

    await invoke("create_weather", {
      weather,
    });

    if (data.wizard) {
      goto(`/trips/${params.tripId}/plan`);
    } else {
      goto(`/trips/${params.tripId}`);
    }
  }
</script>

<Header
  title={data.isPlan ? "Weather Forecast" : "Weather Observation"}
  backHref={`/trips/${params.tripId}`}
/>

<main class="container mx-auto p-4 flex flex-col gap-6 overflow-y-auto">
  <WeatherForm bind:weather {isPlan} />
</main>

<Footer
  buttonText={data.wizard ? "Next Step" : "Save"}
  on:click={updateAndNext}
/>
