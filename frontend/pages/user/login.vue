<script setup>
definePageMeta({
	layout: "form-page",
});

const username = ref("");
const password = ref("");

const api_url = useRuntimeConfig().public.api_url;

async function get_session() {
	if (username.value !== "" && password.value !== "") {
		const url = `${api_url}/api/user/get_session`;
		const body = { username: username.value, password: password.value };
		const options = {
			method: "POST",
			headers: { "content-type": "application/json" },
			body: JSON.stringify(body),
		};

		const request = await apiRequest(url, options);
		console.log(request);

		// Error With the request (Client Side)
		if (request.is_request_error) {
			alert("An error occurred: request error");
			return;
		}

		// Error with the Response (non 200 Response code or data says it is an error)
		if (request.is_response_error || request.data.is_error) {
			alert(`An error occurred: ${request.data.error_msg}`);
			return;
		}

		const session_cookie = useCookie("session");
		const session = {
			username: request.data.data.username,
			session_id: request.data.data.session_id,
		};

		session_cookie.value = session;

		// TODO: Navigate the user to where he came from
		return navigateTo("/");

		// biome-ignore lint/style/noUselessElse: <explanation>
	} else {
		alert("Bitte füllen Sie alle Eingabefelder aus.");
	}
}
</script>


<template>
    <h1 class="mt-5">Anmeldung</h1>

    <p>Wenn Sie bereits ein Benutzerkonto besitzen, können Sie sich hier anmelden.</p>

    <form class="grid grid-cols-1 gap-3" @submit.prevent="get_session">
        <MyIconTextInput placeholder="Benutzername" icon="material-symbols:person" v-model="username" />

        <MyIconTextInput :password="true" placeholder="Passwort" icon="material-symbols:key" v-model="password" />

        <MyIconButton icon="material-symbols:login">Anmelden</MyIconButton>
    </form>
</template>