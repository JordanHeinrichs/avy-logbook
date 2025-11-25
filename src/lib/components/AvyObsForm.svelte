<script lang="ts">
  import type { NewAvyObservation } from "$lib/types/NewAvyObservation";
  import type { AvalancheSize } from "$lib/types/AvalancheSize";
  import type { AvalancheTrigger } from "$lib/types/AvalancheTrigger";
  import type { InstabilityObservation } from "$lib/types/InstabilityObservation";
  import type { ProblemType } from "$lib/types/ProblemType";
  import type { TestResult } from "$lib/types/TestResult";

  let { observation = $bindable() } = $props<{
    observation: NewAvyObservation;
  }>();

  const sizeOptions: { value: AvalancheSize; label: string }[] = [
    { value: "none", label: "None" },
    { value: "lessThan1", label: "Size < 1" },
    { value: "from15To2", label: "Size 1.5-2" },
    { value: "moreThan25", label: "Size > 2.5" },
    { value: "unknown", label: "?" },
  ];

  const triggerOptions: { value: AvalancheTrigger; label: string }[] = [
    { value: "natural", label: "Natural" },
    { value: "light", label: "Light (Skier)" },
    { value: "moderate", label: "Mod" },
    { value: "heavy", label: "Heavy (Sled)" },
    { value: "unknown", label: "?" },
  ];

  // 2-Column layout for Problem Types as names are longer
  const problemOptions: { value: ProblemType; label: string }[] = [
    { value: "looseDry", label: "Loose Dry (LD)" },
    { value: "looseWet", label: "Loose Wet (LW)" },
    { value: "stormSlabs", label: "Storm Slab (SS)" },
    { value: "windSlabs", label: "Wind Slab (WD)" },
    { value: "persistentSlabs", label: "Persistent (PS)" },
    { value: "deepPersistentSlabs", label: "Deep Pers. (DP)" },
    { value: "wetSlab", label: "Wet Slab (WS)" },
    { value: "cornices", label: "Cornice (CO)" },
  ];

  const instabilityOptions: { value: InstabilityObservation; label: string }[] =
    [
      { value: "none", label: "None" },
      { value: "whumpf", label: "Whumpf" },
      { value: "crack", label: "Crack" },
      { value: "drum", label: "Drum" },
    ];

  const testResultOptions: { value: TestResult; label: string }[] = [
    { value: "none", label: "No Result" },
    { value: "failure", label: "Failure" },
    { value: "popDrop", label: "Pop / Drop" },
  ];
</script>

<div class="flex flex-col gap-6">
  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <label class="form-control w-full">
        <div class="label">
          <span class="label-text font-bold">Observation Time</span>
        </div>
        <input
          type="time"
          class="input input-bordered w-full text-lg"
          bind:value={observation.observationTime}
        />
      </label>
    </div>
  </div>

  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <h2 class="card-title text-lg mb-4">Avalanche Activity</h2>

      <h3 class="font-bold text-sm mb-1">Size</h3>
      <div class="grid grid-cols-2 gap-2 mb-4">
        {#each sizeOptions as opt}
          <button
            class="btn {observation.avyActivitySize === opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}
              {opt.value === 'unknown' ? 'col-span-2' : ''}"
            onclick={() => (observation.avyActivitySize = opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>

      <h3 class="font-bold text-sm mb-1">Trigger</h3>
      <div class="grid grid-cols-2 gap-2 mb-4">
        {#each triggerOptions as opt}
          <button
            class="btn {observation.avyActivityTrigger === opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}
              {opt.value === 'unknown' ? 'col-span-2' : ''}"
            onclick={() => (observation.avyActivityTrigger = opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>

      <h3 class="font-bold text-sm mb-1">Characteristic / Type</h3>
      <div class="grid grid-cols-2 gap-2">
        {#each problemOptions as opt}
          <button
            class="btn h-auto min-h-12 leading-tight {observation.avyActivityCharacteristic ===
            opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}"
            onclick={() => (observation.avyActivityCharacteristic = opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>
    </div>
  </div>

  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <h2 class="card-title text-lg mb-4">Signs of Instability</h2>

      <h3 class="font-bold text-sm mb-1">See / Feel</h3>
      <div class="grid grid-cols-2 gap-2 mb-4">
        {#each instabilityOptions as opt}
          <button
            class="btn {observation.instabilitySeeFeel === opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}"
            onclick={() => (observation.instabilitySeeFeel = opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>

      <h3 class="font-bold text-sm mb-1">Compression Test (CT)</h3>
      <div class="grid grid-cols-3 gap-2 mb-4">
        {#each testResultOptions as opt}
          <button
            class="btn px-1 {observation.instabilityCt === opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}"
            onclick={() => (observation.instabilityCt = opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>

      <h3 class="font-bold text-sm mb-1">Extended Column (ECT)</h3>
      <div class="grid grid-cols-3 gap-2">
        {#each testResultOptions as opt}
          <button
            class="btn px-1 {observation.instabilityEct === opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}"
            onclick={() => (observation.instabilityEct = opt.value)}
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
          <span class="label-text font-bold">Comments</span>
        </div>
        <textarea
          class="textarea textarea-bordered h-24 text-base"
          placeholder="Specific details on avalanche path, aspect, or test depth..."
          bind:value={observation.comment}
        ></textarea>
      </label>
    </div>
  </div>
</div>
