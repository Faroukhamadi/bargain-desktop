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
					content: 'Hello there nice to meet you in this chat it is a pleasure to meet you'
				}
			},
			{
				type: 'ai',
				data: {
					content:
						'Hi there, I am a chatbot, how can I help you today? I am here to help you with any questions you may have.'
				}
			},
			{
				type: 'human',
				data: {
					content: 'I am looking for a new car, can you help me find one?'
				}
			},
			{
				type: 'ai',
				data: {
					content: 'Sure, I can help you with that. What kind of car are you looking for?'
				}
			}
		]
	};

	return {
		chat
	};
};
