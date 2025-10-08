<script lang="ts">
  import "../style.css";
  import { goto } from "$app/navigation";
  import Footer from "$lib/components/Footer.svelte";

  let { data } = $props();

  async function createTrip() {
    console.info(`Navigating to create new log.`);
    await goto(`/trips/new`);
  }

  async function gotoTrip(id: number) {
    console.info(`Navigating to log ID ${id}.`);
    await goto(`/trips/${id}`);
  }
</script>

<main class="container">
  <div
    class="flex flex-auto flex-row items-center bg-base-200 sticky top-0 shadow-md"
  >
    <img src="/logo.png" alt="Logo" class="size-20 ml-2 w-20" />
    <h1 class="text-3xl font-bold grow text-center text-primary">
      Avy Logbook
    </h1>
    <div class="w-20"></div>
  </div>

  <div class="list bg-base-100 rounded-box shadow-md">
    {#if data.trips.length === 0}
      <div class="list-row">
        <p>Add entry to get started.</p>
      </div>
    {:else}
      {#each data.trips as trip}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="list-row shadow-sm flex flex-row justify-between cursor-pointer items-center hover:bg-primary-content"
          onclick={() => gotoTrip(trip.id)}
          aria-label="Go to log"
        >
          <div class="flex items-baseline gap-4">
            <div class="">{trip.name}</div>
            <div class="text-xs uppercase font-semibold opacity-60">
              {trip.tripDate}
            </div>
          </div>

          <!-- svelte-ignore a11y_consider_explicit_label -->
          <button class="btn btn-square btn-ghost">
            <svg
              class="size-[1.2em]"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              ><g
                stroke-linejoin="round"
                stroke-linecap="round"
                stroke-width="2"
                fill="none"
                stroke="currentColor"><path d="M6 3L20 12 6 21 6 3z"></path></g
              ></svg
            >
          </button>
        </div>
      {/each}
    {/if}
  </div>
</main>

<Footer buttonText="New Entry" on:click={createTrip} />
