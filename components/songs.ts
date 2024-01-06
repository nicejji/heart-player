import fs from "fs";
import drawMessage from "./message";
import Lyrics from "./lyrics";
import type Board from "../board";
import { COLORS, colorify, italic } from "../utils";

const formatTime = (ms: number) => {
	const mins = `${~~(ms / 60000)}`.padStart(2, "0");
	const secs = `${~~((ms % 60000) / 1000)}`.padStart(2, "0");
	return `${mins}:${secs}`;
};

type Track = {
	lyrics: string;
	song: string;
};
const findTracks = () => {
	const files = fs.readdirSync(".");
	const songs = files.filter((f) => f.endsWith(".mp3"));
	return songs.map((song) => ({
		song,
		lyrics: song.replace(".mp3", ".json"),
	})) as Track[];
};

export default class Songs {
	menuOpen = false;
	selected = 0;
	board: Board;
	tracks: Track[];
	chunkSize: number;
	lyrics: Lyrics | null = null;
	songDuration = 1;

	constructor(board: Board, chunkSize = 4) {
		this.board = board;
		this.tracks = findTracks();
		this.chunkSize = chunkSize;
		setupCallbacks(this);
	}

	get playing() {
		return this.lyrics?.playing ?? false;
	}

	play() {
		if (!this.menuOpen) return;
		const { board, tracks, selected, chunkSize } = this;
		const { song, lyrics } = tracks[selected];
		this?.lyrics?.pause();
		this.lyrics = new Lyrics(board, song, chunkSize);
		this.lyrics.loadFromSpotifyJson(lyrics);
		this.lyrics.play();
		getSongDuration(song).then((duration) => {
			this.songDuration = duration;
		});
	}

	selectNext() {
		if (!this.menuOpen) return;
		if (this.selected < this.tracks.length - 1) {
			this.selected += 1;
		} else {
			this.selected = 0;
		}
	}

	selectPrev() {
		if (!this.menuOpen) return;
		if (this.selected > 0) {
			this.selected -= 1;
		} else {
			this.selected = this.tracks.length - 1;
		}
	}

	update() {
		this?.lyrics?.update();
	}

	draw() {
		// draw lyrics
		if (this.lyrics) {
			this.lyrics.draw();
		}
		// draw status
		//
		if (!this.menuOpen) {
			const { board, lyrics } = this;
			const barWidth = 40;
			const percentPlayed = (this.lyrics?.playedTime ?? 1) / this.songDuration;
			const bar = Array.from({ length: barWidth }, (_, i) => {
				const passed = i <= barWidth * percentPlayed;
				return colorify(
					passed ? "━" : "─",
					passed ? COLORS.foam : COLORS.highlight_high,
				);
			});
			const statusLines = lyrics
				? [
						[...lyrics.songPath.replace(".mp3", "").split("").map(italic)],
						` ${lyrics.playing ? "▶" : "⏸"} ${formatTime(lyrics.playedTime)} `
							.split("")
							.map((ch) =>
								colorify(ch, lyrics.playing ? COLORS.iris : COLORS.gold),
							),
						bar,
				  ]
				: ["Nothing playing now :)".split("")];
			drawMessage(board, statusLines, 1);
		}
		// draw menu
		if (!this.menuOpen) return;
		const lines = this.tracks.map((t, i) => {
			const line = ` ${t.song.replace(".mp3", "")} `.split("");
			return i === this.selected
				? line.map((ch) =>
						colorify(colorify(ch, COLORS.text, true), COLORS.base),
				  )
				: line;
		});
		drawMessage(this.board, lines, 1);
	}
}

const setupCallbacks = (songs: Songs) => {
	process.stdin.on("data", (data) => {
		const key = data.toString();
		switch (key) {
			case "m":
				songs.menuOpen = !songs.menuOpen;
				break;

			case "j":
				songs.selectNext();
				break;
			case "k":
				songs.selectPrev();
				break;
			case "\r":
				songs.play();
				songs.menuOpen = false;
				break;
			case " ":
				songs?.lyrics?.playing ? songs?.lyrics?.pause() : songs?.lyrics?.play();
				break;
			case "l":
				songs?.lyrics?.seek(1);
				break;
			case "h":
				songs?.lyrics?.seek(-1);
				break;
		}
	});
};

const getSongDuration = (path: string): Promise<number> =>
	new Promise((res, rej) => {
		Bun.spawn(["soxi", "-D", path], {
			onExit(p) {
				const { stdout } = p;
				if (stdout && typeof stdout !== "number") {
					new Response(stdout)
						.text()
						.then((text) => {
							res(Number(text.trim()) * 1000);
						})
						.catch(() => rej());
				} else {
					rej();
				}
			},
		});
	});
