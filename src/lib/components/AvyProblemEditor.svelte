<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { AvalancheProblem } from "$lib/types/AvalancheProblem";
  import type { ProblemType } from "$lib/types/ProblemType";
  import type { Elevation } from "$lib/types/Elevation";

  let { forecastId, problems = $bindable<AvalancheProblem[]>() } = $props<{
    forecastId: number;
    problems: AvalancheProblem[];
  }>();

  // --- Configuration ---
  const ELEVATIONS: Elevation[] = ["ALP", "TL", "BTL"];

  const PROBLEM_TYPES: { type: ProblemType; label: string; short: string }[] = [
    { type: "stormSlabs", label: "Storm Slabs", short: "SS" },
    { type: "windSlabs", label: "Wind Slabs", short: "WD" },
    { type: "persistentSlabs", label: "Persistent Slabs", short: "PS" },
    { type: "deepPersistentSlabs", label: "Deep Persistent", short: "DP" },
    { type: "wetSlab", label: "Wet Slabs", short: "WS" },
    { type: "looseDry", label: "Loose Dry", short: "LD" },
    { type: "looseWet", label: "Loose Wet", short: "LW" },
    { type: "cornices", label: "Cornices", short: "CO" },
  ];

  function findProblem(
    problem: ProblemType,
    elevation: Elevation
  ): AvalancheProblem | undefined {
    return problems.find(
      (p) => p.problemType === problem && p.elevation === elevation
    );
  }

  function getBtnClass(exists: boolean): string {
    const base = "btn btn-sm h-10 w-full px-0";
    if (exists) {
      return `${base} btn-error text-white`;
    }
    return `${base} btn-outline btn-ghost border-base-300 text-base-content/30`;
  }

  async function toggleProblem(problem: ProblemType, elevation: Elevation) {
    const existing = findProblem(problem, elevation);

    if (existing) {
      try {
        await invoke("delete_avy_problem", { id: existing.id });
        problems = problems.filter((p) => p.id !== existing.id);
      } catch (e) {
        console.error("Failed to delete problem", e);
      }
    } else {
      try {
        const newProblem: AvalancheProblem = await invoke(
          "create_avy_problem",
          {
            forecastId,
            elevation: elevation,
            problemType: problem,
          }
        );
        problems = [...problems, newProblem];
      } catch (e) {
        console.error("Failed to create problem", e);
      }
    }
  }
</script>

<div class="flex flex-col gap-3 w-full">
  <div
    class="grid grid-cols-[2fr_1fr_1fr_1fr] gap-2 items-center text-center font-bold text-sm"
  >
    <div class="text-left">Problem</div>
    <div>ALP</div>
    <div>TL</div>
    <div>BTL</div>
  </div>

  {#each PROBLEM_TYPES as p}
    <div class="grid grid-cols-[2fr_1fr_1fr_1fr] gap-2 items-center">
      <div class="text-sm font-semibold leading-tight">
        {p.label}
      </div>

      {#each ELEVATIONS as elev}
        {@const existing = findProblem(p.type, elev)}
        <button
          class={getBtnClass(!!existing)}
          onclick={() => toggleProblem(p.type, elev)}
          aria-label={`Toggle ${p.label} at ${elev}`}
        >
          {elev}
        </button>
      {/each}
    </div>
  {/each}
</div>
