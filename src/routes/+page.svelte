<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	const readFile = async () => {
		try {
			let res: string = await invoke('read_file', { path: '/tmp/bargain/messages_123.json' });
			res = JSON.parse(res);
			return res;
		} catch (error) {
			console.error('error: ', error);
		}
	};

	import type { PageData } from './$types';
	import { goto, invalidate } from '$app/navigation';
	export let data: PageData;

	async function onCreateChat(event: Event) {
		const form = document.getElementById('form-create-chat') as HTMLFormElement;

		const formData = new FormData(form);

		const convertedFormEntries = Array.from(formData, ([key, value]) => [
			key,
			typeof value === 'string' ? value : value.name
		]);
		const searchParams = new URLSearchParams(convertedFormEntries);

		const r = await fetch('/api/chat/?' + searchParams.toString(), {
			method: 'POST'
		});

		if (r.ok) {
			const data = await r.json();
			await goto('/chat/' + data);
			await invalidate('/api/chat/');
		}
	}
</script>

<div class="flex flex-col items-center justify-center pt-5">
	<h1 class="pb-2 text-3xl font-bold">Say Hi to BargainGPT</h1>
</div>
<h1 class="pb-5 pt-2 text-center text-xl font-light">
	An easy way to chat with LLaMA based models.
</h1>

<form
	on:submit|preventDefault={onCreateChat}
	id="form-create-chat"
	class="p-5"
	aria-label="Model Settings"
>
	<div class="w-full pb-20">
		<div class="mx-auto w-fit pt-5 flex flex-col lg:flex-row justify-center">
			<button type="submit" class="btn-primary btn mb-2 lg:mr-10 lg:mb-0">Start a new chat</button>
		</div>
	</div>
</form>
