<script lang="ts">
	import type { PageData } from './$types';
	import { invalidate, invalidateAll } from '$app/navigation';
	import { page } from '$app/stores';
	import { newChat, themeStore } from '$lib/stores';
	import { onMount, onDestroy } from 'svelte';
	import ClipboardJS from 'clipboard';
	import hljs from 'highlight.js';
	import 'highlight.js/styles/github-dark.css';
	import bash from 'highlight.js/lib/languages/bash';
	import css from 'highlight.js/lib/languages/css';
	import cpp from 'highlight.js/lib/languages/cpp';
	import dockerfile from 'highlight.js/lib/languages/dockerfile';
	import go from 'highlight.js/lib/languages/go';
	import javascript from 'highlight.js/lib/languages/javascript';
	import json from 'highlight.js/lib/languages/json';
	import ini from 'highlight.js/lib/languages/ini';
	import nginx from 'highlight.js/lib/languages/nginx';
	import plaintext from 'highlight.js/lib/languages/plaintext';
	import powershell from 'highlight.js/lib/languages/powershell';
	import python from 'highlight.js/lib/languages/python';
	import rust from 'highlight.js/lib/languages/rust';
	import swift from 'highlight.js/lib/languages/swift';
	import sql from 'highlight.js/lib/languages/sql';
	import typescript from 'highlight.js/lib/languages/typescript';
	import html from 'highlight.js/lib/languages/xml';
	import yaml from 'highlight.js/lib/languages/yaml';
	import MarkdownIt from 'markdown-it';
	import mdHighlight from 'markdown-it-highlightjs';

	hljs.registerLanguage('bash', bash);
	hljs.registerLanguage('css', css);
	hljs.registerLanguage('cpp', cpp);
	hljs.registerLanguage('dockerfile', dockerfile);
	hljs.registerLanguage('go', go);
	hljs.registerLanguage('javascript', javascript);
	hljs.registerLanguage('json', json);
	hljs.registerLanguage('ini', ini);
	hljs.registerLanguage('nginx', nginx);
	hljs.registerLanguage('plaintext', plaintext);
	hljs.registerLanguage('powershell', powershell);
	hljs.registerLanguage('python', python);
	hljs.registerLanguage('rust', rust);
	hljs.registerLanguage('sql', sql);
	hljs.registerLanguage('swift', swift);
	hljs.registerLanguage('typescript', typescript);
	hljs.registerLanguage('xml', html);
	hljs.registerLanguage('yaml', yaml);

	export let data: PageData;
	let messageContainer: any;
	let theme: string;
	let styleElement: HTMLLinkElement;
	const isLoading = false;
	$: startDate = new Date(data.chat.created);
	$: history = data.chat.history;
	$: newChat.set(data.chat);
	$: if (messageContainer) {
		messageContainer.scrollBottom = messageContainer.scrollHeight;
	}
	let prompt = '';

	async function askQuestion() {
		const data = new URLSearchParams();

		if (!prompt || prompt === '') {
			prompt = 'Reformulate your last answer.';
		}

		data.append('prompt', prompt);

		const eventSource = new EventSource(
			'http://localhost:8000/api/conversation/' + $page.params.id + '/question?' + data.toString()
		);

		history = [
			...history,
			// {
			// 	type: 'human',
			// 	data: {
			// 		content: prompt
			// 	}
			// },
			{
				type: 'ai',
				data: {
					content: ''
				}
			}
		];

		prompt = '';

		eventSource.addEventListener('message', (event) => {
			history[history.length - 1].data.content += event.data;
		});

		eventSource.addEventListener('close', async () => {
			eventSource.close();
			await invalidate('http://localhost:8000/api/conversation/' + $page.params.id);
			prompt = '';
		});

		eventSource.onerror = async (event) => {
			eventSource.close();
		};
	}

	async function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			await askQuestion();
		}
	}

	async function deletePrompt(chatID: string, idx: number) {
		const response = await fetch(`/api/chat/${chatID}/prompt?idx=${idx.toString()}`, {
			method: 'DELETE'
		});

		if (response.status === 200) {
			await invalidate('/api/chat/' + $page.params.id);
		} else if (response.status === 202) {
			showToast('Chat in progress!');
		} else {
			showToast('An error occurred: ' + response.statusText);
		}
	}

	function showToast(message: string) {
		// Create the toast element
		const toast = document.createElement('div');
		toast.className = `alert alert-info`;
		toast.textContent = message;
		const toastContainer = document.getElementById('toast-container');

		// Append the toast to the toast container if it exists
		if (toastContainer) {
			toastContainer.appendChild(toast);
		} else {
			console.error('Toast container not found?');
			return;
		}

		// Automatically remove the toast after a delay
		setTimeout(() => {
			toast.remove();
		}, 3000);
	}

	const md: MarkdownIt = new MarkdownIt({
		html: true,
		linkify: true,
		typographer: true,
		breaks: true,
		highlight: (code_string: string, lang: string) => {
			if (lang && hljs.getLanguage(lang)) {
				try {
					const code = hljs.highlight(code_string, lang).value;
					return hljs.highlight(code, { language: lang }).value;
				} catch (ex) {
					/**/
				}
			}
			return '';
		}
	}).use(mdHighlight, { hljs });

	const originalFenceRenderer = md.renderer.rules.fence;

	md.renderer.rules.fence = (tokens: any, index: any, options: any, env: any, self: any) => {
		// Increment the codeblock id
		const id = `code-block-${Math.random().toString(36).substring(7)}`;
		// Generate original fenced code block HTML
		if (!originalFenceRenderer) throw new Error('originalFenceRenderer is undefined');
		// Create copy button HTML
		const copyButton = `<div class="tooltip tooltip-top absolute top-0.5 right-1" data-tip="copy"><button class="btn btn-xs btn-square btn-ghost copy-button" data-clipboard-target="#${id}"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="12" height="12"><path class="fill-base-content" d="M0 6.75C0 5.784.784 5 1.75 5h1.5a.75.75 0 0 1 0 1.5h-1.5a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-1.5a.75.75 0 0 1 1.5 0v1.5A1.75 1.75 0 0 1 9.25 16h-7.5A1.75 1.75 0 0 1 0 14.25Z"></path><path class="fill-base-content" d="M5 1.75C5 .784 5.784 0 6.75 0h7.5C15.216 0 16 .784 16 1.75v7.5A1.75 1.75 0 0 1 14.25 11h-7.5A1.75 1.75 0 0 1 5 9.25Zm1.75-.25a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25Z"></path></svg></button></div>`;

		const codeBlock = originalFenceRenderer(tokens, index, options, env, self)
			.replace(`<code `, `<code id="${id}"`)
			.replace(`<pre>`, `<pre class="relative max-w-7xl mx-auto">${copyButton}`);

		// Get the token and code content
		const token = tokens[index];
		const content = token.content;

		// Wrap the fenced code block and copy button in a container
		const html = `
      ${codeBlock}
    `;

		return html;
	};

	const renderMarkdown = (q: any) => {
		return md.render(q);
	};

	onMount(() => {
		styleElement = document.createElement('link');
		styleElement.rel = 'stylesheet';
		const cbJS = new ClipboardJS('.copy-button');
		newChat.set(data.chat);
		updateThemeStyle($themeStore);
		themeStore.subscribe((newTheme) => {
			updateThemeStyle(newTheme);
		});
	});

	function updateThemeStyle(currentTheme: string) {
		if (currentTheme === 'dark') {
			styleElement.href = '/css/github-dark.css';
		} else {
			styleElement.href = '/css/github.css';
		}
		document.head.appendChild(styleElement);
	}

	let sendBottomHovered = false;
	const onMouseEnter = () => {
		sendBottomHovered = true;
	};
	const onMouseLeave = () => {
		sendBottomHovered = false;
	};
	const scrollToBottom = (node: Element, history: any[]) => {
		const scroll = () =>
			node.scroll({
				top: node.scrollHeight,
				behavior: 'smooth'
			});
		scroll();

		return { update: scroll };
	};

	const ACTIONS = [
		{ label: 'greet', value: 'Please say hello and start the conversation.' },
		{ label: 'inquire', value: 'Please ask any question about product, year, price, usage, etc.' },
		{ label: 'inform', value: 'Please provide information about the product, year, usage, etc.' },
		{ label: 'propose', value: 'Please initiate a price or a price range for the product.' },
		{ label: 'counter', value: 'Please propose a new price or a new price range.' },
		{
			label: 'counter-noprice',
			value: 'Please propose a vague price by using comparatives with existing price.'
		},
		{ label: 'confirm', value: 'Please ask a question about the information to be confirmed.' },
		{ label: 'affirm', value: 'Please give an affirmative response to a confirm.' },
		{ label: 'deny', value: 'Please give a negative response to a confirm.' },
		{ label: 'agree', value: 'Please agree with the proposed price.' },
		{ label: 'disagree', value: 'Please disagree with the proposed price.' },
		{
			label: 'improvise',
			value: 'Please keep the negotiation going by trying to get to the goal price'
		}
	];

	onDestroy(() => {
		styleElement && styleElement.remove();
	});
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="relative h-full max-h-screen overflow-hidden" on:keydown={handleKeyDown}>
	<div
		class="mb-11 h-[calc(98vh-10rem)] 2xl:h-[calc(105vh-12rem)] overflow-y-auto"
		use:scrollToBottom={history}
	>
		<div class="h-max py-4">
			<select
				class="select select-bordered w-full max-w-xs"
				on:change={async (e) => {
					const product = e.target?.value;
					if (product) {
						const url = `http://localhost:8000/api/${$page.params.id}/products?product=${product}`;
						await fetch(url, {
							method: 'POST'
						});
						await invalidateAll();
					}
				}}
			>
				<option selected>Products</option>
				{#each data.products as product}
					<option selected={product === data.product} value={product}>{product}</option>
				{/each}
			</select>
			{#each history as question, i}
				{#if question.type === 'ai'}
					<div class="w-10/12 mx-auto sm:w-10/12 chat chat-end py-4">
						<div class="chat-image self-start pl-1 pt-1">
							<div
								class="mask mask-squircle online flex aspect-square w-8 items-center justify-center overflow-hidden bg-gradient-to-b from-primary to-primary-focus"
							>
								<span class="text-xs text-neutral-content">GPT</span>
							</div>
						</div>
						<div class="chat-header">Prompt</div>
						<div
							class="chat-bubble whitespace-normal break-words bg-base-300 text-base font-light text-base-content"
						>
							<!-- {question.data.content} -->
							<div class="w-full overflow-hidden break-words">
								{@html renderMarkdown(question.data.content)}
							</div>
						</div>
					</div>
				{:else if question.type === 'human'}
					<div class="w-10/12 mx-auto sm:w-10/12 chat chat-start py-4">
						<div class="chat-image self-start pl-1 pt-1">
							<div
								class="mask mask-squircle online flex aspect-square w-8 items-center justify-center overflow-hidden bg-gradient-to-b from-primary to-primary-focus"
							>
								<span class="text-xs text-neutral-content">Seller</span>
							</div>
						</div>
						<div
							class="chat-bubble whitespace-normal bg-base-200 text-base font-light text-base-content w-full"
						>
							{#if question.data.content === ''}
								<div class="inline-block rounded bg-base-200 px-4 py-1">
									<div class="dots-load aspect-square w-2 rounded-full" />
								</div>
							{/if}
							<!-- {question.data.content} -->
							<div class="markdown">
								{@html renderMarkdown(question.data.content)}
							</div>
						</div>
					</div>
				{:else if question.type === 'system'}
					<div class="text-md w-full px-10 pt-2 pb-4 text-center font-light md:px-16">
						<h4 class="badge badge-ghost text-center text-xs font-semibold">
							{startDate.toLocaleString('en-US')}
						</h4>
						<br />
						{question.data.content}
					</div>
				{/if}
			{/each}
		</div>
	</div>
	<div
		class="flex h-0 w-full flex-row items-center justify-center bg-gradient-to-t from-indigo-500 px-0"
	>
		<div
			class="input-bordered input flex h-auto w-full max-w-3xl flex-row items-center justify-between rounded-lg bg-base-200 px-0 drop-shadow-md"
		>
			<select
				class="select select-bordered w-1/4 max-w-xs"
				on:change={async (e) => {
					const action = e.target?.value;
					if (action) {
						prompt = ACTIONS.find((a) => a.label === action)?.value;
						console.log('prompt: ', prompt);
					}
				}}
			>
				<option selected>Action</option>
				{#each ACTIONS as action}
					<option selected={action === data.action} value={action.label}>{action.label}</option>
				{/each}
			</select>
			<!-- <textarea
				name="question"
				class="textarea h-10 flex-1 resize-y bg-[transparent] text-lg placeholder-base-content outline-0 ring-0 focus:outline-0"
				disabled={isLoading}
				placeholder="Message BargainGPT..."
				bind:value={prompt}
			/> -->
			<button
				on:mouseenter={onMouseEnter}
				on:mouseleave={onMouseLeave}
				type="submit"
				disabled={isLoading}
				class="btn btn-ghost h-10 w-14 rounded-l-none rounded-r-lg border-0 text-lg"
				class:loading={isLoading}
				on:click|preventDefault={askQuestion}
				><span class="sr-only">Send</span>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="16" height="16">
					<path
						class:fill-primary-content={sendBottomHovered}
						class:fill-base-content={!sendBottomHovered}
						d="M.989 8 .064 2.68a1.342 1.342 0 0 1 1.85-1.462l13.402 5.744a1.13 1.13 0 0 1 0 2.076L1.913 14.782a1.343 1.343 0 0 1-1.85-1.463L.99 8Zm.603-5.288L2.38 7.25h4.87a.75.75 0 0 1 0 1.5H2.38l-.788 4.538L13.929 8Z"
					/>
				</svg>
			</button>
		</div>
	</div>
	<div id="toast-container" class="toast">
		<!-- Toast notifications will be added here -->
	</div>
</div>
