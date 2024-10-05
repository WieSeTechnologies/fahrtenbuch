// TODO: Make this more informative regaeding errors (like api request)
export default async function () {
	// Get the session cookie
	let session_cookie;
	try {
		session_cookie = useCookie("session");
	} catch (_) {
		console.error("no session cookie");
		return false;
	}

	// If the session cookie is broken, delete it and return false
	if (
		!session_cookie ||
		!session_cookie.value ||
		!session_cookie.value.username ||
		!session_cookie.value.session_id
	) {
		session_cookie.value = null;
		return false;
	}

	// Create the session object using the session data from the cookie
	const session = {
		username: session_cookie.value.username,
		session_id: session_cookie.value.session_id,
	};

	// Verify the session with the backend
	const api_url = useRuntimeConfig().public.api_url;
	const url = `${api_url}/api/user/verify_session`;
	const options = {
		method: "POST",
		headers: { "content-type": "application/json" },
		body: JSON.stringify(session),
	};
	const request = await apiRequest(url, options);
	// If there is any error during the request return false
	if (
		request.is_request_error ||
		request.is_response_error ||
		request.data.is_error
	) {
		console.error("Could not parse the reqest.");
		return false;
	}

	// Return false if the session is invalid
	if (request.data.data !== true) {
		session_cookie.value = null;
		return false;
	}

	return true;
}
