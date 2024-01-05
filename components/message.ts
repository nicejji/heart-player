import type Board from "../board";
import { padCenter } from "../utils";

export default function drawMessage(board: Board, lines: string[][]) {
	const longest = Math.max(...lines.map((l) => l.length));
	const padded = lines.map((l) => padCenter(l, longest));
	const horizontal = Array(longest + 2).fill("─");
	const x = board.centerX - Math.floor((longest + 4) / 2);
	const y = 3;
	board.putChars(x, y, ["╭", ...horizontal, "╮"]);
	for (const [i, line] of padded.entries()) {
		board.putChars(x, y + i + 1, ["│", " ", ...line, " ", "│"]);
	}
	board.putChars(x, y + lines.length + 1, ["╰", ...horizontal, "╯"]);
}
