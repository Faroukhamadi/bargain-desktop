// define a chat type

interface Message {
	from: string;
	text: string;
}

interface Chat {
	id: string;
	title: string;
	created_at: string;
	messages: Message[];
}
