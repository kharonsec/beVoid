const CHANNEL = 'bevoid-void-bus';
const DB_NAME = 'bevoid-void-db';
const DB_VERSION = 1;
const STORE = 'kv';
const KEY = 'state';

function rot13(input) {
	let out = '';
	for (const ch of input) {
		const code = ch.charCodeAt(0);
		if (code >= 65 && code <= 90) out += String.fromCharCode(((code - 65 + 13) % 26) + 65);
		else if (code >= 97 && code <= 122) out += String.fromCharCode(((code - 97 + 13) % 26) + 97);
		else out += ch;
	}
	return out;
}

function encode(payload) {
	return rot13(JSON.stringify(payload));
}

function decode(raw) {
	return JSON.parse(rot13(raw));
}

function openDb() {
	return new Promise((resolve, reject) => {
		const req = indexedDB.open(DB_NAME, DB_VERSION);
		req.onupgradeneeded = () => {
			req.result.createObjectStore(STORE);
		};
		req.onsuccess = () => resolve(req.result);
		req.onerror = () => reject(req.error);
	});
}

async function persist(state) {
	const db = await openDb();
	await new Promise((resolve, reject) => {
		const tx = db.transaction(STORE, 'readwrite');
		tx.objectStore(STORE).put(state, KEY);
		tx.oncomplete = resolve;
		tx.onerror = () => reject(tx.error);
	});
	db.close();
}

async function restore() {
	const db = await openDb();
	const state = await new Promise((resolve, reject) => {
		const tx = db.transaction(STORE, 'readonly');
		const req = tx.objectStore(STORE).get(KEY);
		req.onsuccess = () => resolve(req.result);
		req.onerror = () => reject(req.error);
	});
	db.close();
	return state;
}

function broadcast(msg) {
	const channel = new BroadcastChannel(CHANNEL);
	channel.postMessage(encode(msg));
	channel.close();
}

async function broadcastState() {
	const state = await restore();
	if (state) broadcast({ type: 'state', state });
}

const bus = new BroadcastChannel(CHANNEL);

bus.onmessage = async (event) => {
	if (typeof event.data !== 'string') return;
	const msg = decode(event.data);
	if (msg.type === 'set') {
		await persist(msg.state);
		broadcast({ type: 'state', state: msg.state });
	} else if (msg.type === 'get') {
		await broadcastState();
	}
};

self.addEventListener('install', () => self.skipWaiting());

self.addEventListener('activate', (event) => {
	event.waitUntil(self.clients.claim());
	broadcastState();
});
