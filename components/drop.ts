import type Board from "../board";
import { COLORS, charBright, colorify } from "../utils";

type DropConfig = {
	x: number;
	size: number;
	delay: number;
	getChar?: (percent: number) => string;
};
export default class Drop {
	x: number;
	size: number;
	delay: number;
	board: Board;
	cooldown = 0;
	start = 0;
	end = 0;
	getChar: (percent: number) => string;

	constructor(board: Board, { size, delay, x: y }: DropConfig) {
		this.board = board;
		this.x = y;
		this.size = size;
		this.delay = delay;
		this.getChar = (percent) => colorify(`${charBright(percent)}`, COLORS.foam);
	}

	get isValid() {
		return this.start < this.board.height - 1;
	}

	update() {
		const {
			start,
			end,
			size,
			cooldown,
			delay,
			isValid,
			board: { height },
		} = this;
		const isUpdating = cooldown >= delay;
		if (isUpdating && isValid) {
			this.cooldown = 0;
			if (end >= size && start < height) {
				this.start += 1;
			}
			if (end < height) {
				this.end += 1;
			}
		} else {
			this.cooldown += 1;
		}
	}

	draw() {
		const { isValid, start, end, x } = this;
		if (isValid) {
			for (let y = start; y < end; y++) {
				const percent = (y - start) / (end - start);
				const char = this.getChar(percent);
				this.board.addPending(x, y, char);
			}
		}
	}
}
