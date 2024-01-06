import {
	setupControlCHandle,
	switchCursor,
	switchScreenMode,
	writeAt,
} from "./utils";

const { stdout, stdin } = process;

type RectConfig = {
	sX?: number;
	sY?: number;
	eX?: number;
	eY?: number;
	char?: string;
};

export default class Board {
	width: number;
	height: number;
	buffer: string[][];
	pending: Map<string, string>;

	constructor() {
		this.width = stdout.columns;
		this.height = stdout.rows;
		this.buffer = Array.from({ length: this.height }, () =>
			Array(this.height).fill(" "),
		);
		this.pending = new Map();
	}

	get centerX() {
		return ~~(this.width / 2);
	}

	get centerY() {
		return ~~(this.height / 2);
	}

	addPending(x: number, y: number, char: string) {
		const key = `${x} ${y}`;
		char === this.buffer[y][x]
			? this.pending.delete(key)
			: this.pending.set(key, char);
	}

	putChars(x: number, y: number, chars: string[]) {
		for (const [i, char] of chars.entries()) {
			this.addPending(x + i, y, char);
		}
	}

	flush() {
		for (const [key, ch] of this.pending) {
			const [col, row] = key.split(" ").map(Number);
			this.buffer[row][col] = ch;
			writeAt(col, row, ch);
		}
		this.pending.clear();
	}

	fillRect({
		sX = 0,
		sY = 0,
		eX = this.width,
		eY = this.height,
		char = " ",
	}: RectConfig) {
		for (let y = sY; y < eY; y++) {
			for (let x = sX; x < eX; x++) this.addPending(x, y, char);
		}
	}

	enter() {
		switchCursor(false);
		switchScreenMode(true);
		stdin.setRawMode(true);
		process.on("exit", () => {
			switchScreenMode(false);
			switchCursor(true);
		});
		setupControlCHandle();
	}
}
