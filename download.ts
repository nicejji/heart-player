const { CLIENT_ID = "", CLIENT_SECRET = "" } = process.env;

const TOKEN_URL = "https://accounts.spotify.com/api/token";
const LYRICS_URL = "https://spotify-lyric-api-984e7b4face0.herokuapp.com";

const SEARCH_URL = "https://api.spotify.com/v1/search";

type TokenResponse = {
	access_token: string;
	token_type: string;
	expires_in: number;
};

type SearchResponse = {
	tracks: {
		items: {
			id: string;
			name: string;
			artists: {
				name: string;
			}[];
		}[];
	};
};

const getToken = async (client_id: string, client_secret: string) => {
	const url = new URL(TOKEN_URL);
	url.searchParams.set("grant_type", "client_credentials");
	url.searchParams.set("client_id", client_id);
	url.searchParams.set("client_secret", client_secret);
	const res = await fetch(url, {
		method: "POST",
		headers: {
			"Content-Type": "application/x-www-form-urlencoded",
		},
	});
	const data = await res.json();
	return (data as TokenResponse).access_token;
};

const searchTrack = async (title: string, token: string) => {
	const url = new URL(SEARCH_URL);
	url.searchParams.set("limit", "1");
	url.searchParams.set("type", "track");
	url.searchParams.set("q", `${title}`);
	const res = await fetch(url, {
		headers: {
			Authorization: `Bearer ${token}`,
		},
	});
	const data = await res.json();
	return (data as SearchResponse).tracks.items[0];
};

const getLyrics = async (track_id: string) => {
	const url = new URL(LYRICS_URL);
	url.searchParams.set("trackid", track_id);
	const res = await fetch(url);
	const data = await res.json();
	return data;
};

const downloadTrack = (query: string, name: string) => {
	Bun.spawn([
		"yt-dlp",
		"-f",
		"ba",
		"-x",
		"--audio-format",
		"mp3",
		"--default-search",
		"auto",
		query,
		"-o",
		`${name}.mp3`,
	]);
};

const query = process.argv[2];

const token = await getToken(CLIENT_ID, CLIENT_SECRET);
const { name, id, artists } = await searchTrack(query, token);

console.log("[LOG] Track found!");
console.log(
	`Title: ${artists.map((a) => a.name).join(",")} - ${name} | id: ${id}`,
);

const lyrics = await getLyrics(id);
console.log("[LOG] Lyrics downloaded!");

const filename = `./${artists.map((a) => a.name).join(",")} - ${name}`;
Bun.write(`${filename}.json`, JSON.stringify(lyrics));
console.log("[LOG] Lyrics writed!");
downloadTrack(`${artists.map((a) => a.name).join(",")} - ${name}`, filename);
console.log("[LOG] Track writed!");
