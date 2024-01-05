export const COLORS = {
	base: [25, 23, 36],
	surface: [31, 29, 46],
	overlay: [38, 35, 58],
	muted: [110, 106, 134],
	subtle: [144, 140, 170],
	text: [224, 222, 244],
	love: [235, 111, 146],
	gold: [246, 193, 119],
	rose: [235, 188, 186],
	pine: [49, 116, 143],
	foam: [156, 207, 216],
	iris: [196, 167, 231],
	highlight_low: [33, 32, 46],
	highlight_med: [64, 61, 82],
	highlight_high: [82, 79, 103],
} as const;

const { stdout, stdin, exit } = process;

export const moveCursor = (column: number, row: number) =>
	stdout.write(`\x1b[?${row + 1};${column + 1}H`);

export const switchCursor = (visible: boolean) =>
	stdout.write(visible ? "\x1b[?25h" : "\x1b[?25l");

export const switchScreenMode = (alternate: boolean) =>
	stdout.write(alternate ? "\x1b[?1049h" : "\x1b[?1049l");

export const setupControlCHandle = () =>
	stdin.on("data", (data) => {
		if (data[0] === 3) exit();
	});

export const writeAt = (column: number, row: number, char: string) => {
	moveCursor(column, row);
	stdout.write(char);
};

type RGB = [number, number, number];

export const colorify = (
	char: string,
	[r, g, b]: RGB | (typeof COLORS)[keyof typeof COLORS],
) => `\x1b[38;2;${r};${g};${b}m${char}\x1b[0m`;

export const italic = (char: string) => `\x1b[3m${char}\x1b[0m`;

const BRIGHT_SCALE = ".`-wW";

export const charBright = (brightness: number) => {
	const index = Math.floor(brightness * (BRIGHT_SCALE.length - 1));
	return BRIGHT_SCALE[index];
};

export const playSong = (path: string) => {
	const p = Bun.spawn(["afplay", path]);
	process.on("exit", () => {
		p.kill();
	});
	return p;
};

export const padCenter = (chars: string[], size: number) => {
	const pad = Array(Math.floor((size - chars.length) / 2)).fill(" ");
	const remain = Array(size - (pad.length * 2 + chars.length)).fill(" ");
	return [...remain, ...pad, ...chars, ...pad];
};
