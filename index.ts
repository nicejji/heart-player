import Board from "./board";
import Heart from "./components/heart";
import Rain from "./components/rain";
import Songs from "./components/songs";

const board = new Board();
board.enter();

const FPS = 60;

const heart = new Heart(board, { delay: 7 });
const rain = new Rain(board, {
	maxDelay: 7,
	probability: 0.3,
});
const songs = new Songs(board);

setInterval(() => {
	board.fillRect({});
	rain.update();
	heart.update();
	songs.update();

	rain.draw();
	heart.draw();
	songs.draw();

	board.flush();
}, 1000 / FPS);
