interface Message {
	from: string;
	text: string;
}

interface Chat {
	id: string;
	title: string;
	subtitle: string;
	created: string;
	messages: Message[];
}
