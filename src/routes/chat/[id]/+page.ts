import { invoke } from '@tauri-apps/api/tauri';

export const load = async ({ params }) => {
	const chatString: string = await invoke('get_message', { id: params.id });
	const chat: Chat = JSON.parse(chatString);

	return {
		chat
	};
};
