export function rot13(input: string): string {
	let out = '';
	for (const ch of input) {
		const code = ch.charCodeAt(0);
		if (code >= 65 && code <= 90) out += String.fromCharCode(((code - 65 + 13) % 26) + 65);
		else if (code >= 97 && code <= 122) out += String.fromCharCode(((code - 97 + 13) % 26) + 97);
		else out += ch;
	}
	return out;
}
