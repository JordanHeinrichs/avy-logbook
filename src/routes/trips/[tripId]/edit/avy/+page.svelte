<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";

  let { params } = $props();
</script>

<Header title="Avy Forecast" backHref={`/trips/${params.tripId}`} />

<main class="p-4 flex flex-col items-center pt-4">
  <div class="w-full max-w-md space-y-4">
    <div>
      <h2 class="text-2xl font-bold">Avalanche Forecast Information</h2>
      <p class="text-base-content/70 mt-1">
        Get this information from the correct avalanche forecast region.
      </p>
    </div>
  </div>
</main>

<Footer buttonText="Next" />
