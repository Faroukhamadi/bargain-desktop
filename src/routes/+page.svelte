<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	const readFile = async () => {
		try {
			let res = await invoke('read_file', { path: '/tmp/bargain/messages_123.json' });
			res = JSON.parse(res);
			return res;
		} catch (error) {
			console.error('error: ', error);
		}
	};
</script>

<button on:click={readFile}>Read file</button>
