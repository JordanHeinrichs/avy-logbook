<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import AvyObsForm from "$lib/components/AvyObsForm.svelte";
  import type { NewAvyObservation } from "$lib/types/NewAvyObservation.js";

  let { params, data } = $props();

  let avyObs = $state<NewAvyObservation>({
    tripId: data.tripId,
    observationTime: new Date().toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    }),
    comment: null,
    avyActivityCharacteristic: null,
    avyActivitySize: null,
    avyActivityTrigger: null,
    instabilityCt: null,
    instabilityEct: null,
    instabilitySeeFeel: null,
  });

  async function save() {
    await invoke("create_avy_observation", {
      avyObservation: avyObs,
    });
    goto(`/trips/${params.tripId}`);
  }
</script>

<Header title="Avalanche Observation" backHref={`/trips/${params.tripId}`} />

<main class="container mx-auto p-4 flex flex-col gap-6 overflow-y-auto">
  <AvyObsForm bind:observation={avyObs} />
</main>

<Footer buttonText="Save" on:click={save} />
