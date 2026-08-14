<script lang="ts">
	import { onMount } from 'svelte';

	interface Point {
		x: number;
		y: number;
		z: number;
		vibes: number;
		color: [number, number, number];
	}

	let { points = [] }: { points?: Point[] } = $props();

	let canvas: HTMLCanvasElement;
	let gl: WebGLRenderingContext;
	let buffer: WebGLBuffer;
	let count = 0;
	let rotation = 0;

	const vertex = `
attribute vec3 a_pos;
attribute float a_vibes;
attribute vec3 a_color;
uniform float u_time;
uniform float u_scale;
varying vec3 v_color;
varying float v_vibes;
void main() {
  float r = u_time * 0.4;
  float cr = cos(r), sr = sin(r);
  vec3 p = vec3(a_pos.x * cr - a_pos.z * sr, a_pos.y, a_pos.x * sr + a_pos.z * cr);
  p.y += sin(u_time * 1.7 + a_vibes * 6.2831) * 0.03;
  float zf = 1.0 / (2.5 - p.z * 0.3);
  gl_Position = vec4(p.x * u_scale * zf, p.y * u_scale * zf, 0.0, 1.0);
  gl_PointSize = (1.5 + a_vibes * 10.0) * zf * 2.0;
  v_color = a_color;
  v_vibes = a_vibes;
}`;

	const fragment = `
precision mediump float;
varying vec3 v_color;
varying float v_vibes;
void main() {
  float d = distance(gl_PointCoord, vec2(0.5));
  if (d > 0.5) discard;
  float glow = smoothstep(0.5, 0.0, d);
  gl_FragColor = vec4(v_color * (0.6 + v_vibes * 0.8), glow * (0.35 + v_vibes * 0.65));
}`;

	function compile(type: number, source: string): WebGLShader {
		const shader = gl.createShader(type)!;
		gl.shaderSource(shader, source);
		gl.compileShader(shader);
		return shader;
	}

	function upload() {
		const data = new Float32Array(points.length * 7);
		points.forEach((p, i) => {
			const o = i * 7;
			data[o] = p.x;
			data[o + 1] = p.y;
			data[o + 2] = p.z;
			data[o + 3] = p.vibes;
			data[o + 4] = p.color[0];
			data[o + 5] = p.color[1];
			data[o + 6] = p.color[2];
		});
		gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
		gl.bufferData(gl.ARRAY_BUFFER, data, gl.DYNAMIC_DRAW);
		count = points.length;
	}

	function frame(now: number) {
		if (!gl) return;
		const width = canvas.clientWidth * devicePixelRatio;
		const height = canvas.clientHeight * devicePixelRatio;
		if (canvas.width !== width || canvas.height !== height) {
			canvas.width = width;
			canvas.height = height;
			gl.viewport(0, 0, width, height);
		}
		gl.clearColor(0.02, 0.01, 0.05, 1);
		gl.clear(gl.COLOR_BUFFER_BIT);
		if (count > 0) {
			gl.uniform1f(gl.getUniformLocation(program, 'u_time'), now / 1000);
			gl.uniform1f(
				gl.getUniformLocation(program, 'u_scale'),
				0.6 * Math.min(width, height)
			);
			gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
			gl.drawArrays(gl.POINTS, 0, count);
		}
		requestAnimationFrame(frame);
	}

	let program: WebGLProgram;

	onMount(() => {
		gl = canvas.getContext('webgl')!;
		buffer = gl.createBuffer()!;
		program = gl.createProgram()!;
		gl.attachShader(program, compile(gl.VERTEX_SHADER, vertex));
		gl.attachShader(program, compile(gl.FRAGMENT_SHADER, fragment));
		gl.linkProgram(program);
		gl.useProgram(program);
		gl.enable(gl.BLEND);
		gl.blendFunc(gl.SRC_ALPHA, gl.ONE);

		const stride = 7 * 4;
		const posLoc = gl.getAttribLocation(program, 'a_pos');
		gl.enableVertexAttribArray(posLoc);
		gl.vertexAttribPointer(posLoc, 3, gl.FLOAT, false, stride, 0);
		const vibesLoc = gl.getAttribLocation(program, 'a_vibes');
		gl.enableVertexAttribArray(vibesLoc);
		gl.vertexAttribPointer(vibesLoc, 1, gl.FLOAT, false, stride, 12);
		const colorLoc = gl.getAttribLocation(program, 'a_color');
		gl.enableVertexAttribArray(colorLoc);
		gl.vertexAttribPointer(colorLoc, 3, gl.FLOAT, false, stride, 16);

		upload();
		requestAnimationFrame(frame);
	});

	$effect(() => {
		if (gl && buffer) upload();
	});
</script>

<canvas bind:this={canvas} class="chart"></canvas>

<style>
	.chart {
		width: 100%;
		height: 100%;
		display: block;
	}
</style>
