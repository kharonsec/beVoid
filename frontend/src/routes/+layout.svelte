<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { openBus } from '$lib/bus';
	import { voidState } from '$lib/store';

	let { children } = $props();

	onMount(() => {
		if (!('serviceWorker' in navigator)) return;
		navigator.serviceWorker.register('/service-worker.js');

		const bus = openBus();
		bus.send({ type: 'get' });
		bus.onState((state) => voidState.set(state));
		return () => bus.close();
	});
</script>

<header>
	<a href="/" class="logo">beVoid<span class="dot">·</span></a>
	<nav>
		<a href="/">the void</a>
		<a href="/settings">settings</a>
		<a href="/login">soul-auth</a>
	</nav>
</header>

<main>
	{@render children()}
</main>

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 14px 24px;
		border-bottom: 1px solid #241a45;
	}
	.logo {
		font-weight: 700;
		font-size: 1.3rem;
		color: #c4b5fd;
		text-decoration: none;
		letter-spacing: 0.5px;
	}
	.dot {
		color: #6b5b9e;
		animation: pulse 2s infinite;
	}
	@keyframes pulse {
		50% {
			opacity: 0.2;
		}
	}
	nav {
		display: flex;
		gap: 20px;
	}
	nav a {
		color: #8b7cc4;
		text-decoration: none;
		font-size: 0.9rem;
	}
	nav a:hover {
		color: #c4b5fd;
	}
	main {
		padding: 24px;
	}
</style>
