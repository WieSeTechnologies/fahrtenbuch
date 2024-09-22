export default async function (url, options) {
	let data = null;
	try {
		const response = await fetch(url, options);
		data = await response.json();
		if (!response.ok || !response) {
			console.error(data);
			return { is_error: true, data: data };
		}
	} catch (error) {
		console.error(error);
		return { is_error: true, data: null };
	}

	return {
		is_error: false,
		data: data,
	};
}
