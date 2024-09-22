// Checks if a user was setup, if not, redirect to the setup page.
export default defineNuxtRouteMiddleware(async (to, from) => {
	if (to.path !== "/error") {
		// Request the user count
		const api_url = useRuntimeConfig().public.api_url;
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
			return navigateTo("/error");
		}

		const user_count = request.data.data.count;

		// Less than one User
		if (user_count < 1) {
			if (to.path !== "/setup") {
				return navigateTo("/setup");
			}
		}
		// At least one user
		else {
			// The user is not on the login page
			if (to.path !== "/user/login") {
				// Check login status
				const session_cookie = useCookie("session");

				// Broken Session Cookie
				if (
					!session_cookie ||
					!session_cookie.value ||
					!session_cookie.value.username ||
					!session_cookie.value.session_id
				) {
					return navigateTo("/user/login");
				}
				const session = {
					username: session_cookie.value.username,
					session_id: session_cookie.value.session_id,
				};
				const valid_session = await verifySession(session);

				// User is logged in and not on the login page
				if (valid_session) {
					// Disallow the setup route
					if (to.path === "/setup") {
						if (from.path === to.path) {
							return navigateTo("/");
						}
						return navigateTo(from.path);
					}
				}

				// User is logged in
			}
		}
	}
});
