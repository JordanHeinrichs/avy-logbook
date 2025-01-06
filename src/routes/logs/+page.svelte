<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Log } from "../lib";
    import { goto } from "$app/navigation";

    let name = $state("");
    let date = $state("");

    async function createLog(event: Event) {
        const log: Log = await invoke("create_log", { name, date });
        goto(`logs/${log.id}`);
    }
</script>

<main class="container">
    <h1 class="text-3xl font-bold text-center">Create Entry</h1>
    <form class="row" onsubmit={createLog}>
        <input
            id="name"
            class="input input-lg"
            placeholder="Enter a name..."
            bind:value={name}
        />
        <input
            id="date"
            class="input"
            placeholder="Enter a date..."
            bind:value={date}
        />
        <button class="btn btn-primary" type="submit">Create</button>
    </form>
</main>
