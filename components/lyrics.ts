import type Board from "../board";

type Line = {
	startTimeMs: number;
	words: string;
};
type SpotifyLyrics = {
	error: boolean;
	syncType: string;
	lines: { startTimeMs: string; words: string }[];
};

export default class Lyrics {
	board: Board;
	lines: Line[] = [];
	startetAt = 0;
	playing = false;
	chunkSize = 4;

	constructor(board: Board, chunkSize: number) {
		this.board = board;
		this.chunkSize = chunkSize;
	}

	play() {
		const { playing, lines } = this;
		if (playing) return;

		this.playing = true;
		this.startetAt = Date.now();

		const last = lines.at(-1) as Line;
		setTimeout(() => {
			this.playing = false;
		}, last.startTimeMs);
	}

	draw() {
		if (!this.playing) return;
		const { startetAt, lines, chunkSize, board } = this;

		const lastLine = lines.findLastIndex(
			({ startTimeMs }) => Date.now() > startTimeMs + startetAt,
		);
		const chunk = lines
			.slice(lastLine - (lastLine % chunkSize), lastLine + 1)
			.map((l) => l.words);

		for (const [i, line] of chunk.entries()) {
			const x = board.centerX - Math.floor(line.length / 2);
			const y = board.centerY - Math.floor(chunk.length / 2) + i;
			board.putChars(x, y, ` ${line.trim()} `.split(""));
		}
	}

	async loadFromSpotifyJson(path: string) {
		const data = (await Bun.file(path).json()) as SpotifyLyrics;
		this.lines = data.lines.map(({ words, startTimeMs }) => ({
			words,
			startTimeMs: parseInt(startTimeMs),
		}));
	}
}
