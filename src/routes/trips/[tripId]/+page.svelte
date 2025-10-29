<script lang="ts">
  import { goto } from "$app/navigation";
  import Header from "$lib/components/Header.svelte";
  import type { Accumulation } from "$lib/types/Accumulation";
  import type { AreaToAvoid } from "$lib/types/AreaToAvoid";
  import type { AvalancheSize } from "$lib/types/AvalancheSize";
  import type { AvalancheTrigger } from "$lib/types/AvalancheTrigger";
  import type { FullTripDetails } from "$lib/types/FullTripDetails";
  import type { InstabilityObservation } from "$lib/types/InstabilityObservation";
  import type { Precipitation } from "$lib/types/Precipitation";
  import type { ProblemType } from "$lib/types/ProblemType";
  import type { TestResult } from "$lib/types/TestResult";
  import type { WindSpeed } from "$lib/types/WindSpeed";

  let { data }: { data: { trip: FullTripDetails } } = $props();

  // Use the data prop directly
  const trip = data.trip;

  // --- Navigation Functions ---
  async function editForecast() {
    goto(`/trips/${trip.id}/edit/avy`);
  }
  async function editTripInfo() {
    goto(`/trips/${trip.id}/edit/prep`);
  }
  async function editWeather(weatherId: number) {
    // You'll need to decide if "forecast" (null time) has a special ID or route
    goto(`/trips/${trip.id}/edit/weather/${weatherId}`);
  }
  async function addWeatherObs() {
    goto(`/trips/${trip.id}/obs/weather/new`);
  }
  async function editAvyObs(obsId: number) {
    goto(`/trips/${trip.id}/obs/avy/${obsId}`);
  }
  async function addAvyObs() {
    goto(`/trips/${trip.id}/obs/avy/new`);
  }

  // --- Display Maps (for human-readable enums) ---
  const ratingColors: Record<string, string> = {
    Low: "badge-success",
    Moderate: "badge-warning",
    Considerable: "badge-error",
    High: "bg-red-700 text-white font-bold",
    Extreme: "bg-black text-white font-bold",
    Unknown: "badge-ghost",
  };

  const problemTypeMap: Record<ProblemType | string, string> = {
    LooseDry: "Loose Dry (LD)",
    LooseWet: "Loose Wet (LW)",
    WetSlab: "Wet Slab (WT)",
    Cornices: "Cornices (CO)",
    WindSlabs: "Wind Slabs (WD)",
    StormSlabs: "Storm Slabs (SS)",
    PersistentSlabs: "Persistent Slabs (PS)",
    DeepPersistentSlabs: "Deep Persistent Slabs (DP)",
  };

  const areaToAvoidMap: Record<AreaToAvoid | string, string> = {
    TerrainTraps: "Terrain Traps",
    StartZones: "Start zones",
    AvyPaths: "Avy Paths",
    RunoutZones: "Runout Zones",
    Slope30To35: "30-35° slope",
    SlopeGreaterThan35: "> 35° slope",
    ConvexUnsupported: "Convex / Unsupported",
    LeeLoaded: "Lee / Loaded",
    Sunny: "Sunny",
    SlopeSizeLarge: "Large Slope/Avy",
    SlopeSizeMedium: "Medium Slope/Avy",
    SlopeSizeSmall: "Small Slope/Avy",
    OverheadHazard: "Overhead Hazard",
  };

  const precipitationMap: Record<Precipitation | string, string> = {
    Nothing: "Nothing",
    S1: "S-1 (Light snowfall <= 1 cm/hr)",
    RL: "Light Rain (RL)",
    S2: "S2 (Moderate snowfall 2 cm/hr)",
    R: "Rain (R)",
    S3Plus: "S3+ (Heavy snowfall >= 3 cm/hr)",
    Unknown: "Unknown",
  };

  const accumulationMap: Record<Accumulation | string, string> = {
    Zero: "0 cm",
    LessThan20: "< 20 cm",
    From20To40: "20-40 cm",
    MoreThan40: "> 40 cm",
    Unknown: "Unknown",
  };

  const windSpeedMap: Record<WindSpeed | string, string> = {
    Calm: "Calm",
    Light: "Light (1-25 km/h)",
    Moderate: "Moderate (26-40 km/h)",
    Strong: "Strong (41-60 km/h)",
    Extreme: "Extreme (>60 km/h)",
    Unknown: "Unknown",
  };

  const avySizeMap: Record<AvalancheSize | string, string> = {
    None: "None",
    LessThan1: "<= 1",
    From1_5To2: "1.5-2",
    MoreThan2_5: ">= 2.5",
    Unknown: "Unknown",
  };

  const avyTriggerMap: Record<AvalancheTrigger | string, string> = {
    Heavy: "Heavy",
    Moderate: "Moderate",
    Light: "Light",
    Natural: "Natural",
    Unknown: "Unknown",
  };

  const instabilityObsMap: Record<InstabilityObservation | string, string> = {
    None: "None",
    Drum: "Drum",
    Crack: "Crack",
    Whumpf: "Whumpf",
  };

  const testResultMap: Record<TestResult | string, string> = {
    None: "None",
    Failure: "Failure",
    PopDrop: "Pop/Drop",
  };

  // --- Helper Functions ---
  function getRatingClass(rating: string | undefined | null) {
    const r = rating || "Unknown";
    return ratingColors[r] || "badge-ghost";
  }

  function getReadable(
    map: Record<string, string>,
    key: string | undefined | null
  ) {
    if (!key) return "None";
    return map[key] || key;
  }

  // --- Computed (Derived) Values ---
  // Use Svelte 5 $derived rune to compute problem lists
  const alpProblems = $derived(
    trip.problems
      .filter((p) => p.elevation === "ALP")
      .map((p) => problemTypeMap[p.problemType] || p.problemType)
  );
  const tlProblems = $derived(
    trip.problems
      .filter((p) => p.elevation === "TL")
      .map((p) => problemTypeMap[p.problemType] || p.problemType)
  );
  const btlProblems = $derived(
    trip.problems
      .filter((p) => p.elevation === "BTL")
      .map((p) => problemTypeMap[p.problemType] || p.problemType)
  );

  // Split weather observations into forecast (null time) and field (has time)
  const weatherForecast = $derived(
    trip.weatherObservations.find((w) => w.observationTime == null)
  );
  const fieldWeatherObs = $derived(
    trip.weatherObservations.filter((w) => w.observationTime != null)
  );

  // --- Icons (embedded as SVG strings for simplicity) ---
  const PencilIcon = `
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
      <path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" />
    </svg>
  `;
  const PlusIcon = `
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
    </svg>
  `;
