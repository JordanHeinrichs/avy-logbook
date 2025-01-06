<script lang="ts">
  import "../style.css";
  import { goto } from "$app/navigation";

  let { data } = $props();

  async function createLog() {
    await goto(`/logs/`);
  }

  async function gotoLog(id: number) {
    await goto(`/logs/${id}`);
  }
</script>

<main class="container">
  <div>
    <h1 class="text-3xl font-bold text-center p-1">Avi Logbook</h1>
  </div>

  <button class="btn btn-primary" onclick={() => createLog()}>New Entry</button>

  <div class="list">
    {#if data.logs.length === 0}
      <div class="list-row">
        <p>Add entry to get started</p>
      </div>
    {:else}
      {#each data.logs as log}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="list-row"
          onclick={() => gotoLog(log.id)}
          aria-label="Go to log"
        >
          <div class="font-semibold">{log.name}</div>
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
