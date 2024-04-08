type MessageType = 'human' | 'ai' | 'system';

interface MessageData {
	content: string;
}

interface Message {
	type: MessageType;
	data: MessageData;
}

interface Chat {
	id: string;
	title: string;
	subtitle: string;
	created: string;
	messages: Message[];
}

interface ChatResponse {
	id: string;
	created: string;
	params: Params;
	history: Message[];
}
