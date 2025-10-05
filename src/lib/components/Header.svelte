<script lang="ts">
  import { goto } from "$app/navigation";

  /**
   * The title to display in the center of the header.
   * @type {string}
   */
  export let title: string;

  /**
   * The URL to navigate to when the back button is clicked.
   * If not provided, the back button will not be displayed.
   * @type {string | null}
   */
  export let backHref: string | null = null;
</script>

<header
  class="flex flex-row items-center bg-base-200 sticky top-0 z-10 shadow-md"
>
  <div class="w-16">
    {#if backHref}
      <button
        class="btn btn-ghost btn-square ml-2"
        on:click={() => goto(backHref)}
        aria-label="Go back"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="2"
          stroke="currentColor"
          class="w-6 h-6"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15.75 19.5 8.25 12l7.5-7.5"
          />
        </svg>
      </button>
    {/if}
  </div>

  <div class="flex-1 text-center">
    <h1 class="text-2xl font-bold text-primary">{title}</h1>
  </div>

  <div class="w-16 mr-2"></div>
</header>
