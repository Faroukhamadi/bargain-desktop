export const prerender = true;
export const ssr = false;

export const load = async () => {
	const chats = [
		{
			id: 1,
			subtitle: 'chat 1',
			model: 'model 1',
			created: '2024-03-29T12:00:00Z'
		},
		{
			id: 2,
			subtitle: 'chat 2',
			model: 'model 2',
			created: '2024-03-29T12:00:00Z'
		},
		{
			id: 3,
			subtitle: 'chat 3',
			model: 'model 3',
			created: '2024-03-29T12:00:00Z'
		}
	];

	return {
		chats
	};
};

// const chats = (await api_chat.json()) as ChatMetadata[];
// const model_api = await fetch('/api/model/all');
// const models = (await model_api.json()) as ModelStatus[];
// return {
// 	chats,
// 	models
// };
