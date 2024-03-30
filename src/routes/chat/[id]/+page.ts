export const load = async () => {
	const chat = {
		id: 2,
		subtitle: 'chat 2',
		model: 'model 2',
		created: '2024-03-29T12:00:00Z',
		history: [
			{
				type: 'human',
				data: {
					content: 'Hello'
				}
			},
			{
				type: 'ai',
				data: {
					content: 'Hi'
				}
			}
		]
	};

	return {
		chat
	};
};
