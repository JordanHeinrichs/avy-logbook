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
  <div class="flex flex-auto flex-row items-center bg-base-200 sticky top-0">
    <img src="/logo.png" alt="Logo" class="size-20 ml-2" />
    <h1 class="text-3xl font-bold grow text-primary">Avy Logbook</h1>
    <button class="btn btn-primary mr-4" onclick={() => createLog()}
      >New Entry</button
    >
  </div>

  <div class="list bg-base-100 rounded-box shadow-md">
    {#if data.logs.length === 0}
      <div class="list-row">
        <p>Add entry to get started.</p>
      </div>
    {:else}
      {#each data.logs as log}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="list-row flex flex-row justify-items-end cursor-pointer items-center hover:bg-primary-content"
          onclick={() => gotoLog(log.id)}
          aria-label="Go to log"
        >
          <div class="">{log.name}</div>
          <div class="text-xs uppercase font-semibold opacity-60">
            {log.date}
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
