import type Board from "../board";
import Drop from "./drop";

type RainConfig = {
	minSize?: number;
	maxSize?: number;
	maxDelay?: number;
	probability?: number;
};

export default class Rain {
	minSize: number;
	maxSize: number;
	maxDelay: number;
	probability: number;
	board: Board;
	drops: Drop[] = [];

	constructor(
		board: Board,
		{ minSize = 1, maxSize = 10, maxDelay = 5, probability = 0.3 }: RainConfig,
	) {
		this.board = board;
		this.minSize = minSize;
		this.maxSize = maxSize;
		this.maxDelay = maxDelay;
		this.probability = probability;
	}

	#addDrop() {
		this.drops.push(
			new Drop(this.board, {
				size: Math.floor(
					Math.random() * (this.maxSize - this.minSize) + this.minSize,
				),
				x: Math.floor(Math.random() * (this.board.width - 1)),
				delay: Math.floor(Math.random() * this.maxDelay),
			}),
		);
	}
	update() {
		const randPadding = (1 - this.probability) / 2;
		const roll = Math.random();
		if (roll > 1 - randPadding || roll < randPadding) {
			this.#addDrop();
		}
		for (const drop of this.drops) {
			drop.update();
		}
		this.drops = this.drops.filter((d) => d.isValid);
	}

	draw() {
		for (const drop of this.drops) {
			drop.draw();
		}
	}
}
