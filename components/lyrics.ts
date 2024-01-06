import type { Subprocess } from "bun";
import type Board from "../board";
import { playSong } from "../utils";

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
	songPath: string;
	audioProcess: Subprocess | null = null;
	board: Board;
	lines: Line[] = [];
	pausedAt = 0;
	playedTime = 0;
	playing = false;
	chunkSize = 4;

	constructor(board: Board, songPath: string, chunkSize: number) {
		this.board = board;
		this.chunkSize = chunkSize;
		this.songPath = songPath;
	}

	play() {
		if (this.playing) return;
		this.audioProcess = playSong(this.songPath, this.playedTime);

		this.pausedAt = Date.now();
		this.playing = true;
	}

	pause() {
		this.audioProcess?.kill();
		this.playing = false;
	}

	seek(seconds: number) {
		if (this.playedTime < 1000) return;
		this.pause();
		this.playedTime += seconds * 1000;
		this.play();
	}

	update() {
		if (this.playing) {
			this.playedTime += Date.now() - this.pausedAt;
		}
		this.pausedAt = Date.now();
	}

	draw() {
		const { playedTime, lines, chunkSize, board } = this;

		const lastLine = lines.findLastIndex(
			({ startTimeMs }) => playedTime > startTimeMs,
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
