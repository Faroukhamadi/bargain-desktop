export const load = async ({ fetch, params }) => {
	const chatData = await fetch('http://localhost:8000/api/' + params.id);
	const chat = (await chatData.json()) as ChatResponse;

	const productData = await fetch('http://localhost:8000/api/products');
	const products = (await productData.json()) as string[];

	return {
		chat,
		products
	};
};
