<script>
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import Header from '$lib/components/Header.svelte';

	let { children } = $props();
</script>

<Header /> 
<div class="max-w-screen-xl mx-auto">
	<ModeWatcher />
	{@render children()}
</div>
