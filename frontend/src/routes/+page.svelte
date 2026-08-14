<script lang="ts">
	import { onMount } from 'svelte';
	import Chart4D from '$lib/Chart4D.svelte';
	import WeekOrder from '$lib/WeekOrder.svelte';
	import { voidClient } from '$lib/client';
	import { HumRequest, type EmotionPoint } from '$lib/gen/bevoid_pb';
	import { voidState } from '$lib/store';

	interface ChartPoint {
		x: number;
		y: number;
		z: number;
		vibes: number;
		color: [number, number, number];
	}

	let points: ChartPoint[] = $state([]);
	let rawPoints: EmotionPoint[] = $state([]);
	let status = $state('awaiting your hum');
	let backendDown = $state(false);
	let days = $state<string[]>([]);
	let fileInput: HTMLInputElement;

	voidState.subscribe((s) => (days = s.weekOrder));

	const EMOTION_Z: Record<string, number> = {
		despair: -1,
		yearning: -0.6,
		melancholy: -0.25,
		pondering: 0.1,
		contentment: 0.45,
		giddiness: 0.75,
		euphoria: 1
	};

	function hexToRgb(hex: string): [number, number, number] {
		const value = hex.replace('#', '');
		return [
			parseInt(value.slice(0, 2), 16) / 255,
			parseInt(value.slice(2, 4), 16) / 255,
			parseInt(value.slice(4, 6), 16) / 255
		];
	}

	function toChartPoints(rows: EmotionPoint[]): ChartPoint[] {
		if (rows.length === 0) return [];
		const minTs = Math.min(...rows.map((r) => Number(r.tsMs)));
		const maxTs = Math.max(...rows.map((r) => Number(r.tsMs)));
		const span = Math.max(1, maxTs - minTs);
		return rows.map((r) => ({
			x: ((Number(r.tsMs) - minTs) / span - 0.5) * 1.6,
			y: (r.freqHz / 1000 - 0.5) * 1.4,
			z: EMOTION_Z[r.emotion] ?? 0,
			vibes: r.vibes,
			color: hexToRgb(r.srgbFallback)
		}));
	}

	async function loadEmotions() {
		try {
			const res = await voidClient.listEmotions({});
			rawPoints = res.points;
			points = toChartPoints(res.points);
			backendDown = false;
		} catch {
			backendDown = true;
			status = 'the void is unreachable (is the backend humming?)';
		}
	}

	async function onFileSelected(event: Event) {
		const file = (event.target as HTMLInputElement).files?.[0];
		if (!file) return;
		status = 'reverse-hashing your hum...';
		const audioBase64 = await new Promise<string>((resolve, reject) => {
			const reader = new FileReader();
			reader.onload = () => resolve((reader.result as string).split(',')[1]);
			reader.onerror = () => reject(reader.error);
			reader.readAsDataURL(file);
		});
		try {
			const res = await voidClient.hum(new HumRequest({ audioBase64 }));
			status = `${res.emotion}: ${res.colorName} (${res.freqHz.toFixed(1)} Hz, vibes ${res.vibes.toFixed(2)})`;
			await loadEmotions();
		} catch (e) {
			status = `the void rejected your hum: ${String(e)}`;
		}
	}

	onMount(() => {
		loadEmotions();
	});
</script>

<svelte:head>
	<title>beVoid: the void</title>
</svelte:head>

<div class="grid">
	<section class="card chart-card">
		<h2>4D scatter: time, frequency, emotion, vibes</h2>
		<div class="chart-wrap">
			<Chart4D {points} />
		</div>
		<p class="status {backendDown ? 'down' : ''}">{status}</p>
		<div class="hum">
			<input bind:this={fileInput} type="file" accept="audio/wav,.wav" onchange={onFileSelected} hidden />
			<button onclick={() => fileInput.click()}>hum at the void (upload .wav)</button>
		</div>
	</section>

	<aside class="card side">
		<h3>personal week order</h3>
		<p class="hint">drag to rearrange; persists via BroadcastChannel in ROT13</p>
		<WeekOrder bind:days />
	</aside>
</div>

<div class="card ledger">
	<h3>emotion ledger (append-only)</h3>
	{#if rawPoints.length === 0}
		<p class="hint">no emotions have been recorded. the void is empty.</p>
	{:else}
		{#each rawPoints as p}
			<div class="row">
				<span class="swatch" style="background: {p.srgbFallback}"></span>
				<span>{p.emotion}</span>
				<span class="freq">{p.freqHz.toFixed(1)} Hz</span>
				<span class="color">{p.colorName}</span>
				<span class="uuid">{p.uuid.slice(0, 13)}…</span>
			</div>
		{/each}
	{/if}
</div>

<style>
	.grid {
		display: grid;
		grid-template-columns: 1fr 280px;
		gap: 16px;
		margin-bottom: 16px;
	}
	@media (max-width: 800px) {
		.grid {
			grid-template-columns: 1fr;
		}
	}
	.chart-card h2 {
		margin-top: 0;
		font-size: 1rem;
	}
	.chart-wrap {
		height: 420px;
	}
	.status {
		color: #8b7cc4;
		min-height: 1.2em;
	}
	.status.down {
		color: #f472b6;
	}
	.hum button {
		background: #1c1440;
		color: #c4b5fd;
		border: 1px solid #3a2b66;
		border-radius: 8px;
		padding: 10px 16px;
		cursor: pointer;
	}
	.side h3 {
		margin-top: 0;
	}
	.hint {
		color: #6b5b9e;
		font-size: 0.85rem;
	}
	.ledger {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
	.ledger h3 {
		margin-top: 0;
	}
	.row {
		display: flex;
		align-items: center;
		gap: 14px;
		font-size: 0.85rem;
	}
	.swatch {
		width: 14px;
		height: 14px;
		border-radius: 50%;
	}
	.freq,
	.uuid {
		color: #6b5b9e;
	}
	.color {
		color: #a78bfa;
	}
</style>
