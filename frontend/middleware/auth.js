// Checks if a user was setup, if not, redirect to the setup page.
export default defineNuxtRouteMiddleware(async (to, from) => {
	// === Get the session form a cookie

	// TODO: Get session from Cookie

	const session = useCookie("session");

	if (session.username === null || session.session_id === null) {
	}

	// === Verification with backend

	// Request a new session
	const api_url = useRuntimeConfig().public.api_url;
	const url = `${api_url}/api/user/verify_session`;
	const options = {
		method: "POST",
		body: JSON.stringify({
			username: "olaf",
			session_id: "UUID",
		}),
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
		return navigateTo("/user/login");
	}

	// Redirect to login page if the session is invalid
	// TODO: Delete the Cookie if it is invalid
	if (request.data.data !== true) {
		console.log("Session is invalid.");
		return navigateTo("/user/login");
	}
});
