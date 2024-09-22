// Checks if a user was setup, if not, redirect to the setup page.
export default defineNuxtRouteMiddleware(async (to, from) => {
	const api_url = useRuntimeConfig().public.api_url;

	// Request the user count
	const url = `${api_url}/api/stats/user_count`;
	const options = {
		method: "GET",
	};
	const request = await apiRequest(url, options);
	// console.log(request);

	// If there is an error during the request, do nothing.
	if (
		request.is_request_error ||
		request.is_response_error ||
		request.data.is_error
	) {
		console.error("Could not parse the reqest.");
		return;
	}

	// If there are no registered users, redirect to the setup page.
	// Otherwiese redirect the setup page to the main route
	if (request.data.data.count <= 0) {
		if (to.path !== "/setup") {
			return navigateTo("/setup");
		}
	} else {
		if (to.path === "/setup") {
			if (from.path === "/setup") {
				return navigateTo("/");
				// biome-ignore lint/style/noUselessElse: <explanation>
			} else {
				return navigateTo(from.path);
			}
		}
	}
});
