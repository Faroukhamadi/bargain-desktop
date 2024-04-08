export const load = async ({ fetch, params }) => {
	const data = await fetch('http://localhost:8000/api/' + params.id);
	const chat = (await data.json()) as ChatResponse;
	console.log('chat:', chat);

	return {
		chat
	};
};
