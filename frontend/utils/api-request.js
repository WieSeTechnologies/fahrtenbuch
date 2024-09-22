export default async function (url, options) {
	let data = null;
	try {
		const response = await fetch(url, options);
		if (!response) {
			console.error("Request Error.");
			return { is_request_error: true, is_response_error: false, data: null };
		}

		data = await response.json();

		if (!response.ok) {
			console.error("Response Error.");
			return { is_request_error: false, is_response_error: true, data: data };
		}
	} catch (error) {
		console.error(error);
		return { is_request_error: true, is_response_error: false, data: null };
	}

	return {
		is_request_error: false,
		is_response_error: false,
		data: data,
	};
}
