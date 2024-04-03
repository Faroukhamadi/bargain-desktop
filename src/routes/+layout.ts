import { invoke } from '@tauri-apps/api/tauri';

export const prerender = true;
export const ssr = false;

export const load = async () => {
	const chatsString: string = await invoke('get_chats');
	const chats: Chat[] = JSON.parse(chatsString);

	return {
		chats
	};
};
