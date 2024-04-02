import { invoke } from '@tauri-apps/api/tauri';

export const prerender = true;
export const ssr = false;

export const load = async () => {
	const chatsString: string = await invoke('get_messages');
	const chats: Chat[] = JSON.parse(chatsString);

	console.log('chats: ', chats);

	return {
		chats
	};
};
