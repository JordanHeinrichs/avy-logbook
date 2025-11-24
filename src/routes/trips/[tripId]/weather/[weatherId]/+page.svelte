<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import WeatherForm from "$lib/components/WeatherForm.svelte";

  let { data } = $props();
  let weather = $state(data.weather);

  async function handleSave() {
    if (!weather) return;

    await invoke("edit_weather", {
      weather,
    });

    await goto(`/trips/${weather.tripId}`);
  }

  let isPlan = $derived(weather?.observationTime === null);
</script>

<Header
  title={isPlan ? "Weather Forecast" : "Weather Observation"}
  backHref={`/trips/${weather.tripId}`}
/>

<main class="container mx-auto p-4 pb-24">
  <WeatherForm bind:weather {isPlan} />
</main>

<Footer buttonText="Save" on:click={handleSave} />
