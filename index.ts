import Board from "./board";
import Heart from "./components/heart";
import Rain from "./components/rain";
import Lyrics from "./components/lyrics";
import { COLORS, colorify, italic, playSong } from "./utils";
import drawMessage from "./components/message";

const SONG = process.argv[2];
const LYRICS = process.argv[3];

const formatTime = (ms: number) => {
	const mins = `${Math.floor(ms / 60000)}`.padStart(2, "0");
	const secs = `${Math.floor((ms % 60000) / 1000)}`.padStart(2, "0");
	return `${mins}:${secs}`;
};

const board = new Board();
board.enter();

const heart = new Heart(board, { delay: 5 });
const rain = new Rain(board, {});
const lyrics = new Lyrics(board, 4);

await lyrics.loadFromSpotifyJson(LYRICS);
lyrics.play();

playSong(SONG);
setInterval(() => {
	board.fillRect({});
	rain.update();
	heart.update();

	rain.draw();
	heart.draw();
	lyrics.draw();
	drawMessage(board, [
		[],
		[..."Playing: ", ...SONG.split("").map(italic)],
		`[ ${formatTime(Date.now() - lyrics.startetAt)} ]`
			.split("")
			.map((ch) => colorify(ch, COLORS.gold)),
		[],
	]);

	board.flush();
}, 20);