</script>

<Header title={trip.name} backHref="/" />

<main class="container mx-auto p-4 flex flex-col gap-6 pb-20">
  <div>
    <h2 class="text-3xl font-bold text-secondary">{trip.tripDate}</h2>
  </div>

  {#if trip.forecast}
    <div class="card bg-base-200 shadow-xl">
      <div class="card-body">
        <div class="card-title flex justify-between items-center">
          <h2 class="text-xl font-bold">Avalanche Forecast</h2>
          <button
            class="btn btn-ghost btn-sm btn-circle"
            title="Edit Forecast"
            onclick={editForecast}
          >
            {@html PencilIcon}
          </button>
        </div>

        <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
          <div class="stat">
            <div class="stat-title">Alpine</div>
            <div
              class={`stat-value text-2xl badge badge-lg ${getRatingClass(trip.forecast.forecastAlp)}`}
            >
              {trip.forecast.forecastAlp ?? "Unknown"}
            </div>
          </div>
          <div class="stat">
            <div class="stat-title">Tree Line</div>
            <div
              class={`stat-value text-2xl badge badge-lg ${getRatingClass(trip.forecast.forecastTl)}`}
            >
              {trip.forecast.forecastTl ?? "Unknown"}
            </div>
          </div>
          <div class="stat">
            <div class="stat-title">Below Tree Line</div>
            <div
              class={`stat-value text-2xl badge badge-lg ${getRatingClass(trip.forecast.forecastBtl)}`}
            >
              {trip.forecast.forecastBtl ?? "Unknown"}
            </div>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
          <div>
            <strong>Confidence:</strong>
            <span class="badge badge-outline ml-2"
              >{trip.forecast.confidence ?? "Unknown"}</span
            >
          </div>
          <div>
            <strong>Macro Trend:</strong>
            <span class="badge badge-outline ml-2"
              >{trip.forecast.macroTrends ?? "Unknown"}</span
            >
          </div>
        </div>

        <h3 class="font-bold mt-4">Problems:</h3>
        <div class="overflow-x-auto">
          <table class="table table-zebra table-sm">
            <thead>
              <tr class="text-base-content">
                <th>Alpine</th>
                <th>Tree Line</th>
                <th>BTL</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>{alpProblems.join(", ") || "None"}</td>
                <td>{tlProblems.join(", ") || "None"}</td>
                <td>{btlProblems.join(", ") || "None"}</td>
              </tr>
            </tbody>
          </table>
        </div>

        {#if trip.forecast.comment}
          <div class="mt-4">
            <h4 class="font-bold">Comment:</h4>
            <p
              class="text-base-content/80 p-3 bg-base-100 rounded-lg whitespace-pre-wrap"
            >
              {trip.forecast.comment}
            </p>
          </div>
        {/if}
      </div>
    </div>
  {:else}
    <div class="card bg-base-200 shadow-xl">
      <div class="card-body items-center text-center">
        <h2 class="card-title">No Forecast Found</h2>
        <p>Add avalanche forecast information for this trip.</p>
        <div class="card-actions">
          <button class="btn btn-primary" onclick={editForecast}
            >Add Forecast</button
          >
        </div>
      </div>
    </div>
  {/if}

  {#if trip.planning}
    <div class="card bg-base-200 shadow-xl">
      <div class="card-body">
        <div class="card-title flex justify-between items-center">
          <h2 class="text-xl font-bold">Trip Prep</h2>
          <button
            class="btn btn-ghost btn-sm btn-circle"
            title="Edit Trip Prep"
            onclick={editTripInfo}
          >
            {@html PencilIcon}
          </button>
        </div>

        <div>
          <h4 class="font-bold">Areas to Avoid:</h4>
          <div class="flex flex-wrap gap-2 mt-2">
            {#if (trip.planning.areasToAvoid ?? []).length > 0}
              {#each trip.planning.areasToAvoid ?? [] as area}
                <span class="badge badge-lg badge-outline"
                  >{getReadable(areaToAvoidMap, area)}</span
                >
              {/each}
            {:else}
              <span class="italic text-base-content/60">None specified.</span>
            {/if}
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4 mt-4">
          <div>
            <strong>Trip plan left?</strong>
            {#if trip.planning.planLeftWithSomeone}
              <span class="badge badge-success ml-2">Yes</span>
            {:else}
              <span class="badge badge-error ml-2">No</span>
            {/if}
          </div>
          <div>
            <strong>Decision points?</strong>
            {#if trip.planning.decisionPointsConsidered}
              <span class="badge badge-success ml-2">Yes</span>
            {:else}
              <span class="badge badge-error ml-2">No</span>
            {/if}
          </div>
        </div>

        {#if trip.planning.decisionPointsComment}
          <div class="mt-4">
            <h4 class="font-bold">Decision Points Comment:</h4>
            <p
              class="text-base-content/80 p-3 bg-base-100 rounded-lg whitespace-pre-wrap"
            >
              {trip.planning.decisionPointsComment}
            </p>
          </div>
        {/if}
      </div>
    </div>
  {:else}
    <div class="card bg-base-200 shadow-xl">
      <div class="card-body items-center text-center">
        <h2 class="card-title">No Trip Prep Found</h2>
        <p>Add trip preparation details.</p>
        <div class="card-actions">
          <button class="btn btn-primary" onclick={editTripInfo}
            >Add Prep Details</button
          >
        </div>
      </div>
    </div>
  {/if}

  <div class="card bg-base-200 shadow-xl">
    <div class="card-body">
      <div class="card-title flex justify-between items-center">
        <h2 class="text-xl font-bold">Weather</h2>
        <button class="btn btn-primary btn-sm" onclick={addWeatherObs}>
          {@html PlusIcon} Add Obs
        </button>
      </div>

      {#if weatherForecast}
        <div class="relative p-4 bg-base-100 rounded-lg mt-2">
          <h3 class="text-lg font-semibold mb-2">Forecast</h3>
          <button
            class="btn btn-xs btn-outline btn-secondary absolute top-4 right-4"
            title="Edit Weather Forecast"
            onclick={() => editWeather(weatherForecast.id)}
          >
            Edit
          </button>
          <div class="grid grid-cols-2 lg:grid-cols-3 gap-x-4 gap-y-2">
            <div>
              <strong>Precip:</strong>
              {getReadable(precipitationMap, weatherForecast.precipitation)}
            </div>
            <div>
              <strong>Accum:</strong>
              {getReadable(accumulationMap, weatherForecast.accumulation)}
            </div>
            <div>
              <strong>Wind:</strong>
              {getReadable(windSpeedMap, weatherForecast.windSpeed)} ({weatherForecast.windDirection ??
                "?"})
            </div>
            <div>
              <strong>Solar:</strong>
              {weatherForecast.solarRadiation ?? "Unknown"}
            </div>
          </div>
        </div>
      {:else}
        <div
          class="p-4 bg-base-100 rounded-lg mt-2 text-center italic text-base-content/60"
        >
          No weather forecast added.
        </div>
      {/if}

      <h3 class="text-lg font-semibold mt-4 mb-2">Field Observations</h3>
      <div class="flex flex-col gap-2">
        {#if fieldWeatherObs.length > 0}
          {#each fieldWeatherObs as obs (obs.id)}
            <div class="collapse collapse-arrow bg-base-100">
              <input type="radio" name="weather-accordion" />
              <div class="collapse-title text-lg font-medium">
                Weather at {obs.observationTime}
              </div>
              <div class="collapse-content relative">
                <button
                  class="btn btn-xs btn-outline btn-secondary absolute top-3 right-12"
                  title="Edit Weather Observation"
                  onclick={() => editWeather(obs.id)}
                >
                  Edit
                </button>
                <div class="grid grid-cols-2 gap-x-4 gap-y-1 text-sm pl-2">
                  <div>
                    <strong>Precip:</strong>
                    {getReadable(precipitationMap, obs.precipitation)}
                  </div>
                  <div>
                    <strong>Accum:</strong>
                    {getReadable(accumulationMap, obs.accumulation)}
                  </div>
                  <div>
                    <strong>Wind:</strong>
                    {getReadable(windSpeedMap, obs.windSpeed)} ({obs.windDirection ??
                      "?"})
                  </div>
                  <div>
                    <strong>Solar:</strong>
                    {obs.solarRadiation ?? "Unknown"}
                  </div>
                </div>
                {#if obs.comment}
                  <h4 class="font-bold text-base mt-2">Comment:</h4>
                  <p
                    class="text-base-content/80 p-3 bg-base-200 rounded-lg text-sm whitespace-pre-wrap"
                  >
                    {obs.comment}
                  </p>
                {/if}
              </div>
            </div>
          {/each}
        {:else}
          <p class="italic text-base-content/60 text-center p-4">
            No field weather recorded yet.
          </p>
        {/if}
      </div>
    </div>
  </div>

  <div class="card bg-base-200 shadow-xl">
    <div class="card-body">
      <div class="card-title flex justify-between items-center">
        <h2 class="text-xl font-bold">Avalanche Observations</h2>
        <button class="btn btn-primary btn-sm" onclick={addAvyObs}>
          {@html PlusIcon} Add Obs
        </button>
      </div>

      <div class="flex flex-col gap-2 mt-4">
        {#if trip.avyObservations.length > 0}
          {#each trip.avyObservations as obs (obs.id)}
            <div class="collapse collapse-arrow bg-base-100">
              <input type="radio" name="obs-accordion" />
              <div class="collapse-title text-lg font-medium">
                Observation at {obs.observationTime}
              </div>
              <div class="collapse-content relative">
                <button
                  class="btn btn-xs btn-outline btn-secondary absolute top-3 right-12"
                  title="Edit Observation"
                  onclick={() => editAvyObs(obs.id)}
                >
                  Edit
                </button>

                <h4 class="font-bold text-base mt-2">Avalanche Activity:</h4>
                <div class="grid grid-cols-2 gap-x-4 gap-y-1 text-sm pl-2">
                  <div>
                    <strong>Size:</strong>
                    {getReadable(avySizeMap, obs.avyActivitySize)}
                  </div>
                  <div>
                    <strong>Trigger:</strong>
                    {getReadable(avyTriggerMap, obs.avyActivityTrigger)}
                  </div>
                  <div class="col-span-2">
                    <strong>Type:</strong>
                    {getReadable(problemTypeMap, obs.avyActivityCharacteristic)}
                  </div>
                </div>

                <h4 class="font-bold text-base mt-2">Signs of Instability:</h4>
                <div class="grid grid-cols-2 gap-x-4 gap-y-1 text-sm pl-2">
                  <div>
                    <strong>See/Feel:</strong>
                    {getReadable(instabilityObsMap, obs.instabilitySeeFeel)}
                  </div>
                  <div>
                    <strong>CT:</strong>
                    {getReadable(testResultMap, obs.instabilityCt)}
                  </div>
                  <div>
                    <strong>ECT:</strong>
                    {getReadable(testResultMap, obs.instabilityEct)}
                  </div>
                </div>

                {#if obs.comment}
                  <h4 class="font-bold text-base mt-2">Comment:</h4>
                  <p
                    class="text-base-content/80 p-3 bg-base-200 rounded-lg text-sm whitespace-pre-wrap"
                  >
                    {obs.comment}
                  </p>
                {/if}
              </div>
            </div>
          {/each}
        {:else}
          <p class="italic text-base-content/60 text-center p-4">
            No avalanche observations recorded yet.
          </p>
        {/if}
      </div>
    </div>
  </div>
</main>
