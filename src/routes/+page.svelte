<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { PageData } from './$types';
	import { goto, invalidate } from '$app/navigation';

	export let data: PageData;

	let init_prompt =
		'Below is an instruction that describes a task. Write a response that appropriately completes the request.';

	let temp = 0.1;

	const readFile = async () => {
		try {
			let res: string = await invoke('read_file', { path: '/tmp/bargain/messages_123.json' });
			res = JSON.parse(res);
			return res;
		} catch (error) {
			console.error('error: ', error);
		}
	};

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
	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<details class="bg-base-200 collapse max-w-xl mx-auto" tabindex="0" id="model_settings">
		<summary class="text-xl font-medium collapse-title">Model settings</summary>
		<div class="collapse-content flex flex-col items-center">
			<div
				class="tooltip tooltip-bottom col-span-1"
				data-tip="Controls how random the generated text is. Higher temperatures lead to more random and creative text, while lower temperatures lead to more predictable and conservative text."
			>
				<label for="temperature" class="label-text">Temperature - [{temp}]</label>
				<input
					id="temperature"
					name="temperature"
					type="range"
					bind:value={temp}
					min="0.05"
					max="2"
					step="0.05"
					class="range range-sm mt-auto"
				/>
			</div>
			<div class="col-span-3 flex flex-col">
				<label for="init_prompt" class="label-text pb-1">Prompt Template</label>
				<textarea
					class="textarea-bordered textarea h-24 w-full"
					name="init_prompt"
					bind:value={init_prompt}
					placeholder="Enter your prompt here"
				/>
			</div>
		</div>
	</details>
	<div class="w-full pb-20">
		<div class="mx-auto w-fit pt-5 flex flex-col lg:flex-row justify-center">
			<button type="submit" class="btn-primary btn mb-2 lg:mr-10 lg:mb-0">Submit</button>
		</div>
	</div>
</form>
