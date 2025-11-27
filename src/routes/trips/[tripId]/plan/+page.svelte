<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import type { TripPlan } from "$lib/types/TripPlan";
  import type { AreaToAvoid } from "$lib/types/AreaToAvoid";

  let { params, data } = $props();

  const plan = $state<TripPlan>(data.plan);

  const areaOptions: { value: AreaToAvoid; label: string }[] = [
    { value: "terrainTraps", label: "Terrain Traps" },
    { value: "startZones", label: "Start Zones" },
    { value: "avyPaths", label: "Avy Paths" },
    { value: "runoutZones", label: "Runout Zones" },
    { value: "slope30To35", label: "30-35° Slope" },
    { value: "slopeGreaterThan35", label: "> 35° Slope" },
    { value: "convexUnsupported", label: "Convex / Unsupported" },
    { value: "leeLoaded", label: "Lee / Loaded" },
    { value: "sunny", label: "Sunny Aspects" },
    { value: "overheadHazard", label: "Overhead Hazard" },
    { value: "slopeSizeLarge", label: "Large Slopes" },
    { value: "slopeSizeMedium", label: "Medium Slopes" },
    { value: "slopeSizeSmall", label: "Small Slopes" },
  ];

  function toggleArea(area: AreaToAvoid) {
    if (!plan.areasToAvoid) {
      plan.areasToAvoid = [];
    }

    if (plan.areasToAvoid.includes(area)) {
      plan.areasToAvoid = plan.areasToAvoid.filter((a) => a !== area);
    } else {
      plan.areasToAvoid.push(area);
    }
  }

  function isSelected(area: AreaToAvoid): boolean {
    return plan.areasToAvoid?.includes(area) ?? false;
  }

  async function save(_event: Event) {
    await invoke("upsert_plan", {
      plan,
    });
    await goto(`/trips/${params.tripId}`);
  }
</script>

<Header title="Trip Plan" backHref={`/trips/${params.tripId}`} />

<main class="container mx-auto p-4 flex flex-col gap-6 overflow-y-auto">
  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <h3 class="font-bold text-lg mb-2">Safety Checks</h3>

      <div class="form-control">
        <label class="label cursor-pointer justify-start gap-4">
          <input
            type="checkbox"
            class="toggle toggle-success toggle-lg"
            bind:checked={plan.planLeftWithSomeone}
          />
          <span class="label-text text-base font-medium"
            >Trip plan left with someone?</span
          >
        </label>
      </div>

      <div class="form-control mt-2">
        <label class="label cursor-pointer justify-start gap-4">
          <input
            type="checkbox"
            class="toggle toggle-success toggle-lg"
            bind:checked={plan.decisionPointsConsidered}
          />
          <span class="label-text text-base font-medium"
            >Decision points considered?</span
          >
        </label>
      </div>
    </div>
  </div>

  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <h3 class="font-bold text-lg mb-2">Areas to Avoid</h3>
      <p class="text-sm text-base-content/70 mb-4">
        Select all that apply based on the forecast.
      </p>

      <div class="grid grid-cols-2 gap-3">
        {#each areaOptions as opt}
          <button
            class="btn h-auto min-h-12 {isSelected(opt.value)
              ? 'btn-error text-white'
              : 'btn-outline bg-base-100'}"
            onclick={() => toggleArea(opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>
    </div>
  </div>

  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <label class="form-control">
        <div class="label">
          <span class="label-text font-bold text-lg"
            >Decision Points / Strategy</span
          >
        </div>
        <textarea
          class="textarea textarea-bordered h-32 text-base"
          placeholder="Notes on specific slopes to avoid, turn-around times, or alternative routes..."
          bind:value={plan.decisionPointsComment}
        ></textarea>
      </label>
    </div>
  </div>
</main>

<Footer buttonText="Save Plan" on:click={save} />
