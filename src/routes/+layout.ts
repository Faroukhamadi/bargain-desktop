export const prerender = true;
export const ssr = false;

export const load = async ({ fetch }) => {
	const data = await fetch('http://localhost:8000/api');
	const chats = (await data.json()) as Chat[];
	console.log('chats', chats);

	return {
		chats
	};
};
