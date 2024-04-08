export const prerender = true;
export const ssr = false;

export const load = async ({ fetch }) => {
	const chats = await fetch('http://localhost:8000/api');

	return {
		chats: (await chats.json()) as Chat[]
	};
};
