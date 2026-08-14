<script lang="ts">
	import { CHEESE_MONTHS, DAYS_PER_MONTH, DAYS_PER_YEAR, emotionalFullDate, gregorianFullDate } from '$lib/calendar';
	import { saveState } from '$lib/bus';
	import { voidState } from '$lib/store';

	let weekOrder = $state<string[]>([]);
	voidState.subscribe((s) => (weekOrder = s.weekOrder));

	const today = new Date();

	function setCalendar(mode: 'gregorian' | 'emotional') {
		voidState.update((state) => {
			const next = { ...state, calendar: mode };
			saveState(next);
			return next;
		});
	}
</script>

<svelte:head>
	<title>beVoid: settings</title>
</svelte:head>

<h1>calendar settings</h1>

<div class="card">
	<h2>reality framework</h2>
	<div class="toggle">
		<button
			class:active={$voidState.calendar === 'gregorian'}
			onclick={() => setCalendar('gregorian')}
		>
			Gregorian
		</button>
		<button
			class:active={$voidState.calendar === 'emotional'}
			onclick={() => setCalendar('emotional')}
		>
			Emotional
		</button>
	</div>
	<p class="today">
		{$voidState.calendar === 'emotional'
			? emotionalFullDate(today, weekOrder)
			: gregorianFullDate(today, weekOrder)}
	</p>
</div>

<div class="card">
	<h2>the 13 cheese months</h2>
	<p class="hint">28 days each, for a year of 364 days. Day 365 is the Nil Day, the void between years.</p>
	<table>
		<thead>
			<tr><th>#</th><th>Month</th><th>Days</th></tr>
		</thead>
		<tbody>
			{#each CHEESE_MONTHS as month, i}
				<tr>
					<td>{i + 1}</td>
					<td>{month}</td>
					<td>{i * DAYS_PER_MONTH + 1}–{(i + 1) * DAYS_PER_MONTH}</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<p class="hint">your personal week order ({weekOrder.join(' · ')}) applies inside the emotional calendar too.</p>
	<p class="nil">after Day {DAYS_PER_YEAR}: Nil Day. Nobody knows what day it is, and that is the point.</p>
</div>

<style>
	h1 {
		font-size: 1.4rem;
	}
	.card {
		margin-bottom: 16px;
		max-width: 640px;
	}
	.card h2 {
		margin-top: 0;
	}
	.toggle {
		display: flex;
		gap: 10px;
	}
	.toggle button {
		background: #16102a;
		color: #8b7cc4;
		border: 1px solid #3a2b66;
		border-radius: 8px;
		padding: 10px 18px;
		cursor: pointer;
	}
	.toggle button.active {
		background: #2b1e5e;
		color: #c4b5fd;
		border-color: #a78bfa;
	}
	.today {
		color: #a78bfa;
		margin-top: 14px;
	}
	.hint {
		color: #6b5b9e;
		font-size: 0.85rem;
	}
	table {
		width: 100%;
		border-collapse: collapse;
		margin: 10px 0;
	}
	th,
	td {
		text-align: left;
		padding: 6px 10px;
		border-bottom: 1px solid #241a45;
		font-size: 0.9rem;
	}
	th {
		color: #6b5b9e;
	}
	.nil {
		color: #f472b6;
		font-size: 0.85rem;
	}
</style>
