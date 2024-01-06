import type Board from "../board";
import Drop from "./drop";

type RainConfig = {
	minSize?: number;
	maxSize?: number;
	maxDelay?: number;
	probability?: number;
	getChar?: (percent: number) => string;
};

export default class Rain {
	minSize: number;
	maxSize: number;
	maxDelay: number;
	probability: number;
	board: Board;
	drops: Drop[] = [];
	getChar?: (percent: number) => string;

	constructor(
		board: Board,
		{
			minSize = 1,
			maxSize = 10,
			maxDelay = 5,
			probability = 0.3,
			getChar,
		}: RainConfig,
	) {
		this.board = board;
		this.minSize = minSize;
		this.maxSize = maxSize;
		this.maxDelay = maxDelay;
		this.probability = probability;
		this.getChar = getChar;
	}

	#addDrop() {
		this.drops.push(
			new Drop(this.board, {
				getChar: this.getChar,
				size: ~~(Math.random() * (this.maxSize - this.minSize) + this.minSize),
				x: ~~(Math.random() * (this.board.width - 1)),
				delay: ~~(Math.random() * this.maxDelay),
			}),
		);
	}
	update() {
		if (Math.random() < this.probability) this.#addDrop();
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
