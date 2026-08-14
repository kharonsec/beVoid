export const CHEESE_MONTHS = [
	'Gouda',
	'Brie',
	'Camembert',
	'Roquefort',
	'Manchego',
	'Emmental',
	'Gorgonzola',
	'Havarti',
	'Pecorino',
	'Monterey Jack',
	'Stilton',
	'Gruyère',
	'Cheddar'
];

export const DAYS_PER_MONTH = 28;
export const DAYS_PER_YEAR = CHEESE_MONTHS.length * DAYS_PER_MONTH;

export interface EmotionalDate {
	nilDay: boolean;
	month: string;
	day: number;
}

export function emotionalDate(date: Date): EmotionalDate {
	const start = new Date(date.getFullYear(), 0, 1);
	const dayOfYear = Math.floor((date.getTime() - start.getTime()) / 86_400_000);
	if (dayOfYear >= DAYS_PER_YEAR) {
		return { nilDay: true, month: 'The Nil Day', day: 1 };
	}
	return {
		nilDay: false,
		month: CHEESE_MONTHS[Math.floor(dayOfYear / DAYS_PER_MONTH)],
		day: (dayOfYear % DAYS_PER_MONTH) + 1
	};
}

export function emotionalWeekday(date: Date, weekOrder: string[]): string {
	if (weekOrder.length !== 7) return '?';
	const idx = (date.getDay() + 6) % 7;
	return weekOrder[idx];
}

export function emotionalFullDate(date: Date, weekOrder: string[]): string {
	const ed = emotionalDate(date);
	if (ed.nilDay) return `Nil Day: the void between years`;
	return `${emotionalWeekday(date, weekOrder)}, Day ${ed.day} of ${ed.month}`;
}

export function gregorianFullDate(date: Date, weekOrder: string[]): string {
	return `${weekOrder[(date.getDay() + 6) % 7] ?? '?'}, ${date.toDateString()}`;
}
