<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>My Avy Logbook</h1>

<p>This will be the table. This is also the only page that can not be prerendered</p>

<p>Welcome {data.post.name}</p>

<footer>
	<a href="/privacy-policy">Privacy Policy</a>
	<p>© 2023 Jordan Heinrichs</p>
</footer>
