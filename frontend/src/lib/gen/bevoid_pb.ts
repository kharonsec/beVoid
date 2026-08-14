import { Message, MethodKind, proto3, ScalarType } from '@bufbuild/protobuf';
import type { ServiceType } from '@bufbuild/protobuf';

export interface HumRequest extends Message<HumRequest> {
	audioBase64: string;
}
export const HumRequest = proto3.makeMessageType<HumRequest>('bevoid.HumRequest', [
	{ no: 1, name: 'audio_base64', kind: 'scalar', T: ScalarType.STRING }
]);

export interface HumResponse extends Message<HumResponse> {
	uuid: string;
	emotion: string;
	freqHz: number;
	cssColor: string;
	colorName: string;
	colorExistsInSrgb: boolean;
	vibes: number;
	tsMs: bigint;
	srgbFallback: string;
}
export const HumResponse = proto3.makeMessageType<HumResponse>('bevoid.HumResponse', [
	{ no: 1, name: 'uuid', kind: 'scalar', T: ScalarType.STRING },
	{ no: 2, name: 'emotion', kind: 'scalar', T: ScalarType.STRING },
	{ no: 3, name: 'freq_hz', kind: 'scalar', T: ScalarType.DOUBLE },
	{ no: 4, name: 'css_color', kind: 'scalar', T: ScalarType.STRING },
	{ no: 5, name: 'color_name', kind: 'scalar', T: ScalarType.STRING },
	{ no: 6, name: 'color_exists_in_srgb', kind: 'scalar', T: ScalarType.BOOL },
	{ no: 7, name: 'vibes', kind: 'scalar', T: ScalarType.DOUBLE },
	{ no: 8, name: 'ts_ms', kind: 'scalar', T: ScalarType.INT64 },
	{ no: 9, name: 'srgb_fallback', kind: 'scalar', T: ScalarType.STRING }
]);

export interface ListEmotionsRequest extends Message<ListEmotionsRequest> {}
export const ListEmotionsRequest = proto3.makeMessageType<ListEmotionsRequest>(
	'bevoid.ListEmotionsRequest',
	[]
);

export interface EmotionPoint extends Message<EmotionPoint> {
	uuid: string;
	emotion: string;
	freqHz: number;
	cssColor: string;
	colorName: string;
	colorExistsInSrgb: boolean;
	vibes: number;
	tsMs: bigint;
	srgbFallback: string;
}
export const EmotionPoint = proto3.makeMessageType<EmotionPoint>('bevoid.EmotionPoint', [
	{ no: 1, name: 'uuid', kind: 'scalar', T: ScalarType.STRING },
	{ no: 2, name: 'emotion', kind: 'scalar', T: ScalarType.STRING },
	{ no: 3, name: 'freq_hz', kind: 'scalar', T: ScalarType.DOUBLE },
	{ no: 4, name: 'css_color', kind: 'scalar', T: ScalarType.STRING },
	{ no: 5, name: 'color_name', kind: 'scalar', T: ScalarType.STRING },
	{ no: 6, name: 'color_exists_in_srgb', kind: 'scalar', T: ScalarType.BOOL },
	{ no: 7, name: 'vibes', kind: 'scalar', T: ScalarType.DOUBLE },
	{ no: 8, name: 'ts_ms', kind: 'scalar', T: ScalarType.INT64 },
	{ no: 9, name: 'srgb_fallback', kind: 'scalar', T: ScalarType.STRING }
]);

export interface ListEmotionsResponse extends Message<ListEmotionsResponse> {
	points: EmotionPoint[];
}
export const ListEmotionsResponse = proto3.makeMessageType<ListEmotionsResponse>(
	'bevoid.ListEmotionsResponse',
	[{ no: 1, name: 'points', kind: 'message', T: EmotionPoint, repeated: true }]
);

export interface AuthenticateRequest extends Message<AuthenticateRequest> {
	x: number[];
	y: number[];
}
export const AuthenticateRequest = proto3.makeMessageType<AuthenticateRequest>(
	'bevoid.AuthenticateRequest',
	[
		{ no: 1, name: 'x', kind: 'scalar', T: ScalarType.DOUBLE, repeated: true },
		{ no: 2, name: 'y', kind: 'scalar', T: ScalarType.DOUBLE, repeated: true }
	]
);

export interface AuthenticateResponse extends Message<AuthenticateResponse> {
	ok: boolean;
	token: string;
}
export const AuthenticateResponse = proto3.makeMessageType<AuthenticateResponse>(
	'bevoid.AuthenticateResponse',
	[
		{ no: 1, name: 'ok', kind: 'scalar', T: ScalarType.BOOL },
		{ no: 2, name: 'token', kind: 'scalar', T: ScalarType.STRING }
	]
);

export const VoidService = {
	typeName: 'bevoid.VoidService',
	methods: {
		hum: { name: 'Hum', I: HumRequest, O: HumResponse, kind: MethodKind.Unary },
		listEmotions: {
			name: 'ListEmotions',
			I: ListEmotionsRequest,
			O: ListEmotionsResponse,
			kind: MethodKind.Unary
		},
		authenticate: {
			name: 'Authenticate',
			I: AuthenticateRequest,
			O: AuthenticateResponse,
			kind: MethodKind.Unary
		}
	}
} as const satisfies ServiceType;
