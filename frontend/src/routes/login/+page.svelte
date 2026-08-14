<script lang="ts">
	import { voidClient } from '$lib/client';
	import { AuthenticateRequest } from '$lib/gen/bevoid_pb';

	let canvas: HTMLCanvasElement;
	let drawing = false;
	let stroke: { x: number; y: number }[] = $state([]);
	let message = $state('draw the Void Sigil: a circle, widdershins');
	let authenticated = $state(false);

	const TOKEN_KEY = 'bevoid-soul-token';

	function toLocal(event: PointerEvent) {
		const rect = canvas.getBoundingClientRect();
		return {
			x: (event.clientX - rect.left) / rect.width,
			y: (event.clientY - rect.top) / rect.height
		};
	}

	function onDown(event: PointerEvent) {
		drawing = true;
		stroke = [];
		canvas.setPointerCapture(event.pointerId);
		canvas.getContext('2d')?.beginPath();
		const p = toLocal(event);
		stroke = [p];
		canvas.getContext('2d')?.moveTo(p.x * canvas.width, p.y * canvas.height);
	}

	function onMove(event: PointerEvent) {
		if (!drawing) return;
		const p = toLocal(event);
		stroke = [...stroke, p];
		const ctx = canvas.getContext('2d')!;
		ctx.lineTo(p.x * canvas.width, p.y * canvas.height);
		ctx.strokeStyle = '#a78bfa';
		ctx.lineWidth = 3;
		ctx.stroke();
	}

	function onUp() {
		drawing = false;
	}

	function clearCanvas() {
		const ctx = canvas.getContext('2d')!;
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		stroke = [];
	}

	async function verify() {
		if (stroke.length < 8) {
			message = 'the sigil needs more strokes of faith';
			return;
		}
		message = 'consulting the void...';
		try {
			const res = await voidClient.authenticate(
				new AuthenticateRequest({
					x: stroke.map((p) => p.x),
					y: stroke.map((p) => p.y)
				})
			);
			if (res.ok) {
				localStorage.setItem(TOKEN_KEY, res.token);
				authenticated = true;
				message = `soul accepted. token: ${res.token.slice(0, 8)}…`;
			} else {
				message = 'the void does not recognize this sigil';
			}
		} catch (e) {
			message = `the void is not listening: ${String(e)}`;
		}
	}
</script>

<svelte:head>
	<title>beVoid: soul-auth</title>
</svelte:head>

<h1>soul authentication</h1>

<div class="card auth">
	<p class="hint">
		Draw a shape with your mouse. It will be compared to a Bézier curve stored in a TOML file on the
		server. No password has ever survived contact with the void.
	</p>
	<canvas
		bind:this={canvas}
		width={420}
		height={420}
		onpointerdown={onDown}
		onpointermove={onMove}
		onpointerup={onUp}
	></canvas>
	<div class="controls">
		<button onclick={verify}>verify my soul</button>
		<button class="ghost" onclick={clearCanvas}>reincarnate</button>
	</div>
	<p class="message {authenticated ? 'ok' : ''}">{message}</p>
</div>

<style>
	h1 {
		font-size: 1.4rem;
	}
	.auth {
		max-width: 500px;
	}
	.hint {
		color: #6b5b9e;
		font-size: 0.9rem;
	}
	canvas {
		background: #0a0618;
		border: 1px solid #3a2b66;
		border-radius: 10px;
		cursor: crosshair;
		touch-action: none;
	}
	.controls {
		display: flex;
		gap: 10px;
		margin-top: 12px;
	}
	button {
		background: #1c1440;
		color: #c4b5fd;
		border: 1px solid #3a2b66;
		border-radius: 8px;
		padding: 10px 16px;
		cursor: pointer;
	}
	button.ghost {
		background: transparent;
		color: #8b7cc4;
	}
	.message {
		color: #f472b6;
	}
	.message.ok {
		color: #a3e635;
	}
</style>
