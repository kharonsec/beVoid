import { writable } from 'svelte/store';
import { DEFAULT_STATE, getState, type VoidState } from './bus';

export const voidState = writable<VoidState>(getState() ?? DEFAULT_STATE);
