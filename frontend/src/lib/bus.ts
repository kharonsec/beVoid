import { rot13 } from './rot13';

export const CHANNEL = 'bevoid-void-bus';

export interface VoidState {
	weekOrder: string[];
	calendar: 'gregorian' | 'emotional';
}

export const DEFAULT_STATE: VoidState = {
	weekOrder: ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'],
	calendar: 'gregorian'
};

const STORAGE_KEY = 'bevoid-state-fallback';

function loadFallback(): VoidState {
	if (typeof localStorage === 'undefined') return DEFAULT_STATE;
	const raw = localStorage.getItem(STORAGE_KEY);
	if (!raw) return DEFAULT_STATE;
	try {
		return JSON.parse(raw) as VoidState;
	} catch {
		return DEFAULT_STATE;
	}
}

function encode(payload: unknown): string {
	return rot13(JSON.stringify(payload));
}

function decode<T>(raw: string): T {
	return JSON.parse(rot13(raw)) as T;
}

export type BusMessage =
	| { type: 'set'; state: VoidState }
	| { type: 'state'; state: VoidState }
	| { type: 'get' };

type Listener = (state: VoidState) => void;

export function openBus(): { send: (msg: BusMessage) => void; onState: (fn: Listener) => void; close: () => void } {
	const channel = new BroadcastChannel(CHANNEL);
	const listeners = new Set<Listener>();

	channel.onmessage = (event: MessageEvent) => {
		if (typeof event.data !== 'string') return;
		const msg = decode<BusMessage>(event.data);
		if (msg.type === 'state') {
			listeners.forEach((fn) => fn(msg.state));
			if (typeof localStorage !== 'undefined') {
				localStorage.setItem(STORAGE_KEY, JSON.stringify(msg.state));
			}
		}
	};

	return {
		send: (msg) => channel.postMessage(encode(msg)),
		onState: (fn) => {
			listeners.add(fn);
		},
		close: () => channel.close()
	};
}

export function getState(): VoidState {
	return loadFallback();
}

export function saveState(state: VoidState): void {
	if (typeof localStorage !== 'undefined') {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
	}
	const channel = new BroadcastChannel(CHANNEL);
	channel.postMessage(encode({ type: 'set', state }));
	channel.close();
}
