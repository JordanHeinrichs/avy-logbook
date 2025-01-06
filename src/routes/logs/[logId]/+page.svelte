<script lang="ts">
    let { data } = $props();
</script>

<main class="container">
    <div>
        <h1 class="text-3xl font-bold text-center p-1">{data.log.name}</h1>
        <h2 class="text-3xl font-bold text-secondary p-1">{data.log.date}</h2>
    </div>

    <a class="btn btn-secondary" href="/">Back</a>
</main>
