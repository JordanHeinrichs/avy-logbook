<script lang="ts">
  import { goto } from "$app/navigation";
  import Header from "$lib/components/Header.svelte";

  let { data } = $props();

  async function editForecast(_event: Event) {
    goto(`/trips/${data.trip.id}/edit/avy`);
  }
</script>

<Header title={data.trip.name} backHref="/" />

<main class="container">
  <div>
    <h2 class="text-3xl font-bold text-secondary p-1">{data.trip.tripDate}</h2>
  </div>

  <p class="text-3xl font-bold text-secondary p-1">
    {JSON.stringify(data.trip.forecast, null, 2)}
  </p>
  <button class="btn btn-secondary" onclick={editForecast}>Edit Forecast</button
  >
</main>
