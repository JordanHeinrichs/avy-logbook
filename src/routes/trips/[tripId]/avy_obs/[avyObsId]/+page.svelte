<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import AvyObsForm from "$lib/components/AvyObsForm.svelte";

  let { data } = $props();
  let avyObs = $state(data.avyObs);

  async function handleSave() {
    await invoke("edit_avy_observation", {
      avyObservation: avyObs,
    });

    await goto(`/trips/${avyObs.tripId}`);
  }
</script>

<Header title="Avalanche Observation" backHref={`/trips/${avyObs.tripId}`} />

<main class="container mx-auto p-4 pb-24">
  <AvyObsForm bind:observation={avyObs} />
</main>

<Footer buttonText="Save" on:click={handleSave} />
