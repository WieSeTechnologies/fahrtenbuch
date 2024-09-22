// TODO: Make this more informative regaeding errors (like api request)
export default async function (session) {
	// Request a new session
	const api_url = useRuntimeConfig().public.api_url;
	const url = `${api_url}/api/user/verify_session`;
	const options = {
		method: "POST",
		headers: { "content-type": "application/json" },
		body: JSON.stringify(session),
	};
	const request = await apiRequest(url, options);
	// console.log(request);

	// If there is any error, redirect to the login page
	if (
		request.is_request_error ||
		request.is_response_error ||
		request.data.is_error
	) {
		console.error("Could not parse the reqest.");
		return false;
	}

	// Redirect to login page if the session is invalid
	// TODO: Delete the Cookie if it is invalid
	if (request.data.data !== true) {
		console.log("Session is invalid.");
		return false;
	}

	return true;
}
