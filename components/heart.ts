import type Board from "../board";
import { COLORS, charBright, colorify } from "../utils";

const { cos, sin, floor } = Math;

type HeartConfig = {
	cX?: number;
	cY?: number;
	size?: number;
	minSize?: number;
	maxSize?: number;
	initialSize?: number;
	isGrowing?: boolean;
	delay?: number;
	getChar?: (offset: number) => string;
};

export default class Heart {
	board: Board;
	size: number;
	minSize: number;
	maxSize: number;
	getChar: (offset: number) => string;
	cX: number;
	cY: number;
	isGrowing: boolean;
	delay: number;
	cooldown = 0;

	constructor(
		board: Board,
		{
			initialSize = 0,
			minSize = 0.8,
			maxSize = 1,
			cX,
			cY,
			delay = 0,
			isGrowing = true,
			getChar = (offset) => colorify(charBright(offset), COLORS.love),
		}: HeartConfig,
	) {
		this.board = board;
		this.cX = cX ?? board.centerX;
		this.cY = cY ?? board.centerY;
		this.size = initialSize;
		this.isGrowing = isGrowing;
		this.maxSize = maxSize;
		this.minSize = minSize;
		this.getChar = getChar;
		this.delay = delay;
	}

	update() {
		const isUpdating = this.cooldown >= this.delay;
		if (!isUpdating) {
			this.cooldown += 1;
			return;
		}
		this.cooldown = 0;
		if (this.cooldown >= this.delay) {
			this.cooldown = 0;
		}
		this.size += 0.05 * (this.isGrowing ? 1 : -1);
		if (this.size >= this.maxSize) {
			this.isGrowing = false;
		}
		if (this.size <= this.minSize) {
			this.isGrowing = true;
		}
	}

	draw() {
		for (let t = -3; t <= 3; t += 0.01) {
			const x = 16 * sin(t) ** 3;
			const y =
				-1 * (13 * cos(t) - 5 * cos(2 * t) - 2 * cos(3 * t) - cos(4 * t));
			for (let i = 0; i <= this.size; i += 0.01) {
				const char = this.getChar(i / (this.size + 0.01));
				this.board.addPending(
					floor(x * i * 2) + this.cX,
					floor(y * i) + this.cY,
					char,
				);
				this.board.addPending(
					floor(x * i * 2) + this.cX - 1,
					floor(y * i) + this.cY,
					char,
				);
			}
		}
	}
}
