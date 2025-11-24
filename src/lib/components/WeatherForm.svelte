<script lang="ts">
  import type { NewWeather } from "$lib/types/NewWeather";
  import type { Precipitation } from "$lib/types/Precipitation";
  import type { Accumulation } from "$lib/types/Accumulation";
  import type { WindSpeed } from "$lib/types/WindSpeed";
  import type { WindDirection } from "$lib/types/WindDirection";
  import type { SolarRadiation } from "$lib/types/SolarRadiation";

  let { weather = $bindable(), isPlan } = $props<{
    weather: NewWeather;
    isPlan: boolean;
  }>();

  const precipitationOptions: { value: Precipitation; label: string }[] = [
    { value: "nothing", label: "Nil" },
    { value: "s1", label: "S-1" },
    { value: "rL", label: "Rain (L)" },
    { value: "s2", label: "S-2" },
    { value: "r", label: "Rain" },
    { value: "s3Plus", label: "S-3+" },
    { value: "unknown", label: "?" },
  ];

  const accumulationOptions: { value: Accumulation; label: string }[] = [
    { value: "zero", label: "0" },
    { value: "lessThan20", label: "< 20" },
    { value: "from20To40", label: "20-40" },
    { value: "moreThan40", label: "> 40" },
    { value: "unknown", label: "?" },
  ];

  const windSpeedOptions: { value: WindSpeed; label: string }[] = [
    { value: "calm", label: "Calm" },
    { value: "light", label: "Light" },
    { value: "moderate", label: "Mod" },
    { value: "strong", label: "Strong" },
    { value: "extreme", label: "Ext" },
    { value: "unknown", label: "?" },
  ];

  const solarOptions: { value: SolarRadiation; label: string }[] = [
    { value: "none", label: "None" },
    { value: "weak", label: "Weak" },
    { value: "moderate", label: "Mod" },
    { value: "strong", label: "Strong" },
    { value: "unknown", label: "?" },
  ];

  const windDirectionGrid: (WindDirection | null)[] = [
    "NW",
    "N",
    "NE",
    "W",
    null,
    "E",
    "SW",
    "S",
    "SE",
  ];
</script>

<div class="flex flex-col gap-6">
  {#if !isPlan}
    <div class="card bg-base-200 shadow-sm">
      <div class="card-body p-4">
        <label class="form-control w-full">
          <div class="label">
            <span class="label-text font-bold">Observation Time</span>
          </div>
          <input
            type="time"
            class="input input-bordered w-full text-lg"
            bind:value={weather.observationTime}
          />
        </label>
      </div>
    </div>
  {/if}

  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <h3 class="font-bold mb-2">Precipitation</h3>
      <div class="grid grid-cols-3 gap-2">
        {#each precipitationOptions as opt}
          <button
            class="btn {weather.precipitation === opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}"
            onclick={() => (weather.precipitation = opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>
    </div>
  </div>

  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <h3 class="font-bold mb-2">Accumulation (cm)</h3>
      <div class="grid grid-cols-3 gap-2">
        {#each accumulationOptions as opt}
          <button
            class="btn {weather.accumulation === opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}"
            onclick={() => (weather.accumulation = opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>
    </div>
  </div>

  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <h3 class="font-bold mb-2">Wind Speed</h3>
      <div class="grid grid-cols-3 gap-2">
        {#each windSpeedOptions as opt}
          <button
            class="btn {weather.windSpeed === opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}"
            onclick={() => (weather.windSpeed = opt.value)}
          >
            {opt.label}
          </button>
        {/each}
      </div>
    </div>
  </div>

  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4 items-center">
      <h3 class="font-bold mb-2 self-start">Wind Direction</h3>
      <div class="grid grid-cols-3 gap-2 w-full max-w-xs">
        {#each windDirectionGrid as dir}
          {#if dir}
            <button
              class="btn h-14 {weather.windDirection === dir
                ? 'btn-primary'
                : 'btn-outline bg-base-100'}"
              onclick={() => (weather.windDirection = dir)}
            >
              {dir}
            </button>
          {:else}
            <div class="flex items-center justify-center">
              <div class="w-2 h-2 bg-base-content/20 rounded-full"></div>
            </div>
          {/if}
        {/each}
      </div>
      <button
        class="btn w-full mt-2 max-w-xs {weather.windDirection === 'Unknown'
          ? 'btn-primary'
          : 'btn-outline bg-base-100'}"
        onclick={() => (weather.windDirection = "Unknown")}
      >
        Unknown
      </button>
    </div>
  </div>

  <div class="card bg-base-200 shadow-sm">
    <div class="card-body p-4">
      <h3 class="font-bold mb-2">Solar Radiation</h3>
      <div class="grid grid-cols-3 gap-2">
        {#each solarOptions as opt}
          <button
            class="btn {weather.solarRadiation === opt.value
              ? 'btn-primary'
              : 'btn-outline bg-base-100'}"
            onclick={() => (weather.solarRadiation = opt.value)}
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
          placeholder="General weather notes..."
          bind:value={weather.comment}
        ></textarea>
      </label>
    </div>
  </div>
</div>
