<script lang="ts">
	import { voidState } from './store';
	import { DEFAULT_STATE, saveState, type VoidState } from './bus';

	let { days = $bindable() }: { days?: string[] } = $props();

	let dragging: number | null = null;

	function update(fn: (s: VoidState) => VoidState) {
		voidState.update((state) => {
			const next = fn(state);
			saveState(next);
			days = next.weekOrder;
			return next;
		});
	}

	function onDragStart(i: number) {
		dragging = i;
	}

	function onDragOver(event: DragEvent) {
		event.preventDefault();
	}

	function onDrop(i: number) {
		if (dragging === null || dragging === i) return;
		update((state) => {
			const order = [...state.weekOrder];
			const [moved] = order.splice(dragging as number, 1);
			order.splice(i, 0, moved);
			return { ...state, weekOrder: order };
		});
		dragging = null;
	}

	function onKeyDown(event: KeyboardEvent, i: number) {
		if (!event.altKey) return;
		const target = event.key === 'ArrowDown' ? i + 1 : event.key === 'ArrowUp' ? i - 1 : -1;
		if (target < 0 || target >= (days ?? []).length) return;
		event.preventDefault();
		update((state) => {
			const order = [...state.weekOrder];
			const [moved] = order.splice(i, 1);
			order.splice(target, 0, moved);
			return { ...state, weekOrder: order };
		});
	}

	function reset() {
		update((state) => ({ ...state, weekOrder: [...DEFAULT_STATE.weekOrder] }));
	}
</script>

<div class="week">
	{#each days ?? [] as day, i}
		<button
			type="button"
			class="day"
			draggable="true"
			ondragstart={() => onDragStart(i)}
			ondragover={onDragOver}
			ondrop={() => onDrop(i)}
			ondragend={() => (dragging = null)}
			onkeydown={(event) => onKeyDown(event, i)}
		>
			<span class="grip">::</span> {day}
			<span class="keys">alt+↑↓</span>
		</button>
	{/each}
	<button class="reset" onclick={reset}>return to consensus time</button>
</div>

<style>
	.week {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	.day {
		width: 100%;
		text-align: left;
		padding: 10px 14px;
		background: #16102a;
		color: #d8d0ea;
		border: 1px solid #3a2b66;
		border-radius: 8px;
		cursor: grab;
		user-select: none;
	}
	.day:hover {
		border-color: #a78bfa;
	}
	.grip {
		color: #6b5b9e;
		margin-right: 8px;
		letter-spacing: 2px;
	}
	.keys {
		float: right;
		color: #4a3d78;
		font-size: 0.7rem;
	}
	.reset {
		margin-top: 6px;
		background: transparent;
		color: #8b7cc4;
		border: 1px dashed #3a2b66;
		border-radius: 8px;
		padding: 8px;
		cursor: pointer;
	}
</style>
