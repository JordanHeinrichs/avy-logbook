<script lang="ts">
  import "cally";

  import { invoke } from "@tauri-apps/api/core";
  import type { Trip } from "$lib/types/Trip";
  import { goto } from "$app/navigation";
  import { format } from "date-fns";

  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";

  let name = $state("");
  let tripDate = $state(format(new Date(), "yyyy-MM-dd"));

  async function createTrip(_event: Event) {
    console.info(`Creating trip, name: ${name}, date: ${tripDate}`);
    const trip: Trip = await invoke("create_trip", { name, tripDate });
    goto(`/trips/${trip.id}/edit/avy`);
  }

  function handleDateChange(event: any) {
    console.info("Date changed:", event.target?.value);
    tripDate = event.target?.value;
  }
</script>

<Header title="Add Trip" backHref="/" />

<main class="p-4 flex flex-col items-center pt-4">
  <div class="w-full max-w-md space-y-4">
    <div>
      <h2 class="text-2xl font-bold">Name and Date</h2>
      <p class="text-base-content/70 mt-1">
        This is the first step. You'll add forecast and field observations next.
      </p>
    </div>

    <div class="form-control w-full">
      <label class="label pb-2" for="location-input">
        <span class="label-text">Location Name</span>
      </label>
      <!-- svelte-ignore a11y_autofocus -->
      <input
        id="location-input"
        type="text"
        name="location"
        autofocus
        placeholder="e.g. Crowfoot Glades, Ursus Trees"
        class="input input-bordered w-full validator"
        minlength="1"
        maxlength="40"
        bind:value={name}
        required
      />
    </div>

    <label class="label" for="date-input">
      <span class="label-text">Trip Date</span>
    </label>
    <!-- svelte-ignore event_directive_deprecated -->
    <calendar-date
      id="date-input"
      class="cally bg-base-100 border border-base-300 shadow-lg rounded-box"
      showOutsideDays
      value={tripDate}
      on:change={handleDateChange}
    >
      <svg
        aria-label="Previous"
        class="fill-current size-4"
        slot="previous"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"><path d="M15.75 19.5 8.25 12l7.5-7.5"></path></svg
      >
      <svg
        aria-label="Next"
        class="fill-current size-4"
        slot="next"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"><path d="m8.25 4.5 7.5 7.5-7.5 7.5"></path></svg
      >
      <calendar-month></calendar-month>
    </calendar-date>
  </div>
</main>

<Footer buttonText="Next" disabled={!name} on:click={createTrip} />
