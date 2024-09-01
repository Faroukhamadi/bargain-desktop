export const load = async ({ fetch, params }) => {
	try {
		const [chatData, productsData, productData] = await Promise.all([
			fetch(`http://localhost:8000/api/${params.id}`),
			fetch(`http://localhost:8000/api/products`),
			fetch(`http://localhost:8000/api/${params.id}/products`)
		]);

		const [chat, products, product] = await Promise.all([
			chatData.json(),
			productsData.json(),
			productData.json()
		]);

		chat.history.reverse();

		return {
			chat,
			products,
			product
		};
	} catch (error) {
		console.error('Error loading chat:', error);
		return {
			error: 'Error loading chat'
		};
	}
};
